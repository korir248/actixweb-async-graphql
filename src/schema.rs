// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Nullable<Varchar>,
        email -> Varchar,
        password -> Varchar,
    }
}
