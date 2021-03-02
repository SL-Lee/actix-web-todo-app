table! {
    todo (id) {
        id -> Integer,
        title -> Text,
        contents -> Nullable<Text>,
        completed -> Bool,
        user_id -> Nullable<Integer>,
    }
}

table! {
    user (id) {
        id -> Integer,
        username -> Text,
        password_hash -> Text,
        date_created -> Nullable<Timestamp>,
    }
}

joinable!(todo -> user (user_id));

allow_tables_to_appear_in_same_query!(
    todo,
    user,
);
