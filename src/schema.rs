table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        user_id -> Int4,
        publish_at -> Timestamp,
        url -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
        create_at -> Timestamp,
        last_login_at -> Timestamp,
    }
}

joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
