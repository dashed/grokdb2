
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Pacific/Palau",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 32276,  // UTC offset 32276, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2177485076, FixedTimespan {  // 1900-12-31T15:02:04.000 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PWT"),
        }),
    ]},
};


