use super::cfg;
use diesel::prelude::*;
use diesel::pg::PgConnection;

pub fn establish_connection() -> PgConnection {
    let database_url = cfg::DATABASE_URL();
    PgConnection::establish(database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
