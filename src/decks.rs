
/* 3rd-party imports */

use rusqlite::{Connection};
use rusqlite::types::ToSql;
use hyper;

/* local imports */

use contexts::{GlobalContext};
use errors::{EndPointError, APIStatus, RawAPIError};
use route::constants::{DeckID};

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct CreateDeck {
    pub name: String, // required
    pub description: String, // required, but may be empty
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct CreateDeckRequest {
    name: String, // required
    description: String, // required, but may be empty
    parent: Option<DeckID>,
}

impl CreateDeckRequest {

    // None => is valid
    fn is_invalid(&self, global_context: &GlobalContext) -> Option<EndPointError> {

        if self.name.trim().len() <= 0 {
            let err_response = EndPointError {
                status: APIStatus::BadRequest,
                developerMessage: "Deck name must be non-empty.".to_string(),
                userMessage: "Deck name must be non-empty.".to_string()
            };

            return Some(err_response);
        }

        match self.parent {
            None => {},
            Some(parent_id) => {

                match global_context.deck_exists(parent_id) {
                    Ok(deck_exists) => {

                        if deck_exists {
                            return None;
                        }

                        let err_response = EndPointError {
                            status: APIStatus::BadRequest,
                            developerMessage: "Parent deck does not exist.".to_string(),
                            userMessage: "Parent deck does not exist.".to_string()
                        };

                        return Some(err_response);
                    },
                    Err(why) => {

                        handle_raw_api_error!(why);

                        let err_response = internal_server_error!();

                        return Some(err_response);
                    }
                }
            }
        }

        return None;
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct DeckResponse {

    profile_url: String,

    // the resource
    deck: Deck,

    has_parent: bool,
    parent_id: Option<DeckID>
}

impl DeckResponse {
    pub fn status_code(&self) -> hyper::status::StatusCode {
        hyper::status::StatusCode::Ok
    }
}

// #[derive(Debug, PartialEq, Serialize)]
// pub struct CreateDeckResponse {

//     // url for client side to redirect to
//     redirect: String,

//     // the created resource
//     deck: Deck
// }

#[derive(Debug, PartialEq, Serialize)]
pub struct Deck {
    pub id: i64,
    pub name: String,
    pub description: String
}

pub mod routes {

    /* 3rd-party imports */

    use hyper::server::{Request, Response};
    use serde_json;
    use serde_json::Value;

    /* local imports */

    use contexts::Context;
    use super::{CreateDeckRequest, CreateDeck, Deck, DeckResponse};
    use route::constants::{DeckID, AppRoute, DeckRoute};
    use route::helpers::{view_route_to_link};
    use errors::{json_deserialize_err};

    ////////////////////////////////////////////////////////////////////////////

    // POST /api/deck
    pub fn create_deck(mut context: Context, request: Request, response: Response) {

        let request: CreateDeckRequest = match serde_json::from_reader(request) {
            Ok(request) => request,
            Err(err) => {
                let payload = json_deserialize_err(format!("Malformed request. Unable to create deck."));
                respond_json!(response; payload);
                return;
            }
        };

        match request.is_invalid(&context.global_context) {
            None => {},
            Some(reason) => {
                respond_json!(response; reason);
                return;
            }
        }

        let create_deck = CreateDeck {
            name: request.name.trim().to_string(),
            description: request.description.trim().to_string()
        };

        let new_deck: Deck = match context.global_context.create_deck(create_deck) {
            Ok(new_deck) => {
                new_deck
            },
            Err(why) => {

                handle_raw_api_error!(why);

                let reason = internal_server_error!();
                respond_json!(response; reason);
                return;
            }
        };

        let (has_parent, parent_id): (bool, Option<DeckID>) = match request.parent {
            None => {
                (false, None)
            },
            Some(parent_id) => {

                let child_id = new_deck.id;

                match context.global_context.connect_decks(child_id, parent_id) {
                    Ok(_) => {
                        (true, Some(parent_id))
                    },
                    Err(why) => {

                        handle_raw_api_error!(why);

                        let reason = internal_server_error!();
                        respond_json!(response; reason);
                        return;
                    }
                }

            }
        };

        let json_response = DeckResponse {

            profile_url: view_route_to_link(AppRoute::Deck(new_deck.id, DeckRoute::Description), &context),
            deck: new_deck,

            has_parent: has_parent,
            parent_id: parent_id
        };

        respond_json!(response; json_response);

        // println!("data: {:?}", request);
    }
}

// decks api
impl<'a> GlobalContext<'a> {

    pub fn deck_exists(&self, deck_id: DeckID) -> Result<bool, RawAPIError> {

        let query = "SELECT COUNT(1) FROM Decks WHERE deck_id = :deck_id LIMIT 1;";

        let params: &[(&str, &ToSql)] = &[
            (":deck_id", &deck_id)
        ];

        db_write_lock!(db_conn; self.db_connection);
        let db_conn: &Connection = db_conn;

        let results = db_conn.query_row_named(query, params, |row| -> bool {
            let count: i64 = row.get(0);
            return count >= 1;
        });

        match results {
            Err(sqlite_error) => {
                return Err(RawAPIError::SQLError(sqlite_error, query));
            },
            Ok(deck_exists) => {
                return Ok(deck_exists);
            }
        };
    }

    pub fn create_deck(&self, create_deck_request: CreateDeck) -> Result<Deck, RawAPIError> {

        let query = "INSERT INTO Decks(name, description) VALUES (:name, :description);";

        let params: &[(&str, &ToSql)] = &[
            (":name", &create_deck_request.name.clone()),
            (":description", &create_deck_request.description.clone())
        ];

        db_write_lock!(db_conn; self.db_connection);
        let db_conn: &Connection = db_conn;

        match db_conn.execute_named(query, &params[..]) {
            Err(sqlite_error) => {
                return Err(RawAPIError::SQLError(sqlite_error, query));
            },
            _ => {/* query sucessfully executed */},
        }

        let row_id = db_conn.last_insert_rowid();

        let created_deck = Deck {
            id: row_id,
            name: create_deck_request.name,
            description: create_deck_request.description
        };

        return Ok(created_deck);
    }

    pub fn connect_decks(&self, child: DeckID, parent: DeckID) -> Result<(), RawAPIError> {

        // moving a child deck subtree consists of two procedures:
        // 1. delete any and all subtree connections between child (and its descendants)
        //    and the child's ancestors
        let query_delete = "
            DELETE FROM DecksClosure

            /* select all descendents of child */
            WHERE descendent IN (
                SELECT descendent
                FROM DecksClosure
                WHERE ancestor = :child
            )
            AND

            /* select all ancestors of child but not child itself */
            ancestor IN (
                SELECT ancestor
                FROM DecksClosure
                WHERE descendent = :child
                AND ancestor != descendent
            )
            AND descendent != ancestor;
        ";

        let params: &[(&str, &ToSql)] = &[
            (":child", &child)
        ];


        db_write_lock!(db_conn; self.db_connection);
        let db_conn: &Connection = db_conn;

        match db_conn.execute_named(query_delete, &params[..]) {
            Err(sqlite_error) => {
                return Err(RawAPIError::SQLError(sqlite_error, query_delete));
            },
            _ => {/* query sucessfully executed */},
        }


        // 2. make parent (and its ancestors) be ancestors of child deck (and its descendants)
        let query_insert = "
            INSERT OR IGNORE INTO DecksClosure(ancestor, descendent, depth)
            SELECT p.ancestor, c.descendent, p.depth+c.depth+1
                FROM DecksClosure AS p, DecksClosure AS c
            WHERE
                c.ancestor = :child
                AND p.descendent = :parent;
        ";

        let params: &[(&str, &ToSql)] = &[
            (":child", &child),
            (":parent", &parent)
        ];

        match db_conn.execute_named(query_insert, &params[..]) {
            Err(sqlite_error) => {
                return Err(RawAPIError::SQLError(sqlite_error, query_insert));
            },
            _ => {/* query sucessfully executed */},
        }

        return Ok(());
    }
}
