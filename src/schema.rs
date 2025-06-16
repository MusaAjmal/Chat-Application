diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    messages (id) {
        id -> Uuid,
        sender_id -> Uuid,
        content -> Text,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(users, messages);
