use diesel::{delete, insert, select};
use diesel::result::Error as ResultError;
use diesel::prelude::*;
use rand::{OsRng, Rng};
use rocket::http::Status;
use rocket::request::{Request, FromRequest, Outcome};
use rocket::outcome::Outcome::*;

use db::CONNECTION_POOL;
use models::{User, Token, NewToken};

/// Token Authentication
pub struct TokenAuth {
    pub user: User,
}

impl TokenAuth {
    /// Invalidate a user's token
    pub fn invalidate_for(user: &User) -> Result<(), &'static str> {
        use schema::auth_tokens::dsl::*;

        let connection = CONNECTION_POOL.get().map_err(
            |_| "Couldn't get connection from pool",
        )?;

        delete(auth_tokens.filter(user_id.eq(user.id)))
            .execute(&*connection)
            .map_err(|_| "Couldn't delete existing keys")?;

        Ok(())
    }

    /// Create and return a token for the specified user.
    ///
    /// - Invalidates any existing user tokens for this user
    /// - Creates a 64-byte ascii-representable secure random token
    /// - ensures that the created token is unique
    /// - inserts the association into the DB for the given user
    ///
    /// Returns the created key
    pub fn create_for(user: &User) -> Result<String, &'static str> {
        use schema::auth_tokens::dsl::*;
        use diesel::expression::dsl::exists;

        let connection = CONNECTION_POOL.get().map_err(
            |_| "Couldn't get connection from pool",
        )?;

        // We need to keep trying random keys until we find an unused one.
        // Typically we'd expect to find this on the first try, but just in case,
        // we make 10 attempts. Normally I'd prefer to use a for loop for this kind
        // of bounded thing, but the simplest way to frame that is to break with a
        // value, which in this version of rust is only allowed from the loop construct.
        let new_key = {
            let mut i = 0;
            loop {
                let proposed_key: String = OsRng::new()
                    .map_err(|_| "Couldn't connect to OS RNG")?
                    .gen_ascii_chars()
                    .take(64)
                    .collect();

                let proposed_key_exists: bool =
                    select(exists(auth_tokens.filter(key.eq(&proposed_key))))
                        .get_result(&*connection)
                        .map_err(|_| "Failed to check for key existence")?;
                if !proposed_key_exists {
                    break Ok(proposed_key);
                }

                i += 1;
                if i >= 10 {
                    break Err("Couldn't find unused key after 10 tries");
                }
            }
        }?;

        delete(auth_tokens.filter(user_id.eq(user.id)))
            .execute(&*connection)
            .map_err(|_| "Couldn't delete existing keys")?;

        insert(&NewToken {
            user_id: user.id,
            key: &new_key,
        }).into(auth_tokens)
            .execute(&*connection)
            .map_err(|_| "Failed to insert key into auth_tokens")?;

        Ok(new_key)
    }
}

macro_rules! try_outcome {
    ($condition:expr; $status:expr) => {
        match $condition {
            Ok(ok) => ok,
            Err(e) => return Failure(($status, e.to_string()))
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for TokenAuth {
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Failure((
                Status::Unauthorized,
                String::from(
                    "`Authorization` header must appear exactly once",
                ),
            ));
        }
        let key = keys[0];
        const TOKEN_PREFIX: &'static str = "Token ";
        if !key.starts_with(TOKEN_PREFIX) {
            return Failure((
                Status::Unauthorized,
                format!(
                    "`Authorization` header must begin with the string '{}'",
                    TOKEN_PREFIX
                ),
            ));
        }
        let incoming_key = &key[TOKEN_PREFIX.len()..];

        let user = {
            // Create a small scope to minimize the amount of time we monopolize
            // the DB connection
            let connection = try_outcome!(CONNECTION_POOL.get(); Status::InternalServerError);

            let token = {
                // encapsulate the use of the dsl
                use schema::auth_tokens::dsl::*;
                match auth_tokens.filter(key.eq(incoming_key)).first::<Token>(
                    &*connection,
                ) {
                    Ok(token) => token,
                    Err(e) => {
                        return if e == ResultError::NotFound {
                            Failure((
                                Status::Forbidden,
                                String::from("Token presented was not valid"),
                            ))
                        } else {
                            Failure((Status::InternalServerError, e.to_string()))
                        }
                    }
                }
            };
            // In the future, we might want to implement token invalidation after some
            // period of time. If that's desired, we should just compare the current time
            // to `token.timestamp`; if that's greater than the invalidation period, then
            // we can return failure. Otherwise, the fact that we found a match for the
            // specified token means that we've logged in successfully.
            {
                // encapsulate this DSL also
                use schema::users::dsl::*;
                match users.find(token.user_id).first::<User>(&*connection) {
                    Ok(user) => user,
                    Err(e) => return Failure((Status::InternalServerError, e.to_string())),
                }
            }
        };

        Success(TokenAuth { user: user })
    }
}
