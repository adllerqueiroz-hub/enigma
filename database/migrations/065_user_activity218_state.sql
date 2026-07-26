CREATE TABLE IF NOT EXISTS user_activity218_state (
    user_id INTEGER NOT NULL,
    activity_id INTEGER NOT NULL,
    finish_game_count INTEGER NOT NULL DEFAULT 0,
    total_coin_num INTEGER NOT NULL DEFAULT 0,
    accepted_reward_id INTEGER NOT NULL DEFAULT 0,
    game_record TEXT NOT NULL DEFAULT '',
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, activity_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
