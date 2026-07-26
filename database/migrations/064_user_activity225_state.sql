CREATE TABLE IF NOT EXISTS user_activity225_state (
    user_id INTEGER NOT NULL,
    activity_id INTEGER NOT NULL,
    last_red_envelope_rain_id INTEGER NOT NULL DEFAULT 0,
    question_id INTEGER NOT NULL DEFAULT 0,
    rock_paper_scissors_daily_count INTEGER NOT NULL DEFAULT 0,
    daily_reset_day INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, activity_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
