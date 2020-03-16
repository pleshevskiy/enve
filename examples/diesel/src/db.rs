use super::config;
use diesel::prelude::*;
use diesel::pg::PgConnection;

pub fn establish_connection() -> PgConnection {
    let database_url = config::DATABASE_URL();
    PgConnection::establish(database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
