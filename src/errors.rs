/* 3rd-party imports */

use hyper;
use serde::{Serialize, Serializer};
use rusqlite::{Error as SqliteError};

////////////////////////////////////////////////////////////////////////////////

// src: https://github.com/WhiteHouse/api-standards#error-handling

// 200 - OK
// 400 - Bad Request
// 500 - Internal Server Error

#[derive(Debug, PartialEq)]
pub enum APIStatus {

    // #[serde(rename = "Reques success")]
    Ok, // 200

    // #[serde(rename = "Failure due to client-side problem")]
    BadRequest, // 400

    // #[serde(rename = "Failure due to server-side problem")]
    ServerError // 500
}

// NOTE: #[derive(Serialize)] is not used. Need custom serialization.
impl Serialize for APIStatus {

    // serialize APIStatus to human friendly message
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer {

        let human_message = match self {
            &APIStatus::Ok => "Request success.",
            &APIStatus::BadRequest => "Failure due to client-side problem.",
            &APIStatus::ServerError => "Failure due to server-side problem.",
        };

        return serializer.serialize_str(human_message);
    }
}

// error struct for api endpoints
#[derive(Debug, PartialEq, Serialize)]
pub struct EndPointError {
    pub status: APIStatus,
    pub developerMessage: String,
    pub userMessage: String,
}

impl EndPointError {
    pub fn status_code(&self) -> hyper::status::StatusCode {
        let status_code = match self.status {
            APIStatus::Ok => {
                hyper::status::StatusCode::Ok
            },
            APIStatus::BadRequest => {
                hyper::status::StatusCode::BadRequest
            },
            APIStatus::ServerError => {
                hyper::status::StatusCode::InternalServerError
            }
        };

        status_code
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum RawAPIError {

        BadInput(api: &'static str, reason: &'static str) {
            display("Bad inputs. API: {} Reason: {}", api, reason)
            description("Bad input to raw api")
        }

        SQLError(sqlite_error: SqliteError, query: &'static str) {
            display("{}\nFor query:\n{}", sqlite_error, query)
            description(sqlite_error.description())
        }
    }
}

// #[derive(Debug)]
// pub struct QueryError {
//     pub sqlite_error: SqliteError,
//     pub query: String,
// }

// impl fmt::Display for QueryError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{} \nFor query:\n{}", self.sqlite_error, self.query)
//     }
// }

// impl error::Error for QueryError {
//     fn description(&self) -> &str {
//         return self.sqlite_error.description();
//     }
// }


pub fn json_deserialize_err(reason: String) -> EndPointError {

    return EndPointError {
        status: APIStatus::BadRequest,
        developerMessage: reason.clone(),
        userMessage: reason
    }
}

macro_rules! internal_server_error(
    () => {{
        use errors;
        errors::EndPointError {
            status: errors::APIStatus::ServerError,
            developerMessage: "Internal server error.".to_string(),
            userMessage: "Internal server error.".to_string()
        }
    }}
);


macro_rules! handle_raw_api_error(
    ($reason: expr) => {{
        use errors;
        let _type_check: errors::RawAPIError = $reason;

        // TODO: logging backend
        println!("RAW API ERROR: {:?}", _type_check);
    }}
);
