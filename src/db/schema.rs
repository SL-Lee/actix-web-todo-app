table! {
    todo (id) {
        id -> Uuid,
        title -> Varchar,
        contents -> Nullable<Varchar>,
        completed -> Bool,
        date_created -> Timestamp,
        user_id -> Uuid,
    }
}

table! {
    user (id) {
        id -> Uuid,
        username -> Varchar,
        password_hash -> Varchar,
        date_created -> Timestamp,
    }
}

joinable!(todo -> user (user_id));

allow_tables_to_appear_in_same_query!(
    todo,
    user,
);
