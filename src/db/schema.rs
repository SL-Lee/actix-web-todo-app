// @generated automatically by Diesel CLI.

diesel::table! {
    todo (id) {
        id -> Uuid,
        #[max_length = 100]
        title -> Varchar,
        #[max_length = 512]
        contents -> Nullable<Varchar>,
        completed -> Bool,
        date_created -> Timestamp,
        user_id -> Uuid,
    }
}

diesel::table! {
    user (id) {
        id -> Uuid,
        #[max_length = 32]
        username -> Varchar,
        #[max_length = 97]
        password_hash -> Varchar,
        date_created -> Timestamp,
    }
}

diesel::joinable!(todo -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    todo,
    user,
);
