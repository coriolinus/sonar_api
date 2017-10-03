//! Models for sonar go here
use chrono::NaiveDateTime;
use super::schema::{users, pings, auth_tokens};

#[derive(Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub real_name: String,
    pub blurb: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub real_name: &'a str,
    pub blurb: &'a str,
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
