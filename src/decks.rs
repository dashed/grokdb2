
/* 3rd-party imports */

use rusqlite::{Connection};

/* local imports */

use contexts::{GlobalContext};

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateDeck {
    name: String, // required
    description: String, // required, but may be empty
    parent: Option<u64>,
}


struct Deck {
    id: u64,
    name: String,
    description: String
}

// deck api
impl<'a> GlobalContext<'a> {

    // POST /api/deck
    pub fn create_deck(&self, create_deck_request: CreateDeck) {

        db_write_lock!(db_conn; self.db_connection);
        let db_conn: &Connection = db_conn;
    }
}
