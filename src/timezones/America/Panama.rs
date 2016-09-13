
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "America/Panama",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -16912,  // UTC offset -16912, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2524504688, FixedTimespan {  // 1890-01-01T04:41:52.000 UTC
            offset: -16824,  // UTC offset -16824, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CMT"),
        }),
        (-1946920776, FixedTimespan {  // 1908-04-22T04:40:24.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
    ]},
};

