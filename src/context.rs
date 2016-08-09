/* local imports */

use database::Database;

/* ////////////////////////////////////////////////////////////////////////// */

/* Request Context */

#[derive(Clone)]
pub struct Context {
    pub database: Database,

    // TODO: cache
}

impl Context {
    pub fn new(db_connection: Database) -> Self {
        Context {
            database: db_connection
        }
    }
}
