
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "America/Creston",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -22436,  // UTC offset -22436, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2713887964, FixedTimespan {  // 1884-01-01T06:13:56.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MST"),
        }),
        (-1680454800, FixedTimespan {  // 1916-10-01T07:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (-1627833600, FixedTimespan {  // 1918-06-02T08:00:00.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MST"),
        }),
    ]},
};


