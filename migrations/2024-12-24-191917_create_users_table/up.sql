-- Your SQL goes here
CREATE TABLE users
(
    id       SERIAL PRIMARY KEY,
    email    VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    name     VARCHAR NOT NULL,
    surname  VARCHAR NOT NULL
);