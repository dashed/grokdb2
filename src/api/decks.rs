/* 3rd-party imports */

use rusqlite::Connection;
use rusqlite::types::ToSql;

/* local imports */

use types::{Database, UnixTimestamp, DeckID};
use errors::RawAPIError;

/// /////////////////////////////////////////////////////////////////////////////

// TODO: fix
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Deck {
    pub id: DeckID,
    pub name: String,
    pub description: String,
    pub created_at: UnixTimestamp, // unix timestamp
    pub updated_at: UnixTimestamp,  // unix timestamp
    pub reviewed_at: UnixTimestamp,  // unix timestamp
    pub has_reviewed: bool // false if reviewed_at == created_at, otherwise true
}

// struct for requesting to create a deck
#[derive(Debug)]
pub struct CreateDeck {
    pub name: String, // required
    pub description: String, // required, but may be empty
}

pub fn get_deck(database: Database, deck_id: DeckID) -> Result<Deck, RawAPIError> {

    let query = format!("
        SELECT
            deck_id,
            name,
            description,
            created_at,
            updated_at,
            reviewed_at
        FROM Decks
        WHERE deck_id = {deck_id}
        LIMIT 1;
    ", deck_id = deck_id);

    db_read_lock!(db_conn; database);
    let db_conn: &Connection = db_conn;

    let results = db_conn.query_row(&query, &[], |row| -> Deck {

        let created_at: UnixTimestamp = row.get(3);
        let reviewed_at: UnixTimestamp = row.get(5);

        return Deck {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
            created_at: created_at,
            updated_at: row.get(4),
            reviewed_at: reviewed_at,
            has_reviewed: created_at != reviewed_at
        };
    });

    match results {
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        },
        Ok(deck) => {
            return Ok(deck);
        }
    };
}

pub fn create_deck(database: Database, create_deck_request:  CreateDeck) -> Result<Deck, RawAPIError> {

    let query = "INSERT INTO Decks(name, description) VALUES (:name, :description);";

    let params: &[(&str, &ToSql)] = &[
        (":name", &create_deck_request.name.clone()),
        (":description", &create_deck_request.description.clone())
    ];

    let deck_id: DeckID = {
        db_write_lock!(db_conn; database);
        let db_conn: &Connection = db_conn;

        match db_conn.execute_named(query, &params[..]) {
            Err(sqlite_error) => {
                return Err(RawAPIError::SQLError(sqlite_error, query.to_string()));
            },
            _ => {/* query sucessfully executed */},
        }

        let row_id = db_conn.last_insert_rowid();
        row_id
    };

    return get_deck(database, deck_id);
}
