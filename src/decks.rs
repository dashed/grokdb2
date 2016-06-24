
/* 3rd-party imports */

use rusqlite::{Connection};

/* local imports */

use contexts::{GlobalContext};

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Deserialize)]
pub struct CreateDeckRequest {
    name: String, // required
    description: String, // required, but may be empty
    parent: Option<u64>,
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

// deck api
impl<'a> GlobalContext<'a> {

    // POST /api/deck
    pub fn create_deck(&self, create_deck_request: CreateDeckRequest) {

        db_write_lock!(db_conn; self.db_connection);
        let db_conn: &Connection = db_conn;
    }
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

        let data: CreateDeckRequest = match serde_json::from_reader(request) {
            Ok(request) => request,
            Err(err) => {
                let payload = json_deserialize_err(format!("Malformed request. Unable to create deck."));

                // TODO: macro
                let mut stream = response.start().unwrap();
                serde_json::to_writer(&mut stream, &payload);

                return;
            }
        };

        println!("data: {:?}", data);
    }
}
