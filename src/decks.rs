
/* 3rd-party imports */

use rusqlite::{Connection};
use rusqlite::types::ToSql;

/* local imports */

use contexts::{GlobalContext};
use errors::{EndPointError, APIStatus, RawAPIError};

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
    parent: Option<u64>,
}

impl CreateDeckRequest {
    fn is_invalid(&self, global_context: &GlobalContext) -> Option<EndPointError> {

        if self.name.trim().len() <= 0 {
            let response = EndPointError {
                status: APIStatus::BadRequest,
                developerMessage: "Deck name must be non-empty.".to_string(),
                userMessage: "Deck name must be non-empty.".to_string()
            };

            return Some(response);
        }

        match self.parent {
            None => {},
            Some(parent_id) => {
                // TODO: check if parent exists
            }
        }

        return None;
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct CreateDeckResponse {

    // url for client side to redirect to
    redirect: String,

    // the created resource
    deck: Deck
}

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
    use super::CreateDeckRequest;
    use errors::{json_deserialize_err};

    ////////////////////////////////////////////////////////////////////////////

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



        println!("data: {:?}", request);
    }
}

// decks api
impl<'a> GlobalContext<'a> {


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
}
