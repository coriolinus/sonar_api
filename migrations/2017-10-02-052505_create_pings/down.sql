-- This file should undo anything in `up.sql`
DROP TABLE pings;
DROP INDEX IF EXISTS pings_user_timestamp_index;
