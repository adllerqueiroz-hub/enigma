use anyhow::Result;
use database::{
    db::game::{block_packages, currencies, dungeons, equipment, items},
    models::game::{currencies::UserCurrencyModel, heros::UserHeroModel, items::UserItemModel},
};
use muipserver::{GmRequest, GmResponse, MaterialQuery};
use serde::Serialize;
use sonettobuf::{
    BlockPackageGainPush, ChapterMapElementUpdatePush, ChapterMapUpdatePush, CmdId,
    CurrencyChangePush, DungeonUpdatePush, EquipUpdatePush, HeroSkinGainPush, HeroUpdatePush,
    ItemChangePush, MaterialChangePush, MaterialData, PlayerCardInfoPush, StoryFinishPush,
    UpdateOpenPush, prost::Message,
};
use std::collections::HashSet;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};
use tracing::{info, warn};

use crate::{
    logic::reward,
    net::{app::AppState, outbound::CommandPacket},
};

pub async fn run_gm_listener(addr: String, state: &'static AppState) -> std::io::Result<()> {
    let listener = TcpListener::bind(&addr).await?;
    info!("MUIP GM bridge listening on {}", listener.local_addr()?);

    loop {
        let (stream, peer) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(err) = handle_connection(stream, state).await {
                warn!("MUIP GM connection {peer} failed: {err}");
            }
        });
    }
}

async fn handle_connection(stream: TcpStream, state: &'static AppState) -> std::io::Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();
    reader.read_line(&mut line).await?;

    let trimmed = line.trim();
    if trimmed.is_empty() {
        return write_response(&mut writer, GmResponse::err(400, "empty GM request")).await;
    }
    if is_http_request(trimmed) {
        return write_http_redirect(&mut writer).await;
    }

    let request = match serde_json::from_str::<GmRequest>(trimmed) {
        Ok(request) => request,
        Err(err) => {
            return write_response(
                &mut writer,
                GmResponse::err(400, format!("invalid GM request: {err}")),
            )
            .await;
        }
    };

    let response = match request {
        GmRequest::Status => status(state).await,
        GmRequest::ListPlayers => list_players(state),
        GmRequest::Dungeons => dungeon_catalog(),
        GmRequest::Materials { query } => materials(state, query).await,
        GmRequest::Execute {
            player_uid,
            command,
        } => execute(state, player_uid, command).await,
    };

    write_response(&mut writer, response).await
}

async fn write_response(
    writer: &mut (impl AsyncWriteExt + Unpin),
    response: GmResponse,
) -> std::io::Result<()> {
    let mut payload = serde_json::to_vec(&response).map_err(std::io::Error::other)?;
    payload.push(b'\n');
    writer.write_all(&payload).await?;
    writer.flush().await
}

fn is_http_request(line: &str) -> bool {
    line.starts_with("GET ") || line.starts_with("HEAD ")
}

