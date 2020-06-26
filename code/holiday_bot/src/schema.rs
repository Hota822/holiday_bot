table! {
    test (id) {
        id -> Bigint,
        view_time -> Datetime,
        url -> Varchar,
        user_agent -> Varchar,
        referrer -> Varchar,
        device_type -> Tinyint,
    }
}

table! {
    users (id) {
        id -> Integer,
        line_id -> Nullable<Integer>,
    }
}

allow_tables_to_appear_in_same_query!(
    test,
    users,
);
