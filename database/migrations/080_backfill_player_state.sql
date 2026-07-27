INSERT INTO player_state (
    player_id,
    created_at,
    updated_at,
    last_daily_reset_time,
    last_weekly_reset_time,
    last_monthly_reset_time
)
SELECT
    users.id,
    users.created_at,
    users.updated_at,
    users.created_at,
    users.created_at,
    users.created_at
FROM users
LEFT JOIN player_state ON player_state.player_id = users.id
WHERE player_state.player_id IS NULL;
