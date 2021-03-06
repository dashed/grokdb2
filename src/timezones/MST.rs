
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "MST",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MST"),
        },
        rest: &[
    ]},
};


