
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "America/Paramaribo",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -8360,  // UTC offset -8360, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1861911640, FixedTimespan {  // 1911-01-01T02:19:20.000 UTC
            offset: -8348,  // UTC offset -8348, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PMT"),
        }),
        (-1104529252, FixedTimespan {  // 1935-01-01T02:19:08.000 UTC
            offset: -8364,  // UTC offset -8364, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PMT"),
        }),
        (-765322836, FixedTimespan {  // 1945-10-01T02:19:24.000 UTC
            offset: -9000,  // UTC offset -9000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("NEGT"),
        }),
        (185682600, FixedTimespan {  // 1975-11-20T02:30:00.000 UTC
            offset: -9000,  // UTC offset -9000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("SRT"),
        }),
        (465445800, FixedTimespan {  // 1984-10-01T02:30:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("SRT"),
        }),
    ]},
};


