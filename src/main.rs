#![recursion_limit = "128"]
#![feature(try_trait)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate argon2rs;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate rand;

mod auth;
mod db;
pub mod models;
mod views;
mod schema;

fn main() {
    rocket::ignite()
        .mount("/v1", routes![views::index])
        .launch();
}
