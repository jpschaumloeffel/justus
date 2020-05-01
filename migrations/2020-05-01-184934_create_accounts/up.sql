-- Your SQL goes here
CREATE TABLE accounts (
	id SERIAL PRIMARY KEY,
	name VARCHAR NOT NULL,
	balance MONEY NOT NULL DEFAULT 0
)
