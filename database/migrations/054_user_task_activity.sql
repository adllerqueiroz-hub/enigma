CREATE TABLE IF NOT EXISTS user_task_activity (
    user_id INTEGER NOT NULL,
    type_id INTEGER NOT NULL,
    define_id INTEGER NOT NULL DEFAULT 0,
    value INTEGER NOT NULL DEFAULT 0,
    gain_value INTEGER NOT NULL DEFAULT 0,
    expiry_time INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, type_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_user_task_activity_user ON user_task_activity(user_id);
