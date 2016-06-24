use std::sync::{Arc, Mutex, RwLock};

use rusqlite::{Connection};

use database::{QueryError};

const SETUP: [&'static str; 5] = [

    // decks

    DECKS,
    DECKSCLOSURE,

    // decks/indices

    DECKSCLOSURE_DEPTH_INDEX,

    // decks/triggers

    DECK_ON_UPDATE_TRIGGER,
    DECKSCLOSURE_NEW_DECK_TRIGGER,
];

/* decks */

// note: updated_at is when the deck was modified, not when it was reviewed.
// note: CHECK (name <> '') ensures name is non-empty string
const DECKS: &'static str = "
CREATE TABLE IF NOT EXISTS Decks (
    deck_id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',

    created_at INT NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INT NOT NULL DEFAULT (strftime('%s', 'now')),
    reviewed_at INT NOT NULL DEFAULT (strftime('%s', 'now')),

    CHECK (name <> '')
);
";

const DECK_ON_UPDATE_TRIGGER: &'static str = "
CREATE TRIGGER IF NOT EXISTS DECK_ON_UPDATE_TRIGGER
AFTER UPDATE OF
    name, description
ON Decks
BEGIN
    UPDATE Decks SET updated_at = strftime('%s', 'now') WHERE deck_id = NEW.deck_id;
END;
";

// description of the closure table from:
// - https://pragprog.com/titles/bksqla/sql-antipatterns
// - http://dirtsimple.org/2010/11/simplest-way-to-do-tree-based-queries.html
//
// allows nested decks
const DECKSCLOSURE: &'static str = "
CREATE TABLE IF NOT EXISTS DecksClosure (
    ancestor INTEGER NOT NULL,
    descendent INTEGER NOT NULL,
    depth INTEGER NOT NULL,
    PRIMARY KEY(ancestor, descendent),
    FOREIGN KEY (ancestor) REFERENCES Decks(deck_id) ON DELETE CASCADE,
    FOREIGN KEY (descendent) REFERENCES Decks(deck_id) ON DELETE CASCADE
);
";

const DECKSCLOSURE_DEPTH_INDEX: &'static str = "
CREATE INDEX IF NOT EXISTS DECKSCLOSURE_DEPTH_INDEX
ON DecksClosure (depth DESC);
";

// any and all node Decks are an/a ancestor/descendent of itself.
const DECKSCLOSURE_NEW_DECK_TRIGGER: &'static str = "
CREATE TRIGGER IF NOT EXISTS DECKSCLOSURE_NEW_DECK_TRIGGER
AFTER INSERT
ON Decks
BEGIN
    INSERT OR IGNORE INTO DecksClosure(ancestor, descendent, depth) VALUES (NEW.deck_id, NEW.deck_id, 0);
END;
";

pub fn create_tables(db_connection: Arc<RwLock<Mutex<Connection>>>) -> Result<(), QueryError> {

    db_write_lock!(db_conn; db_connection);
    let db_conn: &Connection = db_conn;

    // // execute every table setup query
    for query in SETUP.into_iter() {

        let ref query = query.to_string();

        match db_conn.execute_batch(query) {
            Err(why) => {
                let err = QueryError {
                    sqlite_error: why,
                    query: query.clone(),
                };
                return Err(err);
            },
            _ => {/* query sucessfully executed */},
        }
    }

    return Ok(());
}
