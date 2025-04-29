-- Your SQL goes here
CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    category_id TEXT NOT NULL,
    price REAL NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at TIMESTAMP
);

CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at TIMESTAMP
);

CREATE TABLE books_categories_junction (
    books_id INTEGER NOT NULL,
    categories_id INTEGER NOT NULL,
    PRIMARY KEY (books_id, categories_id)
);

ALTER TABLE
    books_categories_junction
ADD
    CONSTRAINT fk_books FOREIGN KEY (books_id) REFERENCES books(id),
ADD
    CONSTRAINT fk_categories FOREIGN KEY (categories_id) REFERENCES categories(id);

    