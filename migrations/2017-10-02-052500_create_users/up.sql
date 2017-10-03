-- Your SQL goes here
CREATE TABLE users (
   id INTEGER PRIMARY KEY AUTOINCREMENT,
   username TEXT UNIQUE NOT NULL,
   password TEXT NOT NULL,
   real_name TEXT NOT NULL DEFAULT '',
   blurb TEXT NOT NULL DEFAULT ''
)