async fn write_http_redirect(writer: &mut (impl AsyncWriteExt + Unpin)) -> std::io::Result<()> {
    let location = format!("http://127.0.0.1:{}/", common::muip_port());
    let body = format!(
        "<!doctype html><meta http-equiv=\"refresh\" content=\"0;url={0}\"><a href=\"{0}\">Open MUIP panel</a>",
        location
    );
    let response = format!(
        "HTTP/1.1 302 Found\r\nLocation: {location}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    writer.write_all(response.as_bytes()).await?;
    writer.flush().await
}

async fn status(state: &AppState) -> GmResponse {
    let online_players = state.online_player_ids();
    let player_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
        .fetch_one(state.db)
        .await
        .unwrap_or_default() as usize;

    let mut response = GmResponse::ok("online");
    response.online = online_players.len();
    response.players = online_players
        .into_iter()
        .map(|id| id.to_string())
        .collect();
    response.data = Some(serde_json::json!({
        "online": response.online,
        "playerCount": player_count,
        "maxPlayers": 99999999,
        "players": response.players,
        "status": "online"
    }));
    response
}

fn list_players(state: &AppState) -> GmResponse {
    let players = state
        .online_player_ids()
        .into_iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>();

    let mut response = GmResponse::ok(format!("{} player(s) online", players.len()));
    response.online = players.len();
    response.players = players;
    response
}

fn dungeon_catalog() -> GmResponse {
    let tables = config::configs::get();
    let mut chapters = tables
        .chapter
        .iter()
        .filter(|chapter| {
            tables
                .episode
                .iter()
                .any(|episode| episode.chapter_id == chapter.id)
        })
        .map(|chapter| DungeonCatalogChapter {
            id: chapter.id,
            name: resolve_name(
                tables,
                if chapter.name_en.is_empty() {
                    &chapter.name
                } else {
                    &chapter.name_en
                },
            ),
        })
        .collect::<Vec<_>>();
    let mut episodes = tables
        .episode
        .iter()
        .filter(|episode| tables.chapter.get(episode.chapter_id).is_some())
        .map(|episode| DungeonCatalogEpisode {
            id: episode.id,
            chapter_id: episode.chapter_id,
            name: resolve_name(
                tables,
                if episode.name_en.is_empty() {
                    &episode.name
                } else {
                    &episode.name_en
                },
            ),
        })
        .collect::<Vec<_>>();
    chapters.sort_unstable_by_key(|chapter| chapter.id);
    episodes.sort_unstable_by_key(|episode| (episode.chapter_id, episode.id));

    GmResponse::ok_data("dungeons", DungeonCatalog { chapters, episodes })
}

async fn materials(state: &AppState, query: MaterialQuery) -> GmResponse {
    match material_catalog(state, query).await {
        Ok(catalog) => GmResponse::ok_data("materials", catalog),
        Err(err) => GmResponse::err(400, err.to_string()),
    }
}

async fn execute(state: &'static AppState, player_uid: String, command: String) -> GmResponse {
    let Ok(player_id) = player_uid.parse::<i64>() else {
        return GmResponse::err(400, format!("invalid player_uid `{player_uid}`"));
    };

    let exists = match sqlx::query_scalar::<_, i64>("SELECT 1 FROM users WHERE id = ?")
        .bind(player_id)
        .fetch_optional(state.db)
        .await
    {
        Ok(exists) => exists,
        Err(err) => return GmResponse::err(500, format!("database error: {err}")),
    };
    if exists.is_none() {
        return GmResponse::err(404, format!("player `{player_uid}` was not found"));
    }

    match run_command(state, player_id, &command).await {
        Ok(response) => response,
        Err(err) => GmResponse::err(400, err.to_string()),
    }
}

async fn run_command(
    state: &'static AppState,
    player_id: i64,
    command: &str,
) -> Result<GmResponse> {
    let parts = command.split_whitespace().collect::<Vec<_>>();
    let Some(first) = parts.first() else {
        anyhow::bail!("command is required");
    };

    let args = match first.to_ascii_lowercase().as_str() {
        "dungeon" => return unlock_dungeon(state, player_id, &parts[1..]).await,
        "material" | "reward" | "give" | "add" => &parts[1..],
        kind if MaterialKind::parse(kind).is_some() => &parts[..],
        "status" => return Ok(status(state).await),
        "players" | "list" | "listplayers" | "list_players" => return Ok(list_players(state)),
        "help" | "?" => {
            return Ok(GmResponse::ok(
                "commands: help, status, players, dungeon unlock <stage|chapter> <id>, material <type> <id> <amount>, give <item|currency|hero|skin|equip|power|insight> <id> <amount>",
            ));
        }
        _ => anyhow::bail!("unknown command '{}'", first),
    };

    grant(state, player_id, args).await
}

async fn unlock_dungeon(
    state: &'static AppState,
    player_id: i64,
    args: &[&str],
) -> Result<GmResponse> {
    if args.len() != 3 || !args[0].eq_ignore_ascii_case("unlock") {
        anyhow::bail!("usage: dungeon unlock <stage|chapter> <id>");
    }

    let id = parse_positive(args[2], "dungeon id")?;
    let (dungeon_infos, finished_story_ids, open_infos) =
        match args[1].to_ascii_lowercase().as_str() {
            "stage" | "episode" => dungeons::unlock_stage(state.db, player_id, id).await?,
            "chapter" => dungeons::unlock_chapter(state.db, player_id, id).await?,
            kind => anyhow::bail!("unknown dungeon unlock type '{kind}'"),
        };

    for story_id in &finished_story_ids {
        send_push(
            state,
            player_id,
            CmdId::StoryFinishPushCmd,
            StoryFinishPush {
                story_id: Some(*story_id),
            },
        )
        .await?;
    }

    let chapter_type_nums = dungeons::get_chapter_type_nums(state.db, player_id)
        .await?
        .into_iter()
        .map(Into::into)
        .collect::<Vec<sonettobuf::UserChapterTypeNum>>();
    let episode_ids = dungeon_infos
        .iter()
        .map(|dungeon| dungeon.episode_id)
        .collect::<Vec<_>>();
    for dungeon_info in dungeon_infos {
        send_push(
            state,
            player_id,
            CmdId::DungeonUpdatePushCmd,
            DungeonUpdatePush {
                dungeon_info: Some(dungeon_info.into()),
                chapter_type_nums: chapter_type_nums.clone(),
            },
        )
        .await?;
    }
    if !open_infos.is_empty() {
        send_push(
            state,
            player_id,
            CmdId::UpdateOpenPushCmd,
            UpdateOpenPush {
                open_infos: open_infos.clone(),
            },
        )
        .await?;
    }
    let (map_ids, elements) = dungeons::reconcile_map_progression(state.db, player_id).await?;
    if !map_ids.is_empty() {
        send_push(
            state,
            player_id,
            CmdId::ChapterMapUpdatePushCmd,
            ChapterMapUpdatePush { map_ids },
        )
        .await?;
    }
    if !elements.is_empty() {
        send_push(
            state,
            player_id,
            CmdId::ChapterMapElementUpdatePushCmd,
            ChapterMapElementUpdatePush { elements },
        )
        .await?;
    }

    Ok(GmResponse::ok_data(
        format!("unlocked dungeon {0} {id}", args[1].to_ascii_lowercase()),
        serde_json::json!({
            "passedPrerequisiteEpisodes": episode_ids,
            "finishedStories": finished_story_ids,
            "unlockedOpenIds": open_infos.into_iter().filter(|info| info.is_open).map(|info| info.id).collect::<Vec<_>>(),
        }),
    ))
}

async fn grant(state: &'static AppState, player_id: i64, args: &[&str]) -> Result<GmResponse> {
    if args.len() != 3 {
        anyhow::bail!(
            "usage: material <type> <id> <amount> or give <item|currency|hero|skin|equip|power|package|insight> <id> <amount>"
        );
    }

    let kind = MaterialKind::parse(args[0])
        .ok_or_else(|| anyhow::anyhow!("unknown material type '{}'", args[0]))?;
    let id = parse_positive(args[1], "material id")?;
    let amount = parse_positive(args[2], "amount")?;

    let mut data = GrantData {
        user_id: player_id,
        rewards: vec![RewardData {
            r#type: kind.id(),
            id,
            amount,
        }],
        changed_item_ids: Vec::new(),
        changed_power_item_ids: Vec::new(),
        changed_insight_item_ids: Vec::new(),
        changed_currency_ids: Vec::new(),
        changed_hero_ids: Vec::new(),
        changed_skin_ids: Vec::new(),
        changed_equip_ids: Vec::new(),
    };

    match kind {
        MaterialKind::Item => {
            let model = UserItemModel::new(player_id, (*state.db).clone());
            data.changed_item_ids = model.create_items(vec![(id as u32, amount)]).await?;
        }
        MaterialKind::Currency => {
            let model = UserCurrencyModel::new(player_id, (*state.db).clone());
            data.changed_currency_ids = model
                .create_currencies(&[(id, amount)])
                .await?
                .into_iter()
                .map(|(id, _)| id)
                .collect();
        }
        MaterialKind::PlayerExp => {
            sqlx::query("UPDATE users SET exp = exp + ?, updated_at = ? WHERE id = ?")
                .bind(amount)
                .bind(common::time::ServerTime::now_ms())
                .bind(player_id)
                .execute(state.db)
                .await?;
        }
        MaterialKind::Hero => {
            let model = UserHeroModel::new(player_id, (*state.db).clone());
            for _ in 0..amount {
                if model.has_hero(id).await? {
                    let duplicate_count = model.add_hero_duplicate(id).await?;
                    let rewards = reward::hero_duplicate_rewards(id, duplicate_count)?;
                    let applied = reward::apply(state.db, player_id, rewards).await?;
                    data.merge_rewards(applied);
                } else {
                    model.create_hero(id).await?;
                }
            }
            data.changed_hero_ids.push(id);
        }
        MaterialKind::Skin => {
            let model = UserHeroModel::new(player_id, (*state.db).clone());
            if model.unlock_skin(id).await? {
                data.changed_skin_ids.push(id);
            }
        }
        MaterialKind::Equipment => {
            data.changed_equip_ids =
                equipment::add_equipments(state.db, player_id, &[(id, amount)]).await?;
        }
        MaterialKind::PowerItem => {
            let model = UserItemModel::new(player_id, (*state.db).clone());
            data.changed_power_item_ids = model.create_power_items(vec![(id, amount)]).await?;
        }
        MaterialKind::BlockPackage => {
            block_packages::add_block_package(state.db, player_id, id).await?;
        }
        MaterialKind::InsightItem => {
            let model = UserItemModel::new(player_id, (*state.db).clone());
            data.changed_insight_item_ids = model.create_insight_items(vec![(id, amount)]).await?;
        }
    }

    send_grant_pushes(state, &data).await?;

    Ok(GmResponse::ok_data(
        format!("granted {amount} of {}#{id} to {player_id}", kind.id()),
        data,
    ))
}

async fn send_grant_pushes(state: &'static AppState, data: &GrantData) -> Result<()> {
    let reward = &data.rewards[0];
    let Some(kind) = MaterialKind::from_raw(reward.r#type) else {
        return Ok(());
    };

    send_push(
        state,
        data.user_id,
        CmdId::MaterialChangePushCmd,
        MaterialChangePush {
            data_list: vec![MaterialData {
                materil_type: Some(kind.id() as u32),
                materil_id: Some(reward.id as u32),
                quantity: Some(reward.amount),
            }],
            get_approach: None,
        },
    )
    .await?;

    let mut changed_items = Vec::new();
    for item_id in &data.changed_item_ids {
        if let Some(item) = items::get_item(state.db, data.user_id, *item_id as u32).await? {
            changed_items.push(item.into());
        }
    }

    let mut changed_power_items = Vec::new();
    for item_id in &data.changed_power_item_ids {
        if let Some(item) = items::get_power_item(state.db, data.user_id, *item_id as u32).await? {
            changed_power_items.push(item.into());
        }
    }

    let mut changed_insight_items = Vec::new();
    for item_id in &data.changed_insight_item_ids {
        if let Some(item) = items::get_insight_item(state.db, data.user_id, *item_id as u32).await?
        {
            changed_insight_items.push(item.into());
        }
    }

    send_item_push(
        state,
        data.user_id,
        changed_items,
        changed_power_items,
        changed_insight_items,
    )
    .await?;

    if kind == MaterialKind::BlockPackage {
        let packages = block_packages::get_block_packages(state.db, data.user_id)
            .await?
            .into_iter()
            .filter(|package| package.block_package_id == reward.id)
            .map(Into::into)
            .collect();
        send_push(
            state,
            data.user_id,
            CmdId::BlockPackageGainPushCmd,
            BlockPackageGainPush {
                block_packages: packages,
            },
        )
        .await?;
    }

    let mut change_currency = Vec::new();
    for currency_id in &data.changed_currency_ids {
        if let Some(currency) =
            currencies::get_currency(state.db, data.user_id, *currency_id).await?
        {
            change_currency.push(currency.into());
        }
    }

    if !change_currency.is_empty() {
        send_push(
            state,
            data.user_id,
            CmdId::CurrencyChangePushCmd,
            CurrencyChangePush { change_currency },
        )
        .await?;
    }

    let mut equips = Vec::new();
    for equip_uid in &data.changed_equip_ids {
        equips.push(
            equipment::get_equipment_by_uid(state.db, data.user_id, *equip_uid)
                .await?
                .into(),
        );
    }

    if !equips.is_empty() {
        send_push(
            state,
            data.user_id,
            CmdId::EquipUpdatePushCmd,
            EquipUpdatePush { equips },
        )
        .await?;
    }

    let heroes = UserHeroModel::new(data.user_id, (*state.db).clone());
    let mut hero_updates = Vec::new();
    for hero_id in &data.changed_hero_ids {
        hero_updates
            .push(crate::logic::hero::snapshot(state.db, heroes.get_hero(*hero_id).await?).await?);
    }

    if !hero_updates.is_empty() {
        send_push(
            state,
            data.user_id,
            CmdId::HeroHeroUpdatePushCmd,
            HeroUpdatePush { hero_updates },
        )
        .await?;
        let player_card =
            crate::logic::player_card::get_player_card_info(state.db, data.user_id).await?;
        send_push(
            state,
            data.user_id,
            CmdId::PlayerCardInfoPushCmd,
            PlayerCardInfoPush {
                player_card_info: player_card.player_card_info,
            },
        )
        .await?;
    }

    for skin_id in &data.changed_skin_ids {
        send_push(
            state,
            data.user_id,
            CmdId::HeroSkinGainPushCmd,
            HeroSkinGainPush {
                skin_id: Some(*skin_id),
                first_gain: Some(true),
                get_approach: None,
            },
        )
        .await?;
    }

    Ok(())
}

async fn send_item_push(
    state: &'static AppState,
    player_id: i64,
    items: Vec<sonettobuf::Item>,
    power_items: Vec<sonettobuf::PowerItem>,
    insight_items: Vec<sonettobuf::InsightItem>,
) -> Result<()> {
    if items.is_empty() && power_items.is_empty() && insight_items.is_empty() {
        return Ok(());
    }

    send_push(
        state,
        player_id,
        CmdId::ItemChangePushCmd,
        ItemChangePush {
            items,
            power_items,
            insight_items,
            expire_items: Vec::new(),
            talent_items: Vec::new(),
        },
    )
    .await
}

async fn send_push<M: Message>(
    state: &'static AppState,
    player_id: i64,
    cmd_id: CmdId,
    message: M,
) -> Result<()> {
    let Some(sender) = state.get_session_sender(player_id) else {
        return Ok(());
    };

    let down_tag = state.reserve_down_tag().await;
    sender
        .send(CommandPacket::Push {
            cmd_id,
            body: message.encode_to_vec(),
            down_tag,
        })
        .await
        .map_err(|err| anyhow::anyhow!("failed to send MUIP push: {err}"))
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct GrantData {
    user_id: i64,
    rewards: Vec<RewardData>,
    changed_item_ids: Vec<i32>,
    changed_power_item_ids: Vec<i32>,
    changed_insight_item_ids: Vec<i32>,
    changed_currency_ids: Vec<i32>,
    changed_hero_ids: Vec<i32>,
    changed_skin_ids: Vec<i32>,
    changed_equip_ids: Vec<i64>,
}

impl GrantData {
    fn merge_rewards(&mut self, rewards: reward::AppliedRewards) {
        self.changed_item_ids
            .extend(rewards.item_ids.into_iter().map(|id| id as i32));
        self.changed_power_item_ids.extend(rewards.power_item_ids);
        self.changed_insight_item_ids
            .extend(rewards.insight_item_ids);
        self.changed_currency_ids
            .extend(rewards.currency_ids.into_iter().map(|(id, _)| id));
        self.changed_hero_ids.extend(rewards.hero_ids);
        self.changed_skin_ids
            .extend(rewards.skin_gains.into_iter().map(|skin| skin.skin_id));
        self.changed_equip_ids.extend(rewards.equip_uids);
    }
}

#[derive(Debug, Serialize)]
struct RewardData {
    r#type: i32,
    id: i32,
    amount: i32,
}

#[derive(Debug, Serialize)]
pub struct CatalogType {
    r#type: i32,
    name: &'static str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogEntry {
    r#type: i32,
    id: i32,
    name: String,
    raw_name: String,
    rare: i32,
}

#[derive(Debug, Serialize)]
struct CatalogResponse {
    types: Vec<CatalogType>,
    items: Vec<CatalogEntry>,
}

#[derive(Debug, Serialize)]
struct DungeonCatalog {
    chapters: Vec<DungeonCatalogChapter>,
    episodes: Vec<DungeonCatalogEpisode>,
}

#[derive(Debug, Serialize)]
struct DungeonCatalogChapter {
    id: i32,
    name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct DungeonCatalogEpisode {
    id: i32,
    chapter_id: i32,
    name: String,
}

async fn material_catalog(state: &AppState, query: MaterialQuery) -> Result<CatalogResponse> {
    let Some(kind) = query.r#type.and_then(MaterialKind::from_raw) else {
        return Ok(CatalogResponse {
            types: catalog_types(),
            items: Vec::new(),
        });
    };

    let q = query.q.unwrap_or_default().to_ascii_lowercase();
    let limit = query.limit.unwrap_or(200).min(1000);
    let owned_skins = if kind == MaterialKind::Skin && query.unowned_only.unwrap_or(false) {
        match query.player_uid {
            Some(player_id) => Some(
                UserHeroModel::new(player_id, (*state.db).clone())
                    .get_skins()
                    .await?
                    .into_iter()
                    .collect::<HashSet<_>>(),
            ),
            None => None,
        }
    } else {
        None
    };
    let mut materials = entries_for_kind(state.tables, kind, owned_skins.as_ref());

    if !q.is_empty() {
        materials.retain(|entry| {
            entry.id.to_string().contains(&q)
                || entry.name.to_ascii_lowercase().contains(&q)
                || entry.raw_name.to_ascii_lowercase().contains(&q)
        });
    }

    materials.truncate(limit);

    Ok(CatalogResponse {
        types: catalog_types(),
        items: materials,
    })
}

fn catalog_types() -> Vec<CatalogType> {
    [
        MaterialKind::Item,
        MaterialKind::Currency,
        MaterialKind::PlayerExp,
        MaterialKind::Hero,
        MaterialKind::Skin,
        MaterialKind::Equipment,
        MaterialKind::PowerItem,
        MaterialKind::BlockPackage,
        MaterialKind::InsightItem,
    ]
    .into_iter()
    .map(|kind| CatalogType {
        r#type: kind.id(),
        name: kind.label(),
    })
    .collect()
}

fn entries_for_kind(
    tables: &config::GameDB,
    kind: MaterialKind,
    owned_skins: Option<&HashSet<i32>>,
) -> Vec<CatalogEntry> {
    match kind {
        MaterialKind::Item => tables
            .item
            .iter()
            .map(|row| catalog_entry(tables, kind, row.id, &row.name, row.rare))
            .collect(),
        MaterialKind::Currency => tables
            .currency
            .iter()
            .map(|row| catalog_entry(tables, kind, row.id, &row.name, row.rare))
            .collect(),
        MaterialKind::BlockPackage => tables
            .block_package
            .iter()
            .map(|row| catalog_entry(tables, kind, row.id, &row.name, row.rare))
            .collect(),
        MaterialKind::PlayerExp => vec![catalog_entry(tables, kind, 1, "Player EXP", 0)],
        MaterialKind::Hero => tables
            .character
            .iter()
            .map(|row| {
                let raw_name = if row.name_eng.is_empty() {
                    &row.name
                } else {
                    &row.name_eng
                };
                catalog_entry(tables, kind, row.id, raw_name, row.rare)
            })
            .collect(),
        MaterialKind::Skin => tables
            .skin
            .iter()
            .filter(|row| is_premium_hero_skin(tables, row))
            .filter(|row| !owned_skins.is_some_and(|owned| owned.contains(&row.id)))
            .map(|row| {
                let raw_name = if row.name_eng.is_empty() {
                    &row.name
                } else {
                    &row.name_eng
                };
                catalog_entry(tables, kind, row.id, raw_name, row.rare)
            })
            .collect(),
        MaterialKind::Equipment => tables
            .equip
            .iter()
            .map(|row| {
                let raw_name = if row.name_en.is_empty() {
                    &row.name
                } else {
                    &row.name_en
                };
                catalog_entry(tables, kind, row.id, raw_name, row.rare)
            })
            .collect(),
        MaterialKind::PowerItem => tables
            .power_item
            .iter()
            .map(|row| catalog_entry(tables, kind, row.id, &row.name, row.rare))
            .collect(),
        MaterialKind::InsightItem => tables
            .insight_item
            .iter()
            .map(|row| catalog_entry(tables, kind, row.id, &row.name, row.rare))
            .collect(),
    }
}

fn is_premium_hero_skin(tables: &config::GameDB, skin: &config::skin::Skin) -> bool {
    let Some(character) = tables.character.get(skin.character_id) else {
        return false;
    };

    skin.character_id == character.id && !matches!(skin.id % 100, 1 | 2)
}

fn catalog_entry(
    tables: &config::GameDB,
    kind: MaterialKind,
    id: i32,
    raw_name: &str,
    rare: i32,
) -> CatalogEntry {
    let name = resolve_name(tables, raw_name);
    CatalogEntry {
        r#type: kind.id(),
        id,
        name,
        raw_name: raw_name.to_string(),
        rare,
    }
}

fn resolve_name(tables: &config::GameDB, raw_name: &str) -> String {
    let resolved = tables
        .language_en
        .get(raw_name)
        .or_else(|| tables.language_server_en.get(raw_name))
        .unwrap_or(raw_name);

    clean_name(resolved)
}

fn clean_name(value: &str) -> String {
    let mut cleaned = String::with_capacity(value.len());
    let mut in_tag = false;
    for ch in value.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            '\r' | '\n' if !in_tag => cleaned.push(' '),
            _ if !in_tag => cleaned.push(ch),
            _ => {}
        }
    }
    cleaned.trim().to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
enum MaterialKind {
    Item = 1,
    Currency = 2,
    PlayerExp = 3,
    Hero = 4,
    Skin = 5,
    Equipment = 9,
    PowerItem = 10,
    BlockPackage = 13,
    InsightItem = 24,
}

impl MaterialKind {
    fn parse(value: &str) -> Option<Self> {
        if let Ok(raw) = value.parse::<i32>() {
            return Self::from_raw(raw);
        }

        match value.to_ascii_lowercase().as_str() {
            "item" | "items" => Some(Self::Item),
            "currency" | "currencies" | "coin" => Some(Self::Currency),
            "exp" | "playerexp" => Some(Self::PlayerExp),
            "hero" | "heroes" => Some(Self::Hero),
            "skin" | "skins" => Some(Self::Skin),
            "equip" | "equipment" | "psychube" | "psychubes" => Some(Self::Equipment),
            "power" | "poweritem" => Some(Self::PowerItem),
            "blockpackage" | "package" => Some(Self::BlockPackage),
            "insight" | "insightitem" => Some(Self::InsightItem),
            _ => None,
        }
    }

    fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            raw if raw == Self::Item as i32 => Some(Self::Item),
            raw if raw == Self::Currency as i32 => Some(Self::Currency),
            raw if raw == Self::PlayerExp as i32 => Some(Self::PlayerExp),
            raw if raw == Self::Hero as i32 => Some(Self::Hero),
            raw if raw == Self::Skin as i32 => Some(Self::Skin),
            raw if raw == Self::Equipment as i32 => Some(Self::Equipment),
            raw if raw == Self::PowerItem as i32 => Some(Self::PowerItem),
            raw if raw == Self::BlockPackage as i32 => Some(Self::BlockPackage),
            raw if raw == Self::InsightItem as i32 => Some(Self::InsightItem),
            _ => None,
        }
    }

    fn id(self) -> i32 {
        self as i32
    }

    fn label(self) -> &'static str {
        match self {
            Self::Item => "item",
            Self::Currency => "currency",
            Self::PlayerExp => "playerExp",
            Self::Hero => "hero",
            Self::Skin => "heroSkin",
            Self::Equipment => "equipment",
            Self::PowerItem => "powerItem",
            Self::BlockPackage => "blockPackage",
            Self::InsightItem => "insightItem",
        }
    }
}

fn parse_positive(value: &str, label: &str) -> Result<i32> {
    let parsed = value.parse::<i32>()?;
    if parsed <= 0 {
        anyhow::bail!("invalid {label} '{value}'");
    }
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::{MaterialKind, entries_for_kind, is_premium_hero_skin};
    use std::collections::HashSet;

    #[test]
    fn hero_skin_catalog_excludes_basic_and_insight_skins() {
        let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
        let _ = config::init(&data_dir);
        let tables = config::configs::get();
        let first_skin_id = tables
            .skin
            .iter()
            .find(|skin| is_premium_hero_skin(tables, skin))
            .unwrap()
            .id;
        let owned = HashSet::from([first_skin_id]);

        let all = entries_for_kind(tables, MaterialKind::Skin, None);
        let unowned = entries_for_kind(tables, MaterialKind::Skin, Some(&owned));

        assert!(all.iter().all(|entry| {
            tables
                .skin
                .get(entry.id)
                .is_some_and(|skin| is_premium_hero_skin(tables, skin))
        }));
        assert!(all.iter().any(|entry| entry.id == first_skin_id));
        assert!(!unowned.iter().any(|entry| entry.id == first_skin_id));
    }
}
