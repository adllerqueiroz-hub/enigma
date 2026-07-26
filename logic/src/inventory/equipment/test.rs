use super::{decompose_config, decompose_count, refine, valid_strengthen_consumes};
use sqlx::SqlitePool;

#[test]
fn decompose_reward_uses_configured_rarity_exp() {
    assert_eq!(decompose_count("2#200|3#300", [2, 3], 1), Some(5));
    assert_eq!(decompose_config("9#999#1"), Some((999, 1)));
}

#[test]
fn strengthen_rejects_empty_duplicate_and_non_positive_fodder() {
    assert!(!valid_strengthen_consumes(&[]));
    assert!(!valid_strengthen_consumes(&[(1, 1), (1, 1)]));
    assert!(!valid_strengthen_consumes(&[(1, 0)]));
    assert!(valid_strengthen_consumes(&[(1, 1), (2, 2)]));
}

#[tokio::test]
async fn refine_rejects_the_whole_request_when_any_fodder_is_invalid() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    database::run_migrations(&pool).await.unwrap();
    sqlx::query(
        "INSERT INTO users (id, username, created_at, updated_at) VALUES (20, 'refine', 0, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();
    for (uid, equip_id, locked) in [(1_i64, 2000, false), (2, 2000, false), (3, 2000, true)] {
        sqlx::query(
            "INSERT INTO equipment
             (uid, user_id, equip_id, level, exp, break_lv, count, is_lock, refine_lv, created_at, updated_at)
             VALUES (?, 20, ?, 1, 0, 0, 1, ?, 1, 0, 0)",
        )
        .bind(uid)
        .bind(equip_id)
        .bind(locked)
        .execute(&pool)
        .await
        .unwrap();
    }

    assert!(refine(&pool, 20, 1, vec![2, 3]).await.is_err());
    let target_level: i32 = sqlx::query_scalar("SELECT refine_lv FROM equipment WHERE uid = 1")
        .fetch_one(&pool)
        .await
        .unwrap();
    let fodder_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM equipment WHERE uid IN (2, 3)")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(target_level, 1);
    assert_eq!(fodder_count, 2);
}
