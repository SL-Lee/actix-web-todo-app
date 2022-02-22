-- Your SQL goes here

CREATE TABLE "user" (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(32) NOT NULL,
    password_hash VARCHAR(96) NOT NULL,
    date_created TIMESTAMP NOT NULL,
    UNIQUE (username)
);
