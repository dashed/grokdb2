
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Pacific/Chuuk",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 36428,  // UTC offset 36428, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2177489228, FixedTimespan {  // 1900-12-31T13:52:52.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CHUT"),
        }),
    ]},
};


