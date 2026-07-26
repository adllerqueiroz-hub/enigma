use super::*;

#[tokio::test]
async fn finish_guide_persists_step() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::query(
        "CREATE TABLE guide_progress (
                user_id INTEGER NOT NULL,
                guide_id INTEGER NOT NULL,
                step_id INTEGER NOT NULL,
                PRIMARY KEY(user_id, guide_id)
            )",
    )
    .execute(&pool)
    .await
    .unwrap();

    let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
    let _ = config::init(&data_dir);
    let configured_step = config::configs::get()
        .guide_step
        .iter()
        .find(|step| story_requirement(&step.action).is_none())
        .unwrap();
    let guide_id = configured_step.id;
    let expected_step_id = configured_step.step_id;
    finish_guide(&pool, 7, guide_id, expected_step_id)
        .await
        .unwrap();
    let stored_step_id: i32 =
        sqlx::query_scalar("SELECT step_id FROM guide_progress WHERE user_id = 7 AND guide_id = ?")
            .bind(guide_id)
            .fetch_one(&pool)
            .await
            .unwrap();

    assert_eq!(stored_step_id, expected_step_id);

    assert!(
        config::configs::get()
            .guide_step
            .iter()
            .all(|step| step.id != 102 || step.step_id != 0)
    );
    finish_guide(&pool, 8, 102, 0).await.unwrap();
    let start_marker: i32 = sqlx::query_scalar(
        "SELECT step_id FROM guide_progress WHERE user_id = 8 AND guide_id = 102",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(start_marker, 0);
}

#[test]
fn hero_reward_is_derived_from_the_following_guide_step() {
    let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
    let _ = config::init(&data_dir);

    assert_eq!(hero_reward_after_step(101, 84), Some(3028));
    assert_eq!(hero_reward_after_step(i32::MAX, i32::MAX), None);
}

#[tokio::test]
async fn prologue_guide_grants_apple_after_story_once() {
    let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
    let _ = config::init(&data_dir);
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    database::run_migrations(&pool).await.unwrap();
    sqlx::query(
        "INSERT INTO users (id, username, created_at, updated_at)
             VALUES (8, 'prologue', 0, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();
    database::db::starter_data::load_all_starter_data(&pool, 8)
        .await
        .unwrap();

    let heroes = UserHeroModel::new(8, pool.clone());
    assert!(heroes.get_all_heroes().await.unwrap().is_empty());
    assert!(matches!(
        finish_guide(&pool, 8, 101, 84).await,
        Err(AppError::InvalidRequest)
    ));

    crate::logic::story::update_story(&pool, 8, 100017, -1, 0)
        .await
        .unwrap();
    assert!(heroes.get_all_heroes().await.unwrap().is_empty());

    let completion = finish_guide(&pool, 8, 101, 84).await.unwrap();
    let apple = heroes.get_hero(3028).await.unwrap();
    assert_eq!(completion.rewards.hero_ids, [3028]);
    assert_eq!(
        completion
            .group_snapshot
            .unwrap()
            .group_info
            .unwrap()
            .hero_list,
        [apple.record.uid]
    );

    let repeated = finish_guide(&pool, 8, 101, 84).await.unwrap();
    assert!(repeated.rewards.hero_ids.is_empty());
    assert_eq!(heroes.get_all_heroes().await.unwrap().len(), 1);
}
