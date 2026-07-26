use anyhow::Result;
use sqlx::{Sqlite, SqlitePool, Transaction, prelude::FromRow};

pub use crate::models::game::player_infos::{PlayerInfo, PlayerInfoData, ShowHero, UserBasicInfo};

const PLAYER_DEFAULT_ICON_CONST_ID: i32 = 8;

pub async fn power_purchase_state(pool: &SqlitePool, player_id: i64) -> Result<(i32, i32)> {
    Ok(sqlx::query_as(
        "SELECT state.power_buy_count, users.level
         FROM player_state AS state
         JOIN users ON users.id = state.player_id
         WHERE state.player_id = ?",
    )
    .bind(player_id)
    .fetch_one(pool)
    .await?)
}

pub fn default_portrait_id() -> Result<i32> {
    config::configs::get()
        .r#const
        .get(PLAYER_DEFAULT_ICON_CONST_ID)
        .ok_or_else(|| anyhow::anyhow!("missing player default icon config"))?
        .value
        .parse()
        .map_err(Into::into)
}

pub async fn get_player_info_data(
    pool: &SqlitePool,
    player_id: i64,
) -> anyhow::Result<Option<PlayerInfoData>> {
    // Get user basic info
    let user_info = sqlx::query_as::<_, (String, i32, i32, Option<i64>, Option<i64>)>(
        "SELECT username, level, exp, created_at, last_login_at FROM users WHERE id = ?",
    )
    .bind(player_id)
    .fetch_optional(pool)
    .await?;

    let Some((username, level, exp, created_at, last_login_at)) = user_info else {
        return Ok(None);
    };

    // Get player info
    let Some(player_info) = get_player_info(pool, player_id).await? else {
        return Ok(None);
    };

    // Get show heroes
    let show_heroes = get_show_heroes(pool, player_id).await?;

    Ok(Some(PlayerInfoData {
        player_id,
        user_info: UserBasicInfo {
            username,
            level,
            exp,
            created_at,
            last_login_at,
        },
        player_info,
        show_heroes,
    }))
}

/// Get player info by player_id
pub async fn get_player_info(pool: &SqlitePool, player_id: i64) -> Result<Option<PlayerInfo>> {
    let mut record =
        sqlx::query_as::<_, PlayerInfo>("SELECT * FROM player_info WHERE player_id = ?1")
            .bind(player_id)
            .fetch_optional(pool)
            .await?;

    if let Some(info) = &mut record {
        apply_dynamic_hero_rarity_counts(pool, info).await?;
    }

    Ok(record)
}

async fn apply_dynamic_hero_rarity_counts(pool: &SqlitePool, info: &mut PlayerInfo) -> Result<()> {
    let Some(tables) = config::configs::try_get() else {
        return Ok(());
    };

    let hero_ids =
        sqlx::query_scalar::<_, i32>("SELECT DISTINCT hero_id FROM heroes WHERE user_id = ?")
            .bind(info.player_id)
            .fetch_all(pool)
            .await?;

    let mut counts = [0; 6];
    for hero_id in hero_ids {
        let Some(character) = tables.character.get(hero_id) else {
            continue;
        };

        if let Ok(rare) = usize::try_from(character.rare)
            && (1..=5).contains(&rare)
        {
            counts[rare] += 1;
        }
    }

    info.hero_rare_nn_count = counts[1];
    info.hero_rare_n_count = counts[2];
    info.hero_rare_r_count = counts[3];
    info.hero_rare_sr_count = counts[4];
    info.hero_rare_ssr_count = counts[5];

    Ok(())
}

/// Get player's show heroes
pub async fn get_show_heroes(pool: &SqlitePool, player_id: i64) -> Result<Vec<ShowHero>> {
    let heroes = sqlx::query_as::<_, ShowHero>(
        "SELECT * FROM player_show_heroes WHERE player_id = ?1 ORDER BY display_order",
    )
    .bind(player_id)
    .fetch_all(pool)
    .await?;

    Ok(heroes)
}

