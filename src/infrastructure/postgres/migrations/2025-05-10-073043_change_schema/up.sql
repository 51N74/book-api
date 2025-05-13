-- Your SQL goes here
-- Drop existing tables to start fresh
DROP TABLE IF EXISTS books CASCADE;
DROP TABLE IF EXISTS users CASCADE;
DROP TABLE IF EXISTS categories CASCADE;
DROP TABLE IF EXISTS books_categories_junction CASCADE;
DROP TABLE IF EXISTS admin CASCADE;
DROP TABLE IF EXISTS book_user_junction CASCADE;

-- Create the 'users' table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    "password" VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

-- Create the 'admin' table
CREATE TABLE admin (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    "password" VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

-- Create the 'categories' table
CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    admin_id INTEGER NOT NULL,
    book_id INTEGER NOT NULL,
    "status" VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    deleted_at TIMESTAMP
);

-- Create the 'books' table
CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    "title" VARCHAR NOT NULL,
    "description" TEXT,
    "author" VARCHAR NOT NULL,
    "status" VARCHAR(255) NOT NULL,
    admin_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    price INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    deleted_at TIMESTAMP
);

-- Add foreign key constraint to 'books' table referencing 'categories'
ALTER TABLE
    books
ADD CONSTRAINT fk_book_category FOREIGN KEY (category_id) REFERENCES categories(id);

-- Add foreign key constraint to 'books' table referencing 'admin'
ALTER TABLE
    books
ADD CONSTRAINT fk_book_admin FOREIGN KEY (admin_id) REFERENCES admin(id);

-- Create the 'book_user_junction' table
CREATE TABLE book_user_junction (
    book_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    PRIMARY KEY (book_id, user_id)
);

-- Create the 'books_categories_junction' table
CREATE TABLE books_categories_junction (
    book_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    admin_id INTEGER NOT NULL,
    PRIMARY KEY (book_id, category_id)
);

-- Add foreign key constraint to 'books_categories_junction' referencing 'books'
ALTER TABLE
    books_categories_junction
ADD CONSTRAINT fk_junction_book FOREIGN KEY (book_id) REFERENCES books(id);

-- Add foreign key constraint to 'books_categories_junction' referencing 'categories'
ALTER TABLE
    books_categories_junction
ADD CONSTRAINT fk_junction_category FOREIGN KEY (category_id) REFERENCES categories(id);

-- Add foreign key constraint to 'books_categories_junction' referencing 'admin'
ALTER TABLE
    books_categories_junction
ADD CONSTRAINT fk_junction_admin FOREIGN KEY (admin_id) REFERENCES admin(id);