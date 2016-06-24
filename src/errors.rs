/* 3rd-party imports */

use serde::{Serialize, Serializer};

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

#[derive(Debug, PartialEq, Serialize)]
pub struct EndPointError {
    status: APIStatus,
    developerMessage: String,
    userMessage: String,
}

pub fn json_deserialize_err(reason: String) -> EndPointError {

    return EndPointError {
        status: APIStatus::BadRequest,
        developerMessage: reason.clone(),
        userMessage: reason
    }
}
