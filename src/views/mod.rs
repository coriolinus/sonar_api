//! Views for the sonar project
//!
//! Views are like Django views: they declare the business logic of the application.
//! However, they also include the routing information.

use rocket_contrib::{Json, Value};

pub mod user_account;
pub use self::user_account::*;

#[error(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}
