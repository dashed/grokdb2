/* 3rd-party imports */

use rusqlite::Error as SqliteError;

/// /////////////////////////////////////////////////////////////////////////////

// TODO: remove all of this
// error struct for api endpoints
// #[derive(Debug, PartialEq, Serialize)]
// pub struct EndPointError {
//     pub status: APIStatus,
//     // pub developerMessage: String,
//     pub userMessage: String,
// }

// impl EndPointError {

//     pub fn bad_request(reason: String) -> Self {
//         EndPointError {
//             status: APIStatus::BadRequest,
//             userMessage: reason
//         }
//     }

//     pub fn server_error(reason: String) -> Self {
//         EndPointError {
//             status: APIStatus::ServerError,
//             userMessage: reason
//         }
//     }

//     pub fn status_code(&self) -> hyper::status::StatusCode {
//         let status_code = match self.status {
//             APIStatus::Ok => hyper::status::StatusCode::Ok,
//             APIStatus::BadRequest => hyper::status::StatusCode::BadRequest,
//             APIStatus::ServerError => hyper::status::StatusCode::InternalServerError,
//         };

//         status_code
//     }
// }

quick_error! {
    #[derive(Debug)]
    pub enum RawAPIError {

        BadInput(api: &'static str, reason: &'static str) {
            display("Bad inputs. API: {} Reason: {}", api, reason)
            description("Bad input to raw API")
        }

        SQLiteError(api: &'static str, sqlite_error: SqliteError) {
            display("SQLiteError. API: {} Reason: {}", api, sqlite_error)
            description("Fatal error in raw API")
        }

        SQLError(sqlite_error: SqliteError, query: String) {
            display("{}\nFor query:\n{}", sqlite_error, query)
            description(sqlite_error.description())
        }
    }
}

// TODO: clean up

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

// pub fn json_err(api_status: APIStatus, dev_message: String, user_message: String) -> EndPointError {
//     return EndPointError {
//         status: APIStatus::BadRequest,
//         developerMessage: reason.clone(),
//         userMessage: reason,
//     };
// }

// pub fn json_deserialize_err(reason: String) -> EndPointError {

//     return EndPointError {
//         status: APIStatus::BadRequest,
//         developerMessage: reason.clone(),
//         userMessage: reason,
//     };
// }

// macro_rules! internal_server_error(
//     () => {{
//         use errors;
//         errors::EndPointError {
//             status: errors::APIStatus::ServerError,
//             developerMessage: "An internal server error occured (status code 500).".to_string(),
//             userMessage: "An internal server error occured (status code 500).".to_string()
//         }
//     }}
// );


// TODO: wtf was this for?
macro_rules! handle_raw_api_error(
    ($reason: expr) => {{
        use errors;
        let _type_check: errors::RawAPIError = $reason;

        // TODO: logging backend
        println!("RAW API ERROR: {:?}", _type_check);
    }}
);

// TODO: wtf was this for?
// TODO: remove
// macro_rules! handle_serde_error(
//     ($reason: expr) => {{
//         use serde_json;
//         let _type_check: serde_json::error::Error = $reason;

// // TODO: logging backend
//         println!("SERDE ERROR: {:?}", _type_check);
//     }}
// );
