// @generated automatically by Diesel CLI.

diesel::table! {
    admin (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    book_user_junction (book_id, user_id) {
        book_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        author -> Varchar,
        #[max_length = 255]
        status -> Varchar,
        admin_id -> Int4,
        category_id -> Int4,
        price -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    books_categories_junction (book_id, category_id) {
        book_id -> Int4,
        category_id -> Int4,
        admin_id -> Int4,
    }
}

diesel::table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
        admin_id -> Int4,
        book_id -> Int4,
        #[max_length = 255]
        status -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(books -> admin (admin_id));
diesel::joinable!(books -> categories (category_id));
diesel::joinable!(books_categories_junction -> admin (admin_id));
diesel::joinable!(books_categories_junction -> books (book_id));
diesel::joinable!(books_categories_junction -> categories (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    admin,
    book_user_junction,
    books,
    books_categories_junction,
    categories,
    users,
);
