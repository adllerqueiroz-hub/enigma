CREATE TABLE IF NOT EXISTS user_tasks (
    user_id INTEGER NOT NULL,
    type_id INTEGER NOT NULL,
    task_id INTEGER NOT NULL,
    progress INTEGER NOT NULL DEFAULT 0,
    has_finished BOOLEAN NOT NULL DEFAULT 0,
    finish_count INTEGER NOT NULL DEFAULT 0,
    expiry_time INTEGER NOT NULL DEFAULT 0,
    min_type_id INTEGER NOT NULL DEFAULT 0,
    activity_id INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, type_id, task_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_user_tasks_user_type ON user_tasks(user_id, type_id);
CREATE INDEX IF NOT EXISTS idx_user_tasks_task ON user_tasks(user_id, task_id);
