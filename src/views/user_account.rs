//! Views which control user accounts.
//!
//! Essentially, this means CRUD views for user accounts.
//! The only thing really particular about these is that
//! not all use the `TokenAuth` guard; after all, you have
//! to get your token from somewhere.

use db::{Connection, DB};
use diesel::prelude::*;
use diesel::select;
use models::{NewUser, User};
use rocket_contrib::{Json, Value};
use status::Status;

macro_rules! DB_FAILURE {
    () => {
        status!(
            InternalServerError,
            Json(json!({"error": "Failed to connect to backing database"}))
        )
    }
}

macro_rules! or_return {
    ($predicate:expr, $rv_func:expr) => {
        match $predicate {
            Ok(v) => v,
            Err(e) => return $rv_func(e),
        }
    }
}

#[derive(Deserialize)]
struct UserData {
    pub username: String,
    pub password: String,
    pub real_name: Option<String>,
    pub blurb: Option<String>,
}

impl UserData {
    /// Check whether the given user data is valid.
    ///
    /// Return Ok(Self) if so.
    /// Return Err(Json) with an explanation if not.
    fn validate(&self, conn: &Connection) -> Result<(), Status<Json<Value>>> {
        use diesel::expression::dsl::exists;
        use schema::users::dsl::*;

        let username_already_exists: bool = select(
            exists(users.filter(username.eq(&self.username))),
        ).get_result(conn)
            .map_err(|_| DB_FAILURE!())?;

        if username_already_exists {
            return Err(status!(
                BadRequest,
                Json(json!({"error": "Username already in use; pick another"}))
            ));
        }

        if self.password.len() < 16 {
            return Err(status!(
                BadRequest,
                Json(json!({"error": "Password too short"}))
            ));
        }

        Ok(())
    }

    fn into_user(self, conn: &Connection) -> Result<User, Status<Json<Value>>> {
        let new_user = self.validate(conn).map(move |_| {
            NewUser::new(
                self.username,
                self.password,
                self.real_name.unwrap_or(String::new()),
                self.blurb.unwrap_or(String::new()),
            )
        })?;
        new_user.insert(conn).map_err(|_| DB_FAILURE!())
    }
}


fn serialize_user(user: User) -> Json<Value> {
    Json(json!({
        "username": user.username,
        "real_name": user.real_name,
        "blurb": user.blurb,
    }))
}


/// View with which to create a user
#[post("/users", format = "application/json", data = "<user_data>")]
fn create_user(user_data: Json<UserData>, db: DB) -> Status<Json<Value>> {
    let conn = db.conn();
    // https://stackoverflow.com/questions/46905070/
    let user = or_return!(user_data.into_inner().into_user(&conn), |e| e);
    status!(
        Created,
        format!("/users/{}", user.username),
        Some(serialize_user(user))
    )
}

/// View with which to get a user
#[get("/users/<username>")]
fn get_user(username: String, db: DB) -> Status<Json<Value>> {
    let conn = db.conn();
    unimplemented!()
}
