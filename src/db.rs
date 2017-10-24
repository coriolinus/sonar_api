use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use rocket::http::Status;
use rocket::request::{Request, FromRequest, Outcome};
use rocket::outcome::Outcome::*;
use r2d2::{Pool, PooledConnection, GetTimeout, Config};
use r2d2_diesel::ConnectionManager;
use std::env;

pub type SqliteConnectionPool = Pool<ConnectionManager<SqliteConnection>>;
pub type PooledSqliteConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

lazy_static! {
    pub static ref DATABASE_URL: String = {
        dotenv().ok();
        env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    };

    pub static ref CONNECTION_POOL: SqliteConnectionPool = create_connection_pool();
}

/// Establish a database connection
///
/// Normally it's preferable to get one out of the connection pool,
/// but this works to create one from scratch.
pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish(&DATABASE_URL).expect(&format!(
        "Error connecting to {}",
        *DATABASE_URL
    ))
}

pub fn create_connection_pool() -> SqliteConnectionPool {
    let config = Config::default();
    let manager = ConnectionManager::<SqliteConnection>::new(DATABASE_URL.clone());
    Pool::new(config, manager).expect("Failed to create pool.")
}

/// Database connection request guard
///
/// Add this guard to your request to get a connection to the DB
pub struct DB(PooledSqliteConnection);

impl DB {
    pub fn conn(&self) -> &SqliteConnection {
        &*self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DB {
    type Error = GetTimeout;
    fn from_request(_: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match CONNECTION_POOL.get() {
            Ok(conn) => Success(DB(conn)),
            Err(e) => Failure((Status::InternalServerError, e)),
        }
    }
}
