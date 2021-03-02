-- Your SQL goes here

CREATE TABLE user (
    id INTEGER NOT NULL,
    username VARCHAR(32) NOT NULL,
    password_hash VARCHAR(88) NOT NULL,
    date_created DATETIME,
    PRIMARY KEY (id),
    UNIQUE (username)
)
