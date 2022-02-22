-- Your SQL goes here

CREATE TABLE todo (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(100) NOT NULL,
    contents VARCHAR(512),
    completed BOOLEAN NOT NULL,
    date_created TIMESTAMP NOT NULL,
    user_id UUID REFERENCES "user" (id) NOT NULL
);
