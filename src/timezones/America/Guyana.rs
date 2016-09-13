
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "America/Guyana",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -7640,  // UTC offset -7640, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1730584360, FixedTimespan {  // 1915-03-01T02:07:20.000 UTC
            offset: -8100,  // UTC offset -8100, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("GBGT"),
        }),
        (-113694300, FixedTimespan {  // 1966-05-26T02:15:00.000 UTC
            offset: -8100,  // UTC offset -8100, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("GYT"),
        }),
        (176004900, FixedTimespan {  // 1975-07-31T02:15:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("GYT"),
        }),
        (662698800, FixedTimespan {  // 1991-01-01T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("GYT"),
        }),
    ]},
};

