CREATE TABLE IF NOT EXISTS user_activity217_state (
    user_id INTEGER NOT NULL,
    activity_id INTEGER NOT NULL,
    exp_episode_count INTEGER NOT NULL DEFAULT 0,
    coin_episode_count INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, activity_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_activity217_type_state (
    user_id INTEGER NOT NULL,
    activity_id INTEGER NOT NULL,
    type INTEGER NOT NULL,
    daily_use_count INTEGER NOT NULL DEFAULT 0,
    total_use_count INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, activity_id, type),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
