//! Models for sonar go here
use auth::pw::SaltyPassword;
use chrono::NaiveDateTime;
use db::Connection;
use diesel;
use diesel::prelude::*;
use diesel::result::QueryResult;
use schema::{users, pings, auth_tokens};

#[derive(Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    password: String,
    pub real_name: String,
    pub blurb: String,
}

impl User {
    /// Validated a given username and plaintext password
    ///
    /// Return `true` if the given username exists and matches the given password
    fn validate(conn: &Connection, username: &str, password: &str) -> bool {
        unimplemented!()
    }

    /// Get the User object corresponding to a given username and plaintext password
    fn get_validated(conn: &Connection, username: &str, password: &str) -> QueryResult<User> {
        unimplemented!()
    }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    username: String,
    password: String,
    real_name: String,
    blurb: String,
}

impl NewUser {
    pub fn new(username: String, password: String, real_name: String, blurb: String) -> NewUser {
        NewUser {
            username: username,
            password: SaltyPassword::new(&password).to_string(),
            real_name: real_name,
            blurb: blurb,
        }
    }

    pub fn insert(self, conn: &Connection) -> QueryResult<User> {
        use schema::users::dsl::*;
        diesel::insert(&self)
            .into(users)
            // ideally we'd use .get_result(conn) here instead of
            // .execute(conn), because we'd prefer to fetch the
            // newly inserted row immediately. Unfortunately,
            // SQLite doesn't support that, so we're stuck making
            // another query to fetch it.
            .execute(conn)?;

        users.filter(username.eq(&self.username)).first::<User>(
            conn,
        )
    }
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(User)]
pub struct Ping {
    pub id: i32,
    pub user_id: i32,
    pub timestamp: NaiveDateTime,
    pub content: String,
    pub likes: u32,
    pub echoes: u32,
}

#[derive(Insertable)]
#[table_name = "pings"]
pub struct NewPing<'a> {
    pub user_id: i32,
    pub content: &'a str,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(User)]
#[table_name = "auth_tokens"]
pub struct Token {
    pub id: i32,
    pub user_id: i32,
    pub timestamp: NaiveDateTime,
    pub key: String,
}

#[derive(Insertable)]
#[table_name = "auth_tokens"]
pub struct NewToken<'a> {
    pub user_id: i32,
    pub key: &'a str,
}
