-- Your SQL goes here
CREATE TABLE pings (
   id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
   user INTEGER NOT NULL,
   "timestamp" TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
   content TEXT NOT NULL,
   likes INTEGER NOT NULL DEFAULT 0,
   echoes INTEGER NOT NULL DEFAULT 0,
   FOREIGN KEY (user) REFERENCES users(id)
);

CREATE INDEX pings_user_timestamp_index ON pings (
   user,
   "timestamp" DESC
);
