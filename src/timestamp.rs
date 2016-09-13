/* 3rd-party imports */

use chrono::naive::datetime::NaiveDateTime;
use chrono::datetime::DateTime;
use chrono::offset::fixed::FixedOffset;

/* ////////////////////////////////////////////////////////////////////////// */

#[inline]
pub fn to_string(source: NaiveDateTime) -> String {

    // TODO: fix
    let edt = 4 * 60 * 60;
    let est = 5 * 60 * 60;

    let timezone = FixedOffset::west(edt);
    let foo: DateTime<FixedOffset> = DateTime::from_utc(source, timezone);

    let formatted = foo.format("%B %-d, %Y %-l:%M %p (%:z)");

    format!("{}", formatted)
}
