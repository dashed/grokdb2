/* ////////////////////////////////////////////////////////////////////////// */

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
        use serde_json;

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
            payload: Some(serde_json::to_value(&$payload))
        };

        route::RenderResponse::JSON($api_status, response)

    }};

    ($api_status:expr; $err:expr; None) => {{

        use route;
        use serde_json;

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
