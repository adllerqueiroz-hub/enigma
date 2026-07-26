CREATE TABLE IF NOT EXISTS user_dice_hero_chapters (
    user_id INTEGER NOT NULL,
    chapter INTEGER NOT NULL,
    current_hero_id INTEGER NOT NULL DEFAULT 0,
    relic_ids TEXT NOT NULL DEFAULT '[]',
    skill_card_ids TEXT NOT NULL DEFAULT '[]',
    pass_level_ids TEXT NOT NULL DEFAULT '[]',
    reward_items_json TEXT NOT NULL DEFAULT '[]',
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, chapter),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
