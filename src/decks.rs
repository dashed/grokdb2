
/* 3rd-party imports */

use rusqlite::{Connection, Row};
use rusqlite::types::ToSql;
use hyper;

/* local imports */

use contexts::{GlobalContext, Context, APIContext};
use errors::{EndPointError, APIStatus, RawAPIError};
use route::constants::{DeckID};
use types::{Page, PerPage, SortOrder, Count, DecksPageSort};

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Serialize)]
pub struct NewDeckPreRenderState {
    pub POST_TO: String
}

pub struct DecksPageRequest {
    pub page: Page,
    pub per_page: PerPage,
    pub sort: DecksPageSort,

    // this enforces the caller to fetch number of pages
    pub number_of_pages: Count
    // search: Option<String>
}

#[derive(Debug)]
pub struct CreateDeck {
    pub name: String, // required
    pub description: String, // required, but may be empty
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct CreateDeckRequest {
    name: String, // required
    description: String, // required, but may be empty
    // parent: Option<DeckID>,
}

impl CreateDeckRequest {

    // None => is valid
    fn is_invalid(&self, api: &APIContext) -> Option<EndPointError> {

        if self.name.trim().len() <= 0 {
            let err_response = EndPointError {
                status: APIStatus::BadRequest,
                developerMessage: "Deck name must be non-empty.".to_string(),
                userMessage: "Deck name must be non-empty.".to_string()
            };

            return Some(err_response);
        }

        // TODO: remove
        // match self.parent {
        //     None => {},
        //     Some(parent_id) => {

        //         match global_context.deck_exists(parent_id) {
        //             Ok(deck_exists) => {

        //                 if deck_exists {
        //                     return None;
        //                 }

        //                 let err_response = EndPointError {
        //                     status: APIStatus::BadRequest,
        //                     developerMessage: "Parent deck does not exist.".to_string(),
        //                     userMessage: "Parent deck does not exist.".to_string()
        //                 };

        //                 return Some(err_response);
        //             },
        //             Err(why) => {

        //                 handle_raw_api_error!(why);

        //                 let err_response = internal_server_error!();

        //                 return Some(err_response);
        //             }
        //         }
        //     }
        // }

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

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Deck {
    pub id: DeckID,
    pub name: String,
    pub description: String,
    pub created_at: i64, // unix timestamp
    pub updated_at: i64,  // unix timestamp
    pub reviewed_at: i64,  // unix timestamp
    pub has_reviewed: bool // false if reviewed_at == created_at, otherwise true
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
    use errors::{json_deserialize_err, EndPointError, APIStatus};
    use types::{DecksPageQuery, Search};

    ////////////////////////////////////////////////////////////////////////////

    // TODO: POST /api/deck (this is very raw); adapt from create_deck(...)

    // POST /api/deck/:deck_id
    pub fn create_deck(mut context: Context, request: Request, response: Response) {

        let parent_deck_id = parse_capture!(&context.captures, "deck_id", DeckID);

        let request: CreateDeckRequest = match serde_json::from_reader(request) {
            Ok(request) => request,
            Err(err) => {
                let payload = json_deserialize_err(format!("Malformed request. Unable to create deck."));
                respond_json!(response; payload);
                return;
            }
        };

        match request.is_invalid(&context.api) {
            None => {},
            Some(reason) => {
                respond_json!(response; reason);
                return;
            }
        }

        match context.api.deck_exists(parent_deck_id) {
            Ok(deck_exists) => {

                if !deck_exists {
                    let err_response = EndPointError {
                        status: APIStatus::BadRequest,
                        developerMessage: "Parent deck does not exist.".to_string(),
                        userMessage: "Parent deck does not exist.".to_string()
                    };

                    respond_json!(response; err_response);
                    return;
                }

            },
            Err(why) => {

                handle_raw_api_error!(why);

                let err_response = internal_server_error!();

                respond_json!(response; err_response);
                return;
            }
        }

        let create_deck = CreateDeck {
            name: request.name.trim().to_string(),
            description: request.description.trim().to_string()
        };

        let new_deck: Deck = match context.api.create_deck(create_deck) {
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

        let child_id = new_deck.id;

        match context.api.connect_decks(child_id, parent_deck_id) {
            Ok(_) => {},
            Err(why) => {

                handle_raw_api_error!(why);

                let reason = internal_server_error!();
                respond_json!(response; reason);
                return;
            }
        }

        let json_response = DeckResponse {

            profile_url: view_route_to_link(
                AppRoute::Deck(new_deck.id, DeckRoute::Decks(DecksPageQuery::NoQuery, Search::NoQuery)),
                &context
            ),
            deck: new_deck,

            has_parent: true,
            parent_id: Some(parent_deck_id)
        };

        respond_json!(response; json_response);

    }
}

// decks api
impl<'a> APIContext<'a> {

    pub fn deck_exists(&self, deck_id: DeckID) -> Result<bool, RawAPIError> {

        if self.should_cache {
            if self.cache.decks.contains_key(&deck_id) {
                return Ok(true);
            }
        }

        let query = r"
            SELECT
                COUNT(1)
            FROM Decks
            WHERE deck_id = :deck_id
            LIMIT 1;
        ";

        let params: &[(&str, &ToSql)] = &[
            (":deck_id", &deck_id)
        ];

        db_read_lock!(db_conn; self.global_context.db_connection);
        let db_conn: &Connection = db_conn;

        let results = db_conn.query_row_named(query, params, |row| -> bool {
            let count: i64 = row.get(0);
            return count >= 1;
        });

        match results {
            Err(sqlite_error) => {
                return Err(RawAPIError::SQLError(sqlite_error, query.to_string()));
            },
            Ok(deck_exists) => {
                return Ok(deck_exists);
            }
        };
    }

    pub fn get_deck(&mut self, deck_id: DeckID) -> Result<Deck, RawAPIError> {

        if self.should_cache {
            if self.cache.decks.contains_key(&deck_id) {
                let deck = self.cache.decks.get(&deck_id).unwrap();
                return Ok(deck.clone());
            }
        }

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

        db_read_lock!(db_conn; self.global_context.db_connection);
        let db_conn: &Connection = db_conn;

        let results = db_conn.query_row(&query, &[], |row| -> Deck {

            let created_at: i64 = row.get(3);
            let reviewed_at: i64 = row.get(5);

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

                if self.should_cache {
                    self.cache.decks.insert(deck.id, deck.clone());
                }
                return Ok(deck);
            }
        };
    }

    pub fn create_deck(&mut self, create_deck_request: CreateDeck) -> Result<Deck, RawAPIError> {

        let query = "INSERT INTO Decks(name, description) VALUES (:name, :description);";

        let params: &[(&str, &ToSql)] = &[
            (":name", &create_deck_request.name.clone()),
            (":description", &create_deck_request.description.clone())
        ];

        let deck_id: DeckID = {
            db_write_lock!(db_conn; self.global_context.db_connection);
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

        return self.get_deck(deck_id);

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


        db_write_lock!(db_conn; self.global_context.db_connection);
        let db_conn: &Connection = db_conn;

        match db_conn.execute_named(query_delete, &params[..]) {
            Err(sqlite_error) => {
                return Err(RawAPIError::SQLError(sqlite_error, query_delete.to_string()));
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
                return Err(RawAPIError::SQLError(sqlite_error, query_insert.to_string()));
            },
            _ => {/* query sucessfully executed */},
        }

        return Ok(());
    }

    // pub fn count_children_of_deck(&self, deck_id: DeckID) -> Result<u64, RawAPIError> {

    //     let query = "
    //         SELECT
    //             Count(descendent)
    //         FROM
    //             DecksClosure
    //         INNER JOIN
    //             Decks
    //         ON DecksClosure.descendent = Decks.deck_id
    //         WHERE
    //             ancestor = :deck_id
    //         AND
    //             depth = 1
    //         ORDER BY
    //             Decks.name
    //         COLLATE NOCASE ASC;
    //     ";
    // }

    pub fn children_of_deck_count(&self, deck_id: DeckID) -> Result<Count, RawAPIError> {

        // TODO: caching

        let query = format!(r"
            SELECT
                COUNT(descendent)
            FROM
                DecksClosure
            INNER JOIN
                Decks
            ON DecksClosure.descendent = Decks.deck_id
            WHERE
                ancestor = {deck_id}
            AND
                depth = 1;
        ", deck_id = deck_id);

        db_read_lock!(db_conn; self.global_context.db_connection);
        let db_conn: &Connection = db_conn;

        let results = db_conn.query_row(&query, &[], |row| -> Count {
            let count: i64 = row.get(0);
            return count as Count;
        });

        match results {
            Err(sqlite_error) => {
                return Err(RawAPIError::SQLError(sqlite_error, query));
            },
            Ok(count) => {
                return Ok(count);
            }
        };

    }

    pub fn children_of_deck(&mut self, deck_id: DeckID, filter: DecksPageRequest) -> Result<Vec<Deck>, RawAPIError> {

        if filter.per_page <= 0 {
            return Err(RawAPIError::BadInput("decks::children_of_deck", "filter.per_page must be at least 1"));
        }

        if filter.page <= 0 {
            return Err(RawAPIError::BadInput("decks::children_of_deck", "filter.page must be at least 1"));
        }

        if filter.page > filter.number_of_pages {
            return Err(RawAPIError::BadInput("decks::children_of_deck", "page out of bounds"));
        }

        let records_to_skip = (filter.page - 1) * filter.per_page;

        // src: http://blog.ssokolow.com/archives/2009/12/23/sql-pagination-without-offset/

        let records_skip_query = format!(r"
            SELECT
                descendent
            FROM
                DecksClosure
            INNER JOIN
                Decks
            ON DecksClosure.descendent = Decks.deck_id
            WHERE
                ancestor = {deck_id}
            AND
                depth = 1
            ORDER BY
                Decks.name
            COLLATE NOCASE ASC
            LIMIT {records_to_skip}
        ", deck_id = deck_id, records_to_skip = records_to_skip);

        let query = format!(r"
            SELECT
                Decks.deck_id,
                Decks.name,
                Decks.description,
                Decks.created_at,
                Decks.updated_at,
                Decks.reviewed_at
            FROM
                DecksClosure
            INNER JOIN
                Decks
            ON DecksClosure.descendent = Decks.deck_id
            WHERE
                Decks.oid NOT IN ({records_skip_query})
            AND
                ancestor = {deck_id}
            AND
                depth = 1
            ORDER BY
                Decks.name
            COLLATE NOCASE ASC
            LIMIT {per_page};
        ",
            records_skip_query = records_skip_query,
            deck_id = deck_id,
            per_page = filter.per_page
        );

        db_read_lock!(db_conn; self.global_context.db_connection);
        let db_conn: &Connection = db_conn;

        let mut statement = match db_conn.prepare(&query) {
            Err(sqlite_error) => {
                return Err(RawAPIError::SQLError(sqlite_error, query));
            },
            Ok(statement) => statement
        };

        let maybe_iter = statement.query_map(&[], |row: &Row| -> Deck {

            let created_at: i64 = row.get(3);
            let reviewed_at: i64 = row.get(5);

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

        match maybe_iter {
            Err(sqlite_error) => {
                return Err(RawAPIError::SQLError(sqlite_error, query));
            },
            Ok(iter) => {

                let mut vec_of_decks: Vec<Deck> = Vec::new();

                for maybe_deck in iter {

                    let item = match maybe_deck {
                        Err(sqlite_error) => {
                            return Err(RawAPIError::SQLError(sqlite_error, query));
                        },
                        Ok(item) => item
                    };

                    if self.should_cache {
                        self.cache.decks.insert(item.id, item.clone());
                    }

                    vec_of_decks.push(item);
                }

                return Ok(vec_of_decks);
            }
        };

    }

    pub fn children_of_deck_old(&self, deck_id: DeckID) -> Result<Vec<DeckID>, RawAPIError> {

        let query = "
            SELECT
                descendent
            FROM
                DecksClosure
            INNER JOIN
                Decks
            ON DecksClosure.descendent = Decks.deck_id
            WHERE
                ancestor = :deck_id
            AND
                depth = 1
            ORDER BY
                Decks.name
            COLLATE NOCASE ASC;
        ";
        // TODO: COLLATE NOCASE ASC necessary?

        let params: &[(&str, &ToSql)] = &[
            (":deck_id", &deck_id),
        ];

        db_read_lock!(db_conn; self.global_context.db_connection);
        let db_conn: &Connection = db_conn;

        let mut statement = match db_conn.prepare(query) {
            Err(sqlite_error) => {
                return Err(RawAPIError::SQLError(sqlite_error, query.to_string()));
            },
            Ok(statement) => statement
        };

        let maybe_iter = statement.query_map_named(params, |row: &Row| -> DeckID {
            return row.get(0);
        });

        match maybe_iter {
            Err(sqlite_error) => {
                return Err(RawAPIError::SQLError(sqlite_error, query.to_string()));
            },
            Ok(iter) => {

                let mut vec_of_deck_id: Vec<DeckID> = Vec::new();

                for maybe_deck_id in iter {

                    let deck_id: DeckID = match maybe_deck_id {
                        Err(sqlite_error) => {
                            return Err(RawAPIError::SQLError(sqlite_error, query.to_string()));
                        },
                        Ok(deck_id) => deck_id
                    };

                    vec_of_deck_id.push(deck_id);
                }

                return Ok(vec_of_deck_id);
            }
        };

    }
}
