// src: https://github.com/WhiteHouse/api-standards#error-handling

// 200 - OK
// 400 - Bad Request
// 500 - Internal Server Error

#[derive(Debug, PartialEq, Serialize)]
pub enum APIStatus {

    #[serde(rename = "Reques success")]
    Ok, // 200

    #[serde(rename = "Failure due to client-side problem")]
    BadRequest, // 400

    #[serde(rename = "Failure due to server-side problem")]
    ServerError // 500
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
