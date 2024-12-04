-- Your SQL goes here
create TABLE todos(
    id SERIAL PRIMARY KEY,
    title VARCHAR not null,
    body TEXT NOT NULL,
    start_time TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now()
);