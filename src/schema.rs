// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
        name -> Varchar,
        surname -> Varchar,
        country -> Varchar,
    }
}
