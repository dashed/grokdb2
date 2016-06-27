/* 3rd-party imports */

use hyper;

////////////////////////////////////////////////////////////////////////////////

macro_rules! default {
    () => (
        Default::default()
    )
}

// alias wrapper
macro_rules! classnames {
    ($($tail:tt)+) => {{
        let classes = labels!($($tail)*).into_string().unwrap();
        if classes.len() > 0 {
            Some(classes)
        } else {
            None
        }
    }};
}

// alias wrapper
macro_rules! stylenames {
    ($($tail:tt)+) => {{
        let styles = labels_sep_by!(""; $($tail)*).into_string().unwrap();
        if styles.len() > 0 {
            Some(styles)
        } else {
            None
        }
    }};
}

// TODO: write docs
// macro to reduce boilerplate code
macro_rules! respond_json {
    ($response:expr; $payload:expr) => (
        let mut response = $response;

        use hyper;

        // any and all json serializables must implement this
        *response.status_mut() = $payload.status_code();

        response.headers_mut().set((hyper::header::ContentType(
            mime!(Application/Json)
        )));


        let json_response = match serde_json::to_string(&$payload) {
            Ok(json_response) => json_response,
            Err(why) => {

                handle_serde_error!(why);

                let message = "Internal server error";

                // 500 status code
                *response.status_mut() = hyper::status::StatusCode::InternalServerError;

                response.send(message.as_bytes()).unwrap();
                return;
            }
        };

        response.send(json_response.as_bytes()).unwrap();

        // TODO: don't stream... first convert to string; capture any panics
        // let mut stream = response.start().unwrap();
        // serde_json::to_writer(&mut stream, &$payload);
    );
}
