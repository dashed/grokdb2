/* 3rd-party imports */

use chrono::naive::datetime::NaiveDateTime;
use chrono::datetime::DateTime;
use chrono::offset::fixed::FixedOffset;
use datetime::zone::FixedTimespan;

/* local imports */

use timezones;
use types::UnixTimestamp;

/* ////////////////////////////////////////////////////////////////////////// */

#[inline]
pub fn get_timespan(pin: UnixTimestamp, timezone_key: &str) -> Option<FixedTimespan> {

    if let Some(result) = timezones::lookup(timezone_key) {

        let list_of_tz = result.fixed_timespans.rest;

        if list_of_tz.len() <= 0 {
            return Some(result.fixed_timespans.first.clone());
        }

        // invariant: list_of_tz is non-empty
        // invariant: list_of_tz is ordered from least recent to most recent
        for (index, &(ref time_start, ref timezone_item)) in list_of_tz.iter().enumerate() {

            if pin < *time_start {

                let chosen_idx = if index > 0 {
                    index - 1
                } else {
                    0
                };

                return Some(list_of_tz[chosen_idx].1.clone());
            }

        }

    }

    return None;
}

#[inline]
pub fn to_string(source: NaiveDateTime) -> String {

    let (offset, tz_name) = match get_timespan(source.timestamp(), "America/Toronto") {
        None => (0, "UTC".to_string()),
        Some(timespan) => (timespan.offset as i32, timespan.name.to_string())
    };

    let timezone = if offset >= 0 {
        FixedOffset::east(offset)
    } else {
        FixedOffset::west(-offset)
    };

    let time_format = format!("%B %-d, %Y %-l:%M:%S %p {tz_name} (%:z)", tz_name = tz_name);

    let date_time: DateTime<FixedOffset> = DateTime::from_utc(source, timezone);
    let formatted = date_time.format(&time_format);

    format!("{}", formatted)
}
