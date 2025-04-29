// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Int4,
        title -> Text,
        author -> Text,
        category_id -> Text,
        price -> Float4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    books_categories_junction (books_id, categories_id) {
        books_id -> Int4,
        categories_id -> Int4,
    }
}

diesel::table! {
    categories (id) {
        id -> Int4,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(books_categories_junction -> books (books_id));
diesel::joinable!(books_categories_junction -> categories (categories_id));

diesel::allow_tables_to_appear_in_same_query!(
    books,
    books_categories_junction,
    categories,
);
