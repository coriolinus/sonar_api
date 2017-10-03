-- This file should undo anything in `up.sql`
DROP TABLE auth_token;
DROP INDEX IF EXISTS auth_token_key_index;
DROP INDEX IF EXISTS auth_tokens_user_index;
