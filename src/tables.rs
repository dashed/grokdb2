/* rust lib imports */

use std::sync::{Arc, Mutex, RwLock};

/* 3rd-party imports */

use rusqlite::{Connection};

/* local imports */

use errors::{RawAPIError};

////////////////////////////////////////////////////////////////////////////////

const SETUP: [&'static str; 7] = [

    // configs

    CONFIGS,

    // configs/triggers

    CONFIG_ON_UPDATE_TRIGGER,

    // decks

    DECKS,
    DECKSCLOSURE,

    // decks/indices

    DECKSCLOSURE_DEPTH_INDEX,

    // decks/triggers

    DECK_ON_UPDATE_TRIGGER,
    DECKSCLOSURE_NEW_DECK_TRIGGER,
];

/**
 * All SQL comply with syntax supported with SQLite v3.9.1
 */

/* configs */

// note: CHECK (setting <> '') ensures setting is non-empty string
const CONFIGS: &'static str = "
CREATE TABLE IF NOT EXISTS Configs (
    setting TEXT PRIMARY KEY NOT NULL,
    value TEXT,

    created_at INT NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INT NOT NULL DEFAULT (strftime('%s', 'now')),

    CHECK (setting <> '')
);
";

const CONFIG_ON_UPDATE_TRIGGER: &'static str = "
CREATE TRIGGER IF NOT EXISTS CONFIG_ON_UPDATE_TRIGGER
AFTER UPDATE OF
    setting, value
ON Configs
BEGIN
    UPDATE Configs SET updated_at = strftime('%s', 'now') WHERE setting = NEW.setting;
END;
";


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

pub fn setup_database(db_connection: Arc<RwLock<Mutex<Connection>>>) -> Result<(), RawAPIError> {

    db_write_lock!(db_conn; db_connection);
    let db_conn: &Connection = db_conn;

    // // execute every table setup query
    for query in SETUP.into_iter() {

        match db_conn.execute_batch(query) {
            Err(sqlite_error) => {
                return Err(RawAPIError::SQLError(sqlite_error, query.to_string()));
            },
            _ => {/* query sucessfully executed */},
        }
    }

    return Ok(());
}
