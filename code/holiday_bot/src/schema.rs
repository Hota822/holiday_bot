table! {
    public_holiday (id) {
        id -> Integer,
        date -> Nullable<Date>,
        title -> Nullable<Varchar>,
    }
}

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
    public_holiday,
    test,
    users,
);
