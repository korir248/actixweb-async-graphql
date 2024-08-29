// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Int4,
        sender -> Int4,
        receiver -> Int4,
        text -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Nullable<Varchar>,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(messages, users,);
