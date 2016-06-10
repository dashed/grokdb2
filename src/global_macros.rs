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
