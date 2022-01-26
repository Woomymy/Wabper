-- Your SQL goes here
CREATE TABLE pastes (
    id SERIAL PRIMARY KEY,
    body TEXT NOT NULL,
    title TEXT NOT NULL,
    author TEXT NOT NULL
)