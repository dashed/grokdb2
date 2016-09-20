/* 3rd-party imports */

use chrono::naive::datetime::NaiveDateTime;
use chrono::datetime::DateTime;
use chrono::offset::fixed::FixedOffset;
use datetime::zone::FixedTimespan;
use chrono::offset::utc::UTC;

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
        for (index, &(ref time_start, ref _timezone_item)) in list_of_tz.iter().enumerate() {

            if pin < *time_start {

                // pick previous item, otherwise the first item
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

    let timestamp = source.timestamp();

    /* generate relative times */

    let rel_time = relative_time(timestamp, UTC::now().timestamp());

    /* generate human string datetime */

    let (offset, tz_name) = match get_timespan(timestamp, "America/Toronto") {
        None => (0, "UTC".to_string()),
        Some(timespan) => (timespan.offset as i32, timespan.name.to_string())
    };

    let timezone = if offset >= 0 {
        FixedOffset::east(offset)
    } else {
        FixedOffset::west(-offset)
    };

    let time_format = format!("%B %-d, %Y %-l:%M:%S %p {tz_name} (%:z) ({rel_time})",
        tz_name = tz_name,
        rel_time = rel_time);

    let date_time: DateTime<FixedOffset> = DateTime::from_utc(source, timezone);
    let formatted = date_time.format(&time_format);

    format!("{}", formatted)
}

#[inline]
pub fn relative_time_from_timestamp(source: UnixTimestamp) -> String {

    /* generate relative times */

    let rel_time = relative_time(source, UTC::now().timestamp());

    return rel_time;
}

#[inline]
fn relative_time(from: i64, to: i64) -> String {

    let elapsed_num: u64 = (to - from).abs() as u64;
    let range = Timerange::new(elapsed_num).print(3);

    if to > from {
        // past
        format!("{} ago", range)
    } else if to == from {
        // now
        format!("just now")
    } else {
        // future
        format!("{} into the future", range)
    }
}

struct Timerange {
    range: u64
}

impl Timerange {

    fn new(range: u64) -> Timerange {
        Timerange {
            range: range
        }
    }

    fn floor_time_unit(&self) -> (u64, u64, String) {

        let sec_per_minute: f64 = 60f64;
        let sec_per_hour: f64 = sec_per_minute * 60f64;
        let sec_per_day: f64 = sec_per_hour * 24f64;
        let sec_per_month: f64 = sec_per_day * 30f64;
        let sec_per_year: f64 = sec_per_day * 365f64;

        let mut elapsed = self.range as f64;
        let mut remainder: f64 = 0f64;
        let unit;

        if elapsed < sec_per_minute {
            unit = "second";
        } else if elapsed < sec_per_hour {
            remainder = elapsed % sec_per_minute;
            elapsed = (elapsed / sec_per_minute).floor();
            unit = "minute"
        } else if elapsed < sec_per_day {
            remainder = elapsed % sec_per_hour;
            elapsed = (elapsed / sec_per_hour).floor();
            unit = "hour"
        } else if elapsed < sec_per_month {
            remainder = elapsed % sec_per_day;
            elapsed = (elapsed / sec_per_day).floor();
            unit = "day"
        } else if elapsed < sec_per_year {
            remainder = elapsed % sec_per_month;
            elapsed = (elapsed / sec_per_month).floor();
            unit = "month"
        } else {
            remainder = elapsed % sec_per_year;
            elapsed = (elapsed / sec_per_year).floor();
            unit = "year"
        }

        // pluralize
        let unit = if elapsed <= 1f64 {
            unit.to_owned()
        } else {
            format!("{}s", unit)
        };

        let elapsed = elapsed as u64;
        let remainder = remainder as u64;

        return (elapsed, remainder, unit);
    }

    fn print(&self, depth: u32) -> String {

        let (elapsed, remainder, unit) = self.floor_time_unit();

        if remainder <= 0 || depth <= 1 {
            return format!("{} {}", elapsed, unit);
        }

        let pretty_remainder = Timerange::new(remainder).print(depth - 1);

        if remainder < 60 || depth <= 2 {
            return format!("{} {}, and {}", elapsed, unit, pretty_remainder);
        }


        return format!("{} {}, {}", elapsed, unit, pretty_remainder);

    }
}