pub async fn get_user_basic_info(
    pool: &SqlitePool,
    user_id: i64,
) -> sqlx::Result<(String, i32, i32)> {
    sqlx::query_as::<_, (String, i32, i32)>("SELECT username, level, exp FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(pool)
        .await
}

pub async fn increment_total_login_days(pool: &SqlitePool, player_id: i64) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE player_info
         SET total_login_days = total_login_days + 1, updated_at = ?
         WHERE player_id = ?",
    )
    .bind(common::time::ServerTime::now_ms())
    .bind(player_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub fn level_for_exp(exp: i32) -> i32 {
    config::configs::get()
        .player_level
        .iter()
        .filter(|level| exp >= level.exp)
        .map(|level| level.level)
        .max()
        .unwrap_or(1)
}

pub struct PlayerLevelChange {
    pub from: i32,
    pub to: i32,
}

pub async fn add_exp(pool: &SqlitePool, player_id: i64, amount: i32) -> Result<PlayerLevelChange> {
    let mut tx = pool.begin().await?;
    let change = add_exp_in_transaction(&mut tx, player_id, amount).await?;
    tx.commit().await?;
    Ok(change)
}

pub async fn add_exp_in_transaction(
    tx: &mut Transaction<'_, Sqlite>,
    player_id: i64,
    amount: i32,
) -> Result<PlayerLevelChange> {
    let (current_level, current_exp) =
        sqlx::query_as::<_, (i32, i32)>("SELECT level, exp FROM users WHERE id = ?")
            .bind(player_id)
            .fetch_one(&mut **tx)
            .await?;
    let level_floor = config::configs::get()
        .player_level
        .iter()
        .find(|level| level.level == current_level)
        .map(|level| level.exp)
        .unwrap_or_default();
    let max_exp = config::configs::get()
        .player_level
        .iter()
        .map(|level| level.exp)
        .max()
        .unwrap_or(i32::MAX);
    let exp = current_exp
        .max(level_floor)
        .saturating_add(amount.max(0))
        .min(max_exp);
    let level = level_for_exp(exp).max(current_level);

    sqlx::query("UPDATE users SET level = ?, exp = ?, updated_at = ? WHERE id = ?")
        .bind(level)
        .bind(exp)
        .bind(common::time::ServerTime::now_ms())
        .bind(player_id)
        .execute(&mut **tx)
        .await?;

    Ok(PlayerLevelChange {
        from: current_level,
        to: level,
    })
}

pub async fn set_portrait(pool: &SqlitePool, player_id: i64, portrait: i32) -> Result<()> {
    sqlx::query("UPDATE player_info SET portrait = ?, updated_at = ? WHERE player_id = ?")
        .bind(portrait)
        .bind(common::time::ServerTime::now_ms())
        .bind(player_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn set_signature(pool: &SqlitePool, player_id: i64, signature: &str) -> Result<()> {
    sqlx::query("UPDATE player_info SET signature = ?, updated_at = ? WHERE player_id = ?")
        .bind(signature)
        .bind(common::time::ServerTime::now_ms())
        .bind(player_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_birthday_once(pool: &SqlitePool, player_id: i64, birthday: &str) -> Result<bool> {
    Ok(sqlx::query(
        "UPDATE player_info SET birthday = ?, updated_at = ?
         WHERE player_id = ? AND birthday = ''",
    )
    .bind(birthday)
    .bind(common::time::ServerTime::now_ms())
    .bind(player_id)
    .execute(pool)
    .await?
    .rows_affected()
        == 1)
}

pub async fn set_character_age(
    pool: &SqlitePool,
    player_id: i64,
    character_age: &[i32],
) -> Result<()> {
    sqlx::query("UPDATE player_info SET character_age = ?, updated_at = ? WHERE player_id = ?")
        .bind(serde_json::to_string(character_age)?)
        .bind(common::time::ServerTime::now_ms())
        .bind(player_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_player_bg(pool: &SqlitePool, player_id: i64, bg_id: i32) -> Result<()> {
    sqlx::query("UPDATE player_info SET bg = ?, updated_at = ? WHERE player_id = ?")
        .bind(bg_id)
        .bind(common::time::ServerTime::now_ms())
        .bind(player_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_show_achievement(
    pool: &SqlitePool,
    player_id: i64,
    show_achievement: String,
) -> Result<()> {
    let now = common::time::ServerTime::now_ms();
    sqlx::query("UPDATE player_info SET show_achievement = ?, updated_at = ? WHERE player_id = ?")
        .bind(&show_achievement)
        .bind(now)
        .bind(player_id)
        .execute(pool)
        .await?;

    sqlx::query(
        r#"
        INSERT INTO user_player_card_info (user_id, show_achievement)
        VALUES (?, ?)
        ON CONFLICT(user_id) DO UPDATE SET
            show_achievement = excluded.show_achievement
        "#,
    )
    .bind(player_id)
    .bind(show_achievement)
    .execute(pool)
    .await?;

    Ok(())
}

/// Create default player info
pub async fn create_player_info(pool: &SqlitePool, player_id: i64, now: i64) -> Result<()> {
    let portrait = default_portrait_id()?;

    sqlx::query(
        "INSERT INTO player_info (
            player_id, signature, birthday, character_age, portrait, show_achievement, bg,
            total_login_days, last_episode_id, last_logout_time,
            hero_rare_nn_count, hero_rare_n_count, hero_rare_r_count,
            hero_rare_sr_count, hero_rare_ssr_count,
            created_at, updated_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)",
    )
    .bind(player_id)
    .bind("")
    .bind("")
    .bind("[]")
    .bind(portrait)
    .bind("")
    .bind(0)
    .bind(0)
    .bind(0)
    .bind(None::<i64>)
    .bind(0)
    .bind(0)
    .bind(0)
    .bind(0)
    .bind(0)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(())
}

/// Update player info
pub async fn update_player_info(pool: &SqlitePool, info: &PlayerInfo) -> Result<()> {
    sqlx::query(
        "UPDATE player_info SET
            signature = ?2,
            birthday = ?3,
            portrait = ?4,
            show_achievement = ?5,
            bg = ?6,
            total_login_days = ?7,
            last_episode_id = ?8,
            last_logout_time = ?9,
            hero_rare_nn_count = ?10,
            hero_rare_n_count = ?11,
            hero_rare_r_count = ?12,
            hero_rare_sr_count = ?13,
            hero_rare_ssr_count = ?14,
            updated_at = ?15
         WHERE player_id = ?1",
    )
    .bind(info.player_id)
    .bind(&info.signature)
    .bind(&info.birthday)
    .bind(info.portrait)
    .bind(&info.show_achievement)
    .bind(info.bg)
    .bind(info.total_login_days)
    .bind(info.last_episode_id)
    .bind(info.last_logout_time)
    .bind(info.hero_rare_nn_count)
    .bind(info.hero_rare_n_count)
    .bind(info.hero_rare_r_count)
    .bind(info.hero_rare_sr_count)
    .bind(info.hero_rare_ssr_count)
    .bind(info.updated_at)
    .execute(pool)
    .await?;

    Ok(())
}

/// Set show heroes
pub async fn set_show_hero(pool: &SqlitePool, player_id: i64, hero_uids: Vec<i64>) -> Result<()> {
    let mut tx = pool.begin().await?;

    for (slot_idx, uid) in hero_uids.into_iter().enumerate() {
        let display_order = slot_idx as i32;

        if uid == 0 {
            // Explicitly clear this slot
            sqlx::query(
                r#"
                DELETE FROM player_show_heroes
                WHERE player_id = ? AND display_order = ?
                "#,
            )
            .bind(player_id)
            .bind(display_order)
            .execute(&mut *tx)
            .await?;

            continue;
        }

        #[derive(FromRow)]
        struct HeroRow {
            hero_id: i32,
            level: i32,
            rank: i32,
            ex_skill_level: i32,
            skin: i32,
        }

        // Resolve hero UID → hero data
        let hero = sqlx::query_as::<_, HeroRow>(
            "
            SELECT
                hero_id,
                level,
                rank,
                ex_skill_level,
                skin
            FROM heroes
            WHERE uid = ? AND user_id = ?
            ",
        )
        .bind(uid)
        .bind(player_id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Invalid hero uid {} for user {}", uid, player_id))?;

        // Set / replace this slot
        sqlx::query(
            r#"
            INSERT INTO player_show_heroes (
                player_id,
                hero_id,
                level,
                rank,
                ex_skill_level,
                skin,
                display_order
            )
            VALUES (?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(player_id, display_order)
            DO UPDATE SET
                hero_id = excluded.hero_id,
                level = excluded.level,
                rank = excluded.rank,
                ex_skill_level = excluded.ex_skill_level,
                skin = excluded.skin
            "#,
        )
        .bind(player_id)
        .bind(hero.hero_id)
        .bind(hero.level)
        .bind(hero.rank)
        .bind(hero.ex_skill_level)
        .bind(hero.skin)
        .bind(display_order)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}
