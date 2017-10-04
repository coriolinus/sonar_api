//! Views for the sonar project
//!
//! Views are like Django views: they declare the business logic of the application.
//! However, they also include the routing information.

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
