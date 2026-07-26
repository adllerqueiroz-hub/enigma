CREATE TABLE user_tower_compose_theme_state (
    user_id INTEGER NOT NULL,
    theme_id INTEGER NOT NULL,
    research_progress INTEGER NOT NULL DEFAULT 0,
    pass_max_layer_id INTEGER NOT NULL DEFAULT 0,
    high_score INTEGER NOT NULL DEFAULT 0,
    curr_score INTEGER NOT NULL DEFAULT 0,
    boss_level INTEGER NOT NULL DEFAULT 0,
    boss_lock BOOLEAN NOT NULL DEFAULT 0,
    saved_record BOOLEAN NOT NULL DEFAULT 0,
    params TEXT NOT NULL DEFAULT '',
    updated_at INTEGER NOT NULL,
    PRIMARY KEY (user_id, theme_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE user_tower_compose_plane_mods (
    user_id INTEGER NOT NULL,
    theme_id INTEGER NOT NULL,
    plane_id INTEGER NOT NULL,
    mods_json TEXT NOT NULL,
    updated_at INTEGER NOT NULL,
    PRIMARY KEY (user_id, theme_id, plane_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
