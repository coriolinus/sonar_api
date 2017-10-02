#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

mod models;
mod views;

fn main() {
    rocket::ignite()
        .mount("/v1", routes![views::index])
        .launch();
}
