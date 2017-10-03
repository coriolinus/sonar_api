-- Your SQL goes here
CREATE TABLE auth_tokens (
   id INTEGER PRIMARY KEY NOT NULL,
   user INTEGER UNIQUE NOT NULL,
   "timestamp" TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
   key TEXT NOT NULL UNIQUE,
   FOREIGN KEY (user) REFERENCES users(id)
);

CREATE UNIQUE INDEX auth_tokens_key_index ON auth_tokens (
   key
);
