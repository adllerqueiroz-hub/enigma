ALTER TABLE user_room_state ADD COLUMN can_share_count INTEGER NOT NULL DEFAULT 10;
ALTER TABLE user_room_state ADD COLUMN can_use_share_count INTEGER NOT NULL DEFAULT 30;
