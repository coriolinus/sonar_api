//! Views which control user accounts.
//!
//! Essentially, this means CRUD views for user accounts.
//! The only thing really particular about these is that
//! not all use the `TokenAuth` guard; after all, you have
//! to get your token from somewhere.

use db::DB;
use diesel::prelude::*;
use diesel::{self, select};
use models::NewUser;
use rocket_contrib::{Json, Value};
use schema::users;
use status::Status;

macro_rules! DB_FAILURE {
    () => {
        status!(
            InternalServerError,
            Json(json!({"error": "Failed to connect to backing database"}))
        )
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
    fn validate(&self, conn: &SqliteConnection) -> Result<(), Status<Json<Value>>> {
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

    fn into_new_user(self, conn: &SqliteConnection) -> Result<NewUser, Status<Json<Value>>> {
        self.validate(conn).map(move |_| {
            NewUser::new(
                &self.username,
                &self.password,
                &self.real_name.unwrap_or(String::new()),
                &self.blurb.unwrap_or(String::new()),
            )
        })
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

/// View with which to create a user
#[post("/users", format = "application/json", data = "<user_data>")]
fn create_user(user_data: Json<UserData>, db: DB) -> Status<Json<Value>> {
    let conn = db.conn();
    // https://stackoverflow.com/questions/46905070/
    let new_user_result = user_data.into_inner().into_new_user(&conn);
    let new_user = or_return!(new_user_result, |e| e);
    or_return!(diesel::insert(&new_user)
        .into(users::table)
        .execute(conn),
        |_| DB_FAILURE!());

    unimplemented!()
}
