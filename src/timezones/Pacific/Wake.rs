
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Pacific/Wake",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 39988,  // UTC offset 39988, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2177492788, FixedTimespan {  // 1900-12-31T12:53:32.000 UTC
            offset: 43200,  // UTC offset 43200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("WAKT"),
        }),
    ]},
};

