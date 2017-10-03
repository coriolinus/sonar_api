use diesel::result::Error as ResultError;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::request::{Request, FromRequest, Outcome};
use rocket::outcome::Outcome::*;

use db::CONNECTION_POOL;
use models::{User, Token};

/// Token Authentication
pub struct TokenAuth {
    pub user: User,
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
                match users.find(token.user).first::<User>(&*connection) {
                    Ok(user) => user,
                    Err(e) => return Failure((Status::InternalServerError, e.to_string())),
                }
            }
        };

        Success(TokenAuth { user: user })
    }
}
