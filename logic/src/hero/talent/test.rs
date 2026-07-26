use super::*;

#[tokio::test]
async fn template_commands_mutate_the_owned_template() {
    let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
    let _ = config::init(&data_dir);
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    database::run_migrations(&pool).await.unwrap();
    sqlx::query(
        "INSERT INTO users (id, username, created_at, updated_at)
             VALUES (24, 'talent', 0, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();
    let hero = UserHeroModel::new(24, pool.clone());
    hero.create_hero(3003).await.unwrap();
    hero.replace_talent_cubes(3003, 1, vec![(1, 0, 0, 0)])
        .await
        .unwrap();

    let manager = HeroManager::new(24);
    let renamed = manager
        .rename_talent_template(&pool, 3003, 1, "  Alpha  ".into())
        .await
        .unwrap();
    assert_eq!(
        renamed
            .template_info
            .as_ref()
            .and_then(|template| template.name.as_deref()),
        Some("Alpha")
    );
    assert!(
        manager
            .rename_talent_template(&pool, 3003, 1, "12345678901".into())
            .await
            .is_err()
    );

    let (reply, hero_info) = manager
        .takeoff_all_talent_cubes(&pool, 3003, 1)
        .await
        .unwrap();
    assert!(reply.template_info.unwrap().talent_cube_infos.is_empty());
    assert!(hero_info.talent_cube_infos.is_empty());
}
