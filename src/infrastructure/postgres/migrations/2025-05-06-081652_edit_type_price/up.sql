-- Your SQL goes here
-- Your SQL goes here
-- Your SQL goes here
DROP TABLE IF EXISTS books CASCADE;
DROP TABLE IF EXISTS users CASCADE;
DROP TABLE IF EXISTS categories CASCADE;
DROP TABLE IF EXISTS books_categories_junction CASCADE;
DROP TABLE IF EXISTS admin CASCADE;
Drop TABLE IF EXISTS book_user_junction CASCADE;

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    "password" VARCHAR(255) NOT NULL,
   created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    "title" VARCHAR NOT NULL,
    "description" TEXT,
    "author" VARCHAR NOT NULL,
     "status" VARCHAR(255) NOT NULL,
    category_id INTEGER NOT NULL,
    price INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (category_id) REFERENCES categories(id)
);

CREATE TABLE admin (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    "password" VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()

);

CREATE TABLE book_user_junction (
    quest_id INTEGER NOT NULL,
    adventurer_id INTEGER NOT NULL,
    PRIMARY KEY (quest_id, adventurer_id)
);