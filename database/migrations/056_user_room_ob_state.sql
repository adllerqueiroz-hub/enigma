CREATE TABLE IF NOT EXISTS user_room_formulas (
    user_id INTEGER NOT NULL,
    formula_id INTEGER NOT NULL,
    count INTEGER NOT NULL DEFAULT 1,
    PRIMARY KEY (user_id, formula_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_room_production_lines (
    user_id INTEGER NOT NULL,
    line_id INTEGER NOT NULL,
    formula_id INTEGER NOT NULL DEFAULT 0,
    finish_count INTEGER NOT NULL DEFAULT 0,
    next_finish_time INTEGER NOT NULL DEFAULT 0,
    pause_time INTEGER NOT NULL DEFAULT 0,
    level INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, line_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_room_skins (
    user_id INTEGER NOT NULL,
    part_id INTEGER NOT NULL,
    skin_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, part_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_room_heroes (
    user_id INTEGER NOT NULL,
    hero_id INTEGER NOT NULL,
    current_faith INTEGER NOT NULL DEFAULT 0,
    next_refresh_time INTEGER NOT NULL DEFAULT 0,
    skin INTEGER NOT NULL DEFAULT 0,
    current_minute INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, hero_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
