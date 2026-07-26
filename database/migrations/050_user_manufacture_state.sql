CREATE TABLE IF NOT EXISTS user_manufacture_state (
    user_id INTEGER PRIMARY KEY,
    trade_level INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_manufacture_slots (
    user_id INTEGER NOT NULL,
    building_uid INTEGER NOT NULL,
    slot_id INTEGER NOT NULL,
    priority INTEGER NOT NULL DEFAULT 0,
    production_id INTEGER NOT NULL DEFAULT 0,
    slot_status INTEGER NOT NULL DEFAULT 0,
    inventory_count INTEGER NOT NULL DEFAULT 0,
    begin_time INTEGER NOT NULL DEFAULT 0,
    next_finish_time INTEGER NOT NULL DEFAULT 0,
    pause_time INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, building_uid, slot_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_manufacture_frozen_items (
    user_id INTEGER NOT NULL,
    material_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 0,
    time INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, material_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

