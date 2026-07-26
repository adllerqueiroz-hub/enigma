CREATE TABLE IF NOT EXISTS user_hero_story_state (
    user_id INTEGER NOT NULL,
    story_id INTEGER NOT NULL,
    progress INTEGER NOT NULL DEFAULT 0,
    get_reward BOOLEAN NOT NULL DEFAULT 0,
    get_score_bonus TEXT NOT NULL DEFAULT '[]',
    score INTEGER NOT NULL DEFAULT 0,
    challenge_wave INTEGER NOT NULL DEFAULT 0,
    challenge_max_wave INTEGER NOT NULL DEFAULT 0,
    get_challenge_reward BOOLEAN NOT NULL DEFAULT 0,
    unlock BOOLEAN NOT NULL DEFAULT 0,
    is_new BOOLEAN NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, story_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

