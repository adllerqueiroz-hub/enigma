CREATE TABLE IF NOT EXISTS user_activity229_stages (
    user_id INTEGER NOT NULL,
    activity_id INTEGER NOT NULL,
    stage_id INTEGER NOT NULL,
    star INTEGER NOT NULL DEFAULT 0,
    max_star INTEGER NOT NULL DEFAULT 0,
    round INTEGER NOT NULL DEFAULT 0,
    min_round INTEGER NOT NULL DEFAULT 0,
    heroes_json TEXT NOT NULL DEFAULT '[]',
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, activity_id, stage_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
