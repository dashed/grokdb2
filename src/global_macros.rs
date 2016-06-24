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

macro_rules! respond_json {
    ($response:expr; $payload:expr) => (
        let mut response = $response;

        use hyper;

        response.headers_mut().set((hyper::header::ContentType(
            mime!(Application/Json)
        )));

        // TODO: macro
        let mut stream = response.start().unwrap();
        serde_json::to_writer(&mut stream, &$payload);
    );
}
