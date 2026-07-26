use super::*;
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::test]
async fn profile_edits_follow_lua_limits_and_item_ownership() {
    let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
    let _ = config::init(&data_dir);
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    database::run_migrations(&pool).await.unwrap();
    sqlx::query(
        "INSERT INTO users (id, username, created_at, updated_at) VALUES (21, 'profile', 0, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();
    player_infos::create_player_info(&pool, 21, 0)
        .await
        .unwrap();

    set_signature(&pool, config::configs::get(), 21, "hello".into())
        .await
        .unwrap();
    assert!(
        set_signature(&pool, config::configs::get(), 21, "x".repeat(51))
            .await
            .is_err()
    );
    set_birthday(&pool, config::configs::get(), 21, "2000-02-29".into())
        .await
        .unwrap();
    assert!(
        set_birthday(&pool, config::configs::get(), 21, "2001-01-01".into())
            .await
            .is_err()
    );
    assert!(
        set_player_bg(&pool, config::configs::get(), 21, 210001)
            .await
            .is_err()
    );
    items::add_item_quantity(&pool, 21, 210001, 1)
        .await
        .unwrap();
    set_player_bg(&pool, config::configs::get(), 21, 210001)
        .await
        .unwrap();
    set_character_age(&pool, config::configs::get(), 21, vec![3, 1])
        .await
        .unwrap();
    assert!(
        set_character_age(&pool, config::configs::get(), 21, vec![1, 99])
            .await
            .is_err()
    );

    let profile = player_infos::get_player_info(&pool, 21)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(profile.signature, "hello");
    assert_eq!(profile.birthday, "2000-02-29");
    assert_eq!(profile.bg, 210001);
    let wire: sonettobuf::PlayerInfo = player_infos::get_player_info_data(&pool, 21)
        .await
        .unwrap()
        .unwrap()
        .into();
    assert_eq!(wire.character_age, vec![3, 1]);
}
