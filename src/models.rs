//! Models for sonar go here
use diesel::types::Timestamp;
use super::schema::{users, pings};

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub handle: String,
    pub real_name: String,
    pub blurb: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub handle: &'a str,
    pub real_name: &'a str,
    pub blurb: &'a str,
}

#[derive(Queryable)]
pub struct Ping {
    pub id: i32,
    pub user: i32,
    pub timestamp: Timestamp,
    pub content: String,
    pub likes: u32,
    pub echoes: u32,
}

#[derive(Insertable)]
#[table_name = "pings"]
pub struct NewPing<'a> {
    pub user: i32,
    pub content: &'a str,
}
