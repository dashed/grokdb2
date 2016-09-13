
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Pacific/Pohnpei",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 37972,  // UTC offset 37972, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2177490772, FixedTimespan {  // 1900-12-31T13:27:08.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PONT"),
        }),
    ]},
};

