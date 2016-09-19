/* ////////////////////////////////////////////////////////////////////////// */

// used for handling API for json
macro_rules! handle_api_result_json {
    ($api:expr) => {{
        match $api {
            Ok(result) => result,
            Err(_err) => {

                // TODO: for translation, pass context to handle_api_result! macro to get
                //       client language preference

                // type enforcement
                use errors::RawAPIError;
                let _err: RawAPIError = _err;

                println!("{:?}", _err);

                // TODO: internal error logging
                return respond_json_with_error!(APIStatus::ServerError;
                    "An internal server error occurred.".to_string(); None);

            }
        }
    }}
}

macro_rules! handle_api_result_html {
    ($api:expr) => {{
        match $api {
            Ok(result) => result,
            Err(_err) => {

                // TODO: for translation, pass context to handle_api_result! macro to get
                //       client language preference

                use errors::RawAPIError;
                let _err: RawAPIError = _err;

                println!("{:?}", _err);

                // TODO: internal error logging

                return RenderResponse::InternalServerError;

            }
        }
    }}
}

macro_rules! respond_json {
    (Some($payload:expr)) => {{

        use route;
        use serde_json;

        let response = JSONResponse {
            error: None,
            payload: Some(serde_json::to_value(&$payload))
        };

        route::RenderResponse::JSON(APIStatus::Ok, response)

    }};

    (None) => {{

        use route;

        let response = JSONResponse {
            error: None,
            payload: None
        };

        route::RenderResponse::JSON(APIStatus::Ok, response)

    }};
}

macro_rules! respond_json_with_error {
    ($api_status:expr; $err:expr; Some($payload:expr)) => {{

        use route;
        use serde_json;

        // TODO: remove
        // {
        //     use types::APIStatus;
        //     match $api_status {
        //         APIStatus::Ok => {
        //             // TODO: error logging??
        //             panic!("unexpected api status");
        //         },
        //         _ => {}
        //     }
        // };

        let response = JSONResponse {
            error: Some($err),
            payload: Some(serde_json::to_value(&$payload))
        };

        route::RenderResponse::JSON($api_status, response)

    }};

    ($api_status:expr; $err:expr; None) => {{

        use route;

        {
            use types::APIStatus;

            match $api_status {
                APIStatus::Ok => {
                    // TODO: error logging??
                    panic!("unexpected api status");
                },
                _ => {}
            }
        };

        let response = JSONResponse {
            error: Some($err),
            payload: None
        };

        route::RenderResponse::JSON($api_status, response)

    }};
}
