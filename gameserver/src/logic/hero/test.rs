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

    assert!(unlock_voice(&pool, 14, 3002, 1_300_302).await.is_err());
    unlock_voice(&pool, 14, 3003, 1_300_302).await.unwrap();
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
    assert!(unlock_item(&pool, 15, 3003, 3).await.is_err());
    sqlx::query("UPDATE heroes SET faith = 100000 WHERE user_id = 15 AND hero_id = 3003")
        .execute(&pool)
        .await
        .unwrap();

    let (_, reward) = unlock_item(&pool, 15, 3003, 3).await.unwrap();

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

    rank_up(&pool, 17, skin.character_id).await.unwrap();

    let hero = heroes.get(skin.character_id).await.unwrap();
    assert_eq!(hero.record.rank, 3);
    assert_eq!(hero.record.skin, skin.id);
    assert!(heroes.has_skin(skin.id).await.unwrap());
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

    assert!(use_skin(&pool, 18, 3003, foreign_skin.id).await.is_err());

    let foreign_uid = database::db::game::equipment::add_equipment(&pool, 19, 1000, 1)
        .await
        .unwrap()[0];
    assert!(default_equip(&pool, 18, 3003, foreign_uid).await.is_err());
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
        choice_hero_3123_weapon(&pool, 21, 3003, 1001, 0)
            .await
            .is_err()
    );
    assert!(
        choice_hero_3123_weapon(&pool, 21, 3123, 9999, 0)
            .await
            .is_err()
    );
    assert!(reset_hero_3124_talent_tree(&pool, 21, 3003).await.is_err());
}
