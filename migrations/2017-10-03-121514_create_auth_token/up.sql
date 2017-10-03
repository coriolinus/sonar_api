-- Your SQL goes here
CREATE TABLE auth_tokens (
   id INTEGER PRIMARY KEY NOT NULL,
   user_id INTEGER UNIQUE NOT NULL,
   "timestamp" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
   key TEXT NOT NULL UNIQUE,
   FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE UNIQUE INDEX auth_tokens_key_index ON auth_tokens (
   key
);

CREATE UNIQUE INDEX auth_tokens_user_index ON auth_tokens (
   user_id
);
