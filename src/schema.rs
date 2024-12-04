// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        start_time -> Timestamp,
        created_at -> Timestamp,
    }
}
