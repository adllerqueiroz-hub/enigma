CREATE TABLE IF NOT EXISTS user_necrologist_stories (
    user_id INTEGER NOT NULL,
    story_id INTEGER NOT NULL,
    info TEXT NOT NULL DEFAULT '',
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, story_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_necrologist_story_plots (
    user_id INTEGER NOT NULL,
    story_id INTEGER NOT NULL,
    plot_id INTEGER NOT NULL,
    state INTEGER NOT NULL DEFAULT 1,
    values_json TEXT NOT NULL DEFAULT '{}',
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, story_id, plot_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
