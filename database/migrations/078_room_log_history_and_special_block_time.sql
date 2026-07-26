CREATE TABLE user_room_logs_new (
    log_uid INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    id INTEGER NOT NULL,
    type INTEGER NOT NULL,
    time INTEGER NOT NULL,
    hero_id INTEGER NOT NULL DEFAULT 0,
    is_new BOOLEAN NOT NULL DEFAULT 1,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

INSERT INTO user_room_logs_new (user_id, id, type, time, hero_id, is_new)
SELECT user_id, id, type, time, hero_id, is_new
FROM user_room_logs
ORDER BY time, id;

DROP TABLE user_room_logs;
ALTER TABLE user_room_logs_new RENAME TO user_room_logs;
CREATE INDEX idx_user_room_logs_user ON user_room_logs(user_id);

UPDATE user_special_blocks
SET create_time = create_time / 1000
WHERE create_time > 2147483647;
