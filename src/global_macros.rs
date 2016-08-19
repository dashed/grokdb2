/* ////////////////////////////////////////////////////////////////////////// */

macro_rules! respond_json {
    ($payload:expr) => {{

        use route;

        match serde_json::to_string(&$payload) {
            Ok(__json_response) => route::RenderResponse::RenderJSON(__json_response),
            Err(why) => {

                // TODO: error logging

                route::RenderResponse::RenderInternalServerError
            }
        }

    }};
}
