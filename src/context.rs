/* local imports */

use types::DeckID;
use database::Database;

/* ////////////////////////////////////////////////////////////////////////// */

/* Request Context */

#[derive(Clone)]
pub struct Context {

    pub root_deck_id: DeckID,

    pub database: Database,

    // TODO: cache
}

impl Context {
    pub fn new(db_connection: Database) -> Self {
        Context {
            root_deck_id: 1, // default
            database: db_connection
        }
    }
}
