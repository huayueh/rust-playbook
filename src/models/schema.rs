table! {
    users (id) {
        id -> Bigint,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    posts (id) {
        id -> Bigint,
        user_id -> Bigint,
        title -> Varchar,
        content -> Text,
        is_read -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
