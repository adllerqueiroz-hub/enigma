CREATE TABLE user_instruction_dungeon_state (
    user_id INTEGER PRIMARY KEY,
    get_final_reward BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE user_instruction_dungeon_unlocks (
    user_id INTEGER NOT NULL,
    instruction_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, instruction_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE user_instruction_dungeon_rewards (
    user_id INTEGER NOT NULL,
    reward_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, reward_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE user_instruction_dungeon_opens (
    user_id INTEGER NOT NULL,
    instruction_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, instruction_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
