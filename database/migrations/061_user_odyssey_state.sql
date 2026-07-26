CREATE TABLE IF NOT EXISTS user_odyssey_state (
    user_id INTEGER NOT NULL PRIMARY KEY,
    exp INTEGER NOT NULL DEFAULT 0,
    level INTEGER NOT NULL DEFAULT 1,
    params TEXT NOT NULL DEFAULT '',
    curr_element_id INTEGER NOT NULL DEFAULT 0,
    talent_point INTEGER NOT NULL DEFAULT 0,
    cassandra_tree TEXT NOT NULL DEFAULT '',
    next_mercenary_refresh INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_odyssey_maps (
    user_id INTEGER NOT NULL,
    map_id INTEGER NOT NULL,
    explore_value INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, map_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_odyssey_elements (
    user_id INTEGER NOT NULL,
    element_id INTEGER NOT NULL,
    status INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, element_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_odyssey_talents (
    user_id INTEGER NOT NULL,
    node_id INTEGER NOT NULL,
    level INTEGER NOT NULL DEFAULT 1,
    consume INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, node_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_odyssey_items (
    user_id INTEGER NOT NULL,
    uid INTEGER NOT NULL,
    item_id INTEGER NOT NULL,
    count INTEGER NOT NULL DEFAULT 0,
    new_flag BOOLEAN NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, uid),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
