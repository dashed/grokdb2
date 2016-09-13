
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Asia/Makassar",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 28656,  // UTC offset 28656, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1577951856, FixedTimespan {  // 1919-12-31T16:02:24.000 UTC
            offset: 28656,  // UTC offset 28656, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MMT"),
        }),
        (-1172908656, FixedTimespan {  // 1932-10-31T16:02:24.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("WITA"),
        }),
        (-880272000, FixedTimespan {  // 1942-02-08T16:00:00.000 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("JST"),
        }),
        (-766054800, FixedTimespan {  // 1945-09-22T15:00:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("WITA"),
        }),
    ]},
};

