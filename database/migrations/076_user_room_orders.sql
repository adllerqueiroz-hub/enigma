CREATE TABLE user_room_order_state (
    user_id INTEGER PRIMARY KEY,
    purchase_order_finish_count INTEGER NOT NULL DEFAULT 0,
    remain_refresh_count INTEGER NOT NULL DEFAULT -1,
    weekly_wholesale_revenue INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE user_room_purchase_orders (
    user_id INTEGER NOT NULL,
    order_id INTEGER NOT NULL,
    last_refresh_time INTEGER NOT NULL,
    buyer_id INTEGER NOT NULL,
    is_advanced BOOLEAN NOT NULL DEFAULT 0,
    is_traced BOOLEAN NOT NULL DEFAULT 0,
    refresh_type INTEGER NOT NULL DEFAULT 1,
    quality INTEGER NOT NULL,
    is_locked BOOLEAN NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, order_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE user_room_purchase_order_goods (
    user_id INTEGER NOT NULL,
    order_id INTEGER NOT NULL,
    production_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    PRIMARY KEY (user_id, order_id, production_id),
    FOREIGN KEY (user_id, order_id)
        REFERENCES user_room_purchase_orders(user_id, order_id) ON DELETE CASCADE
);

CREATE TABLE user_room_wholesale_orders (
    user_id INTEGER NOT NULL,
    order_id INTEGER NOT NULL,
    good_id INTEGER NOT NULL,
    today_sold_count INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, order_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
