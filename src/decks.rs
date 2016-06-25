
/* 3rd-party imports */

use rusqlite::{Connection};

/* local imports */

use contexts::{GlobalContext};
use errors::{EndPointError, APIStatus};

////////////////////////////////////////////////////////////////////////////////

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
    id: u64,
    name: String,
    description: String
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


    pub fn create_deck(&self, create_deck_request: CreateDeckRequest) {

        db_write_lock!(db_conn; self.db_connection);
        let db_conn: &Connection = db_conn;
    }
}
