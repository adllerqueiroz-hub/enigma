CREATE TABLE IF NOT EXISTS user_turnback_state (
    user_id INTEGER NOT NULL PRIMARY KEY,
    turnback_id INTEGER NOT NULL,
    bonus_point INTEGER NOT NULL DEFAULT 0,
    first_show BOOLEAN NOT NULL DEFAULT 0,
    has_get_task_bonus TEXT NOT NULL DEFAULT '[]',
    sign_in_day INTEGER NOT NULL DEFAULT 0,
    once_bonus BOOLEAN NOT NULL DEFAULT 0,
    start_time INTEGER NOT NULL DEFAULT 0,
    end_time INTEGER NOT NULL DEFAULT 0,
    remain_addition_count INTEGER NOT NULL DEFAULT 0,
    leave_time INTEGER NOT NULL DEFAULT 0,
    month_card_added_buy_count INTEGER NOT NULL DEFAULT 0,
    version INTEGER NOT NULL DEFAULT 0,
    buy_double_bonus BOOLEAN NOT NULL DEFAULT 0,
    get_daily_bonus INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_turnback_sign_ins (
    user_id INTEGER NOT NULL,
    turnback_id INTEGER NOT NULL,
    day INTEGER NOT NULL,
    state INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, turnback_id, day),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_turnback_drops (
    user_id INTEGER NOT NULL,
    turnback_id INTEGER NOT NULL,
    drop_id INTEGER NOT NULL,
    current_num INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, turnback_id, drop_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
