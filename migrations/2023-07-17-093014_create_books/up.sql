-- Your SQL goes here
CREATE TABLE books (
	id INTEGER NOT NULL PRIMARY KEY,
	title VARCHAR NOT NULL,
	author VARCHAR NOT NULL,
	genre VARCHAR NOT NULL,
	rating INTEGER NOT NULL,
	status VARCHAR NOT NULL,
	start_date DATE,
	end_date DATE
)
