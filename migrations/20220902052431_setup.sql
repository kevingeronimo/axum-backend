-- Add migration script here
DROP TABLE IF EXISTS users;
CREATE TABLE users (
	id SERIAL PRIMARY KEY,
	username VARCHAR(20) NOT NULL UNIQUE,
	"password" VARCHAR(20) NOT NULL
);