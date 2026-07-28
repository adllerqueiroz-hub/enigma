use super::*;

#[test]
fn updates_hero_3124_talent_extra_str() {
    let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
    let _ = config::init(&data_dir);

    let level_1 = hero_3124_talent_id(2, 1).unwrap();
    let level_2 = hero_3124_talent_id(2, 2).unwrap();
    let extra = update_talent_extra_str("", 2, 1, level_1, true);
    let extra = update_talent_extra_str(&extra, 2, 2, level_2, true);
    assert_eq!(extra, format!("2#{level_1},{level_2}"));

    let extra = update_talent_extra_str(&extra, 2, 2, level_2, false);
    assert_eq!(extra, format!("2#{level_1}"));
}

#[test]
fn duplicate_item_id_comes_from_character_duplicate_item() {
    let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
    let _ = config::init(&data_dir);

    assert_eq!(duplicate_item_id(3125).unwrap(), 133125);
}

#[tokio::test]
async fn hero_level_up_accepts_levels_between_stat_breakpoints_and_consumes_currency() {
    let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
    let _ = config::init(&data_dir);
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    database::run_migrations(&pool).await.unwrap();
    sqlx::query(
        "INSERT INTO users (id, username, created_at, updated_at)
         VALUES (22, 'level-up', 0, 0);
         INSERT INTO currencies (user_id, currency_id, quantity) VALUES
         (22, 3, 1000),
         (22, 5, 1000);",
    )
    .execute(&pool)
    .await
    .unwrap();
    let heroes = UserHeroModel::new(22, pool.clone());
    heroes.create_hero(3023).await.unwrap();

    let (_, updated, consumed) = HeroManager::new(22).level_up(&pool, 3023, 3).await.unwrap();

    assert_eq!(updated.level, Some(3));
    assert_eq!(consumed.currency_ids, vec![(3, -230), (5, -250)]);
    assert!(consumed.material_changes.is_empty());
}

#[test]
fn destiny_progression_follows_the_configured_slot_order() {
    let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
    let _ = config::init(&data_dir);

    let first = next_destiny_slot(3052, 0, 0).unwrap();
    assert_eq!((first.stage, first.node), (1, 1));

    let last_stage_one = config::configs::get()
        .character_destiny_slots
        .iter()
        .filter(|slot| slot.slots_id == 3052 && slot.stage == 1)
        .map(|slot| slot.node)
        .max()
        .unwrap();
    let next_rank = next_destiny_slot(3052, 1, last_stage_one).unwrap();
    assert_eq!((next_rank.stage, next_rank.node), (2, 1));
    assert_eq!(destiny_stones(3052), vec![305201]);
}

