/* local imports */

use types::DeckID;
use database::Database;

/* ////////////////////////////////////////////////////////////////////////// */

/* Request Context */

#[derive(Clone)]
pub struct Context {
    pub database: Database,
    pub root_deck_id: DeckID,
    pub request_uri: String,

    /* cache */

    pub should_cache: bool
    // TODO: cache
}

impl Context {
    pub fn new(db_connection: Database) -> Self {
        Context {
            database: db_connection,
            root_deck_id: 1,
            request_uri: "".to_string(),

            // cache
            should_cache: false
        }
    }
}
