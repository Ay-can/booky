// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Integer,
        title -> Text,
        author -> Text,
        genre -> Text,
        rating -> Integer,
        status -> Text,
        start_date -> Nullable<Date>,
        end_date -> Nullable<Date>,
    }
}
