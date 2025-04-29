// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Uuid,
        title -> Varchar,
        author -> Varchar,
        category_id -> Uuid,
        price -> Numeric,
    }
}

diesel::table! {
    categories (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(books -> categories (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    books,
    categories,
    users,
);
