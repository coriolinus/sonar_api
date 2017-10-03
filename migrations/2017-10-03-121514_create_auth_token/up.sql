-- Your SQL goes here
CREATE TABLE auth_token (
   id INTEGER PRIMARY KEY AUTOINCREMENT,
   user INTEGER UNIQUE,
   "timestamp" TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
   key TEXT NOT NULL UNIQUE,
   FOREIGN KEY (user) REFERENCES users(id)
);

CREATE UNIQUE INDEX auth_token_key_index ON auth_token (
   key
);
