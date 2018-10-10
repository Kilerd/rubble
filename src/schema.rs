table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        user_id -> Nullable<Int4>,
        publish_at -> Nullable<Timestamp>,
        url -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Nullable<Text>,
        password -> Nullable<Text>,
        create_at -> Nullable<Timestamp>,
        last_login_at -> Nullable<Timestamp>,
    }
}

joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
