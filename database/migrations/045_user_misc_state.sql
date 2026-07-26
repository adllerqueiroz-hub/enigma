CREATE TABLE user_setting_infos (
    user_id INTEGER NOT NULL,
    type INTEGER NOT NULL,
    param TEXT NOT NULL,
    PRIMARY KEY (user_id, type),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE user_handbook_reads (
    user_id INTEGER NOT NULL,
    type INTEGER NOT NULL,
    handbook_id INTEGER NOT NULL,
    is_read BOOLEAN NOT NULL DEFAULT 1,
    PRIMARY KEY (user_id, type, handbook_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE user_handbook_fragments (
    user_id INTEGER NOT NULL,
    element INTEGER NOT NULL,
    dialog_ids TEXT NOT NULL DEFAULT '[]',
    PRIMARY KEY (user_id, element),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE user_unlock_vouchers (
    user_id INTEGER NOT NULL,
    voucher_id INTEGER NOT NULL,
    get_time INTEGER NOT NULL,
    PRIMARY KEY (user_id, voucher_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

ALTER TABLE user_room_state ADD COLUMN room_level INTEGER NOT NULL DEFAULT 0;
ALTER TABLE user_room_state ADD COLUMN room_theme_ids TEXT NOT NULL DEFAULT '[]';
ALTER TABLE user_room_state ADD COLUMN room_skin_ids TEXT NOT NULL DEFAULT '[]';
ALTER TABLE user_room_state ADD COLUMN have_fishing_bonus BOOLEAN NOT NULL DEFAULT 0;

CREATE TABLE user_room_plans (
    user_id INTEGER NOT NULL,
    plan_id INTEGER NOT NULL,
    name TEXT NOT NULL DEFAULT '',
    cover_id INTEGER NOT NULL DEFAULT 0,
    block_infos TEXT NOT NULL DEFAULT '[]',
    building_infos TEXT NOT NULL DEFAULT '[]',
    skins TEXT NOT NULL DEFAULT '[]',
    building_degree INTEGER NOT NULL DEFAULT 0,
    block_count INTEGER NOT NULL DEFAULT 0,
    share_code TEXT NOT NULL DEFAULT '',
    use_count INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, plan_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE user_room_logs (
    user_id INTEGER NOT NULL,
    id INTEGER NOT NULL,
    type INTEGER NOT NULL,
    time INTEGER NOT NULL,
    hero_id INTEGER NOT NULL DEFAULT 0,
    is_new BOOLEAN NOT NULL DEFAULT 1,
    PRIMARY KEY (user_id, id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE user_power_maker_state (
    user_id INTEGER PRIMARY KEY,
    status INTEGER NOT NULL DEFAULT 0,
    next_remain_second INTEGER NOT NULL DEFAULT 0,
    make_count INTEGER NOT NULL DEFAULT 0,
    logout_second INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE user_battle_pass_state (
    user_id INTEGER NOT NULL,
    bp_id INTEGER NOT NULL,
    score INTEGER NOT NULL DEFAULT 0,
    weekly_score INTEGER NOT NULL DEFAULT 0,
    pay_status INTEGER NOT NULL DEFAULT 0,
    first_show BOOLEAN NOT NULL DEFAULT 1,
    sp_first_show BOOLEAN NOT NULL DEFAULT 1,
    has_get_self_select_bonus TEXT NOT NULL DEFAULT '[]',
    has_get_free_bonus TEXT NOT NULL DEFAULT '[]',
    has_get_pay_bonus TEXT NOT NULL DEFAULT '[]',
    has_get_sp_free_bonus TEXT NOT NULL DEFAULT '[]',
    has_get_sp_pay_bonus TEXT NOT NULL DEFAULT '[]',
    updated_at INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, bp_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_user_unlock_vouchers_user ON user_unlock_vouchers(user_id);
CREATE INDEX idx_user_room_logs_user ON user_room_logs(user_id);
CREATE INDEX idx_user_battle_pass_state_user ON user_battle_pass_state(user_id);
