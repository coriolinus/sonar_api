-- Your SQL goes here
CREATE TABLE users (
   id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
   username TEXT UNIQUE NOT NULL,
   password TEXT NOT NULL,
   real_name TEXT NOT NULL DEFAULT '',
   blurb TEXT NOT NULL DEFAULT ''
);

CREATE UNIQUE INDEX users_username_index ON users (
   username
);
