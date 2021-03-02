-- Your SQL goes here

CREATE TABLE todo (
    id INTEGER NOT NULL,
    title VARCHAR(100) NOT NULL,
    contents VARCHAR(512),
    completed BOOLEAN NOT NULL,
    user_id INTEGER,
    PRIMARY KEY (id),
    CHECK (completed IN (0, 1)),
    FOREIGN KEY(user_id) REFERENCES user (id)
)
