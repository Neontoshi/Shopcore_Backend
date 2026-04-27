-- Add migration script here
CREATE TABLE password_resets (
    email TEXT NOT NULL,
    token TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL
);