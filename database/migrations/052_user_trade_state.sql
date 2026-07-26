CREATE TABLE IF NOT EXISTS user_trade_tasks (
    user_id INTEGER NOT NULL,
    task_id INTEGER NOT NULL,
    progress INTEGER NOT NULL DEFAULT 0,
    has_finish BOOLEAN NOT NULL DEFAULT 0,
    is_new BOOLEAN NOT NULL DEFAULT 1,
    finish_time INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, task_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_trade_support_bonus (
    user_id INTEGER NOT NULL,
    bonus_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, bonus_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
