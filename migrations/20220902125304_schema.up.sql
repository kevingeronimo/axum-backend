-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
	id SERIAL PRIMARY KEY,
	username VARCHAR(20) NOT NULL UNIQUE,
	password VARCHAR(150) NOT NULL
);