#[tokio::test]
async fn voice_unlock_requires_an_owned_matching_hero() {
    let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
    let _ = config::init(&data_dir);
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    database::run_migrations(&pool).await.unwrap();
    sqlx::query(
        "INSERT INTO users (id, username, created_at, updated_at) VALUES (14, 'voice', 0, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();
    UserHeroModel::new(14, pool.clone())
        .create_hero(3003)
        .await
        .unwrap();

    let manager = HeroManager::new(14);
    assert!(manager.unlock_voice(&pool, 3002, 1_300_302).await.is_err());
    manager.unlock_voice(&pool, 3003, 1_300_302).await.unwrap();
    let unlocked: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM hero_voices WHERE voice_id = 1300302")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(unlocked, 1);
}

#[tokio::test]
async fn item_unlock_uses_faith_config_and_existing_hero_storage() {
    let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
    let _ = config::init(&data_dir);
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    database::run_migrations(&pool).await.unwrap();
    sqlx::query(
        "INSERT INTO users (id, username, created_at, updated_at) VALUES (15, 'item-unlock', 0, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();
    UserHeroModel::new(15, pool.clone())
        .create_hero(3003)
        .await
        .unwrap();
    let manager = HeroManager::new(15);
    assert!(manager.unlock_item(&pool, 3003, 3).await.is_err());
    sqlx::query("UPDATE heroes SET faith = 100000 WHERE user_id = 15 AND hero_id = 3003")
        .execute(&pool)
        .await
        .unwrap();

    let (_, reward) = manager.unlock_item(&pool, 3003, 3).await.unwrap();

    assert_eq!(reward, (2, 40));
    let unlocked: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM hero_item_unlocks WHERE item_id = 3")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(unlocked, 1);
}

#[tokio::test]
async fn stale_skill_upgrade_rolls_back_its_cost() {
    let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
    let _ = config::init(&data_dir);
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    database::run_migrations(&pool).await.unwrap();
    sqlx::query(
        "INSERT INTO users (id, username, created_at, updated_at) VALUES (16, 'skill-race', 0, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();
    let heroes = UserHeroModel::new(16, pool.clone());
    heroes.create_hero(3125).await.unwrap();
    let item_id = duplicate_item_id(3125).unwrap();
    sqlx::query("INSERT INTO items (user_id, item_id, quantity) VALUES (16, ?, 1)")
        .bind(item_id)
        .execute(&pool)
        .await
        .unwrap();

    let mut tx = pool.begin().await.unwrap();
    reward::consume(
        &mut tx,
        16,
        &reward::RewardSet {
            items: vec![(item_id, 1)],
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert!(
        !heroes
            .upgrade_ex_skill_in_transaction(&mut tx, 3125, 2, 1)
            .await
            .unwrap()
    );
    tx.rollback().await.unwrap();

    let quantity: i32 =
        sqlx::query_scalar("SELECT quantity FROM items WHERE user_id = 16 AND item_id = ?")
            .bind(item_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(quantity, 1);
}

#[tokio::test]
async fn rank_and_insight_skin_commit_together() {
    let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
    let _ = config::init(&data_dir);
    let skin = config::configs::get()
        .skin
        .iter()
        .find(|skin| skin.id % 100 == 2 && skin.gain_approach == 1)
        .unwrap();
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    database::run_migrations(&pool).await.unwrap();
    sqlx::query(
        "INSERT INTO users (id, username, created_at, updated_at)
         VALUES (17, 'rank-skin', 0, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();
    let heroes = UserHeroModel::new(17, pool.clone());
    heroes.create_hero(skin.character_id).await.unwrap();
    heroes
        .set_rank_and_level(skin.character_id, 2, 30)
        .await
        .unwrap();

    HeroManager::new(17)
        .rank_up(&pool, skin.character_id)
        .await
        .unwrap();

    let hero = heroes.get(skin.character_id).await.unwrap();
    assert_eq!(hero.record.rank, 3);
    assert_eq!(hero.record.skin, skin.id);
    assert!(heroes.has_skin(skin.id).await.unwrap());
}

#[tokio::test]
async fn skin_can_be_owned_before_its_hero_but_not_equipped() {
    let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
    let _ = config::init(&data_dir);
    let game_data = config::configs::get();
    let skin = game_data
        .skin
        .iter()
        .find(|skin| {
            skin.character_id > 0
                && game_data.character.get(skin.character_id).is_some()
                && game_data
                    .default_character_skin(skin.character_id)
                    .is_some_and(|default| default.id != skin.id)
        })
        .unwrap();
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    database::run_migrations(&pool).await.unwrap();
    sqlx::query(
        "INSERT INTO users (id, username, created_at, updated_at)
         VALUES (20, 'skin-before-hero', 0, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();

    let heroes = UserHeroModel::new(20, pool.clone());
    let applied = crate::reward::RewardManager::new(20)
        .apply(
            &pool,
            crate::reward::RewardSet {
                skins: vec![(skin.id, 1)],
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(applied.skin_gains.len(), 1);
    assert_eq!(applied.skin_gains[0].skin_id, skin.id);
    assert!(applied.skin_gains[0].first_gain);
    assert!(heroes.has_skin(skin.id).await.unwrap());
    assert!(
        HeroManager::new(20)
            .use_skin(&pool, skin.character_id, skin.id)
            .await
            .is_err()
    );

    heroes.create_hero(skin.character_id).await.unwrap();
    let hero = heroes.get(skin.character_id).await.unwrap();
    assert_ne!(hero.record.skin, skin.id);
    assert!(hero.skin_list.iter().any(|owned| owned.skin == skin.id));
    HeroManager::new(20)
        .use_skin(&pool, skin.character_id, skin.id)
        .await
        .unwrap();
    assert_eq!(
        heroes.get(skin.character_id).await.unwrap().record.skin,
        skin.id
    );
}

#[tokio::test]
async fn profile_rejects_foreign_skins_and_equipment() {
    let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
    let _ = config::init(&data_dir);
    let foreign_skin = config::configs::get()
        .skin
        .iter()
        .find(|skin| skin.character_id == 3125)
        .unwrap();

    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    database::run_migrations(&pool).await.unwrap();
    for (id, name) in [(18_i64, "profile"), (19, "other")] {
        sqlx::query("INSERT INTO users (id, username, created_at, updated_at) VALUES (?, ?, 0, 0)")
            .bind(id)
            .bind(name)
            .execute(&pool)
            .await
            .unwrap();
    }
    let heroes = UserHeroModel::new(18, pool.clone());
    heroes.create_hero(3003).await.unwrap();

    let manager = HeroManager::new(18);
    assert!(
        manager
            .use_skin(&pool, 3003, foreign_skin.id)
            .await
            .is_err()
    );

    let foreign_uid = database::db::game::equipment::add_equipment(&pool, 19, 1000, 1)
        .await
        .unwrap()[0];
    assert!(
        manager
            .default_equip(&pool, 3003, foreign_uid)
            .await
            .is_err()
    );
}

#[tokio::test]
async fn specialization_rejects_the_wrong_hero_and_unknown_weapon_group() {
    let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
    let _ = config::init(&data_dir);
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    database::run_migrations(&pool).await.unwrap();
    sqlx::query(
        "INSERT INTO users (id, username, created_at, updated_at)
         VALUES (21, 'specialization', 0, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();
    let heroes = UserHeroModel::new(21, pool.clone());
    heroes.create_hero(3003).await.unwrap();
    heroes.create_hero(3123).await.unwrap();

    assert!(
        HeroManager::new(21)
            .choice_weapon(&pool, 3003, 1001, 0)
            .await
            .is_err()
    );
    assert!(
        HeroManager::new(21)
            .choice_weapon(&pool, 3123, 9999, 0)
            .await
            .is_err()
    );
    assert!(
        HeroManager::new(21)
            .reset_talents(&pool, 3003)
            .await
            .is_err()
    );
}
