CREATE TABLE user_open_infos (
    user_id INTEGER NOT NULL,
    open_id INTEGER NOT NULL,
    is_open BOOLEAN NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, open_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_user_open_infos_user ON user_open_infos(user_id);
