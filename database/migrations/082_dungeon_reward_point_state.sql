INSERT INTO user_dungeon_reward_points
    (user_id, chapter_id, reward_point, created_at, updated_at)
SELECT
    id,
    0,
    0,
    CAST(strftime('%s', 'now') AS INTEGER) * 1000,
    CAST(strftime('%s', 'now') AS INTEGER) * 1000
FROM users
WHERE true
ON CONFLICT(user_id, chapter_id) DO NOTHING;
