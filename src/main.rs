#![recursion_limit = "128"]
#![feature(conservative_impl_trait)]
#![feature(plugin)]
#![feature(try_trait)]
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
#[macro_use]
extern crate serde_derive;


pub mod auth;
pub mod db;
mod models;
pub mod status;
mod schema;
mod views;

use views::*;

fn main() {
    rocket::ignite()
        .mount("/v1", routes![create_user])
        .catch(errors![not_found])
        .launch();
}
