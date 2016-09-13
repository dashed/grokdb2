
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Indian/Reunion",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 13312,  // UTC offset 13312, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1848886912, FixedTimespan {  // 1911-05-31T20:18:08.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("RET"),
        }),
    ]},
};


