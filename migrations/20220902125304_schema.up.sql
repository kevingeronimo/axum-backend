-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
	id bigserial PRIMARY KEY,
	username text NOT NULL UNIQUE,
	password text NOT NULL
);