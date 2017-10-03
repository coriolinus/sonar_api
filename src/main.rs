#![recursion_limit = "128"]
#![feature(try_trait)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate argon2rs;
extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate r2d2;
extern crate r2d2_diesel;


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
