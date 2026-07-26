CREATE TABLE IF NOT EXISTS user_rouge2_state (
    user_id INTEGER NOT NULL PRIMARY KEY,
    state INTEGER NOT NULL DEFAULT 0,
    difficulty INTEGER NOT NULL DEFAULT 0,
    coin INTEGER NOT NULL DEFAULT 0,
    end_id INTEGER NOT NULL DEFAULT 0,
    game_num INTEGER NOT NULL DEFAULT 0,
    genius_point INTEGER NOT NULL DEFAULT 0,
    genius_ids TEXT NOT NULL DEFAULT '[]',
    reward_point INTEGER NOT NULL DEFAULT 0,
    max_difficulty INTEGER NOT NULL DEFAULT 0,
    pass_layer_ids TEXT NOT NULL DEFAULT '[]',
    pass_event_ids TEXT NOT NULL DEFAULT '[]',
    pass_end_ids TEXT NOT NULL DEFAULT '[]',
    pass_entrust_ids TEXT NOT NULL DEFAULT '[]',
    pass_collections TEXT NOT NULL DEFAULT '[]',
    last_game_time INTEGER NOT NULL DEFAULT 0,
    hotfix_str TEXT NOT NULL DEFAULT '',
    updated_at INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_rouge2_unlocks (
    user_id INTEGER NOT NULL,
    unlock_type INTEGER NOT NULL,
    unlock_id INTEGER NOT NULL,
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, unlock_type, unlock_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_rouge2_career_levels (
    user_id INTEGER NOT NULL,
    career_id INTEGER NOT NULL,
    exp INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, career_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_rouge2_rewards (
    user_id INTEGER NOT NULL,
    reward_id INTEGER NOT NULL,
    buy_count INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, reward_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_rouge2_materials (
    user_id INTEGER NOT NULL,
    material_id INTEGER NOT NULL,
    num INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, material_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
