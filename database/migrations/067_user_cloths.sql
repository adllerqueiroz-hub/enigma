CREATE TABLE user_cloths (
    user_id INTEGER NOT NULL,
    cloth_id INTEGER NOT NULL,
    level INTEGER NOT NULL DEFAULT 1,
    exp INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, cloth_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Preserve the ownership previously exposed to accounts by the static reply.
INSERT INTO user_cloths (user_id, cloth_id, level, exp)
SELECT id, 1, 1, 0 FROM users
UNION ALL SELECT id, 2, 1, 0 FROM users
UNION ALL SELECT id, 6, 1, 0 FROM users;
