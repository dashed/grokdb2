/* 3rd-party imports */

use rusqlite::Connection;
use rusqlite::types::ToSql;

/* local imports */

use context::Context;
use types::{UnixTimestamp, DeckID};
use errors::RawAPIError;

/// /////////////////////////////////////////////////////////////////////////////
// TODO: fix
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Deck {
    pub id: DeckID,
    pub name: String,
    pub description: String,
    pub created_at: UnixTimestamp, // unix timestamp
    pub updated_at: UnixTimestamp, // unix timestamp
    pub reviewed_at: UnixTimestamp, // unix timestamp
    pub has_reviewed: bool, // false if reviewed_at == created_at, otherwise true
}

// struct for requesting to create a deck
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateDeck {
    pub name: String, // required
    pub description: String, // required, but may be empty
}

pub fn get_deck(context: Context, deck_id: DeckID) -> Result<Deck, RawAPIError> {

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

    db_read_lock!(db_conn; context.database);
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
            has_reviewed: created_at != reviewed_at,
        };
    });

    match results {
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        }
        Ok(deck) => {
            return Ok(deck);
        }
    };
}

pub fn deck_exists(context: Context, deck_id: DeckID) -> Result<bool, RawAPIError> {

    let query = format!("
        SELECT
            COUNT(1)
        FROM Decks
        WHERE deck_id = {deck_id}
        LIMIT 1;
    ", deck_id = deck_id);

    db_read_lock!(db_conn; context.database);
    let db_conn: &Connection = db_conn;

    let deck_exists = db_conn.query_row(&query, &[], |row| -> bool {
        let count: i64 = row.get(0);
        return count >= 1;
    });

    match deck_exists {
        Ok(deck_exists) => return Ok(deck_exists),
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        }
    }
}

pub fn create_deck(context: Context, create_deck_request: CreateDeck) -> Result<Deck, RawAPIError> {

    let query = "INSERT INTO Decks(name, description) VALUES (:name, :description);";

    let params: &[(&str, &ToSql)] = &[(":name", &create_deck_request.name.clone()),
                                      (":description", &create_deck_request.description.clone())];

    let deck_id: DeckID = {
        db_write_lock!(db_conn; context.database);
        let db_conn: &Connection = db_conn;

        match db_conn.execute_named(query, &params[..]) {
            Err(sqlite_error) => {
                return Err(RawAPIError::SQLError(sqlite_error, query.to_string()));
            }
            _ => {
                /* query sucessfully executed */
            }
        }

        let row_id = db_conn.last_insert_rowid();
        row_id
    };

    return get_deck(context, deck_id);
}

#[test]
fn decks_test() {

    /* imports */

    use std::fs;
    use database;
    use api::decks;

    /* setup */

    let file_path = "test/assets/decks_test.db".to_string();

    let db_connection = database::get_database(file_path.clone());

    // deck doesn't exist

    {
        match decks::get_deck(Context::new(db_connection.clone()), 1) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    };

    // create deck

    {
        // case: don't allow empty deck name

        let request = CreateDeck {
            name: format!(""),
            description: format!(""),
        };

        match decks::create_deck(Context::new(db_connection.clone()), request) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    };

    {
        // case: add new deck

        let request = CreateDeck {
            name: format!("Foo"),
            description: format!(""),
        };

        match decks::create_deck(Context::new(db_connection.clone()), request) {
            Ok(actual) => {
                assert_eq!(actual.id, 1);
                assert_eq!(actual.name, format!("Foo"));
                assert_eq!(actual.description, format!(""));
                assert_eq!(actual.created_at, actual.updated_at);
                assert_eq!(actual.created_at, actual.reviewed_at);
                assert_eq!(actual.has_reviewed, false);
            },
            Err(_) => assert!(false),
        }

        let request = CreateDeck {
            name: format!("Bar"),
            description: format!("Amazing description of this deck."),
        };

        match decks::create_deck(Context::new(db_connection.clone()), request) {
            Ok(actual) => {
                assert_eq!(actual.id, 2); // ensure increment
                assert_eq!(actual.name, format!("Bar"));
                assert_eq!(actual.description, format!("Amazing description of this deck."));
                assert_eq!(actual.created_at, actual.updated_at);
                assert_eq!(actual.created_at, actual.reviewed_at);
                assert_eq!(actual.has_reviewed, false);
            },
            Err(_) => assert!(false),
        }
    };

    {
        // case: retrieve created decks

        match decks::get_deck(Context::new(db_connection.clone()), 2) {
            Ok(actual) => {
                assert_eq!(actual.id, 2);
                assert_eq!(actual.name, format!("Bar"));
                assert_eq!(actual.description, format!("Amazing description of this deck."));
                assert_eq!(actual.created_at, actual.updated_at);
                assert_eq!(actual.created_at, actual.reviewed_at);
                assert_eq!(actual.has_reviewed, false);
            },
            Err(_) => assert!(false),
        }

        match decks::get_deck(Context::new(db_connection.clone()), 1) {
            Ok(actual) => {
                assert_eq!(actual.id, 1);
                assert_eq!(actual.name, format!("Foo"));
                assert_eq!(actual.description, format!(""));
                assert_eq!(actual.created_at, actual.updated_at);
                assert_eq!(actual.created_at, actual.reviewed_at);
                assert_eq!(actual.has_reviewed, false);
            },
            Err(_) => assert!(false),
        }
    };

    // deck exists

    {
        // case: doesn't exist

        match decks::deck_exists(Context::new(db_connection.clone()), 3) {
            Ok(actual) => {
                assert_eq!(actual, false);
            }
            Err(_) => assert!(false),
        }

        // case: exists

        match decks::deck_exists(Context::new(db_connection.clone()), 1) {
            Ok(actual) => {
                assert_eq!(actual, true);
            }
            Err(_) => assert!(false),
        }

        match decks::deck_exists(Context::new(db_connection.clone()), 2) {
            Ok(actual) => {
                assert_eq!(actual, true);
            }
            Err(_) => assert!(false),
        }
    };

    /* teardown */

    fs::remove_file(file_path.clone()).unwrap();
}
