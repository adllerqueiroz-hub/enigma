CREATE TABLE user_activity_state (
    user_id INTEGER NOT NULL,
    activity_id INTEGER NOT NULL,
    kind INTEGER NOT NULL,
    entry_id INTEGER NOT NULL,
    state INTEGER NOT NULL DEFAULT 0,
    progress INTEGER NOT NULL DEFAULT 0,
    ext TEXT NOT NULL DEFAULT '',
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, activity_id, kind, entry_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_user_activity_state_user ON user_activity_state(user_id, activity_id, kind);
