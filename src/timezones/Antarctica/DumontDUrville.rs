
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Antarctica/DumontDUrville",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 0,  // UTC offset 0, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("-00"),
        },
        rest: &[
        (-725846400, FixedTimespan {  // 1947-01-01T00:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PMT"),
        }),
        (-566992800, FixedTimespan {  // 1952-01-13T14:00:00.000 UTC
            offset: 0,  // UTC offset 0, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("-00"),
        }),
        (-415497600, FixedTimespan {  // 1956-11-01T00:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("DDUT"),
        }),
    ]},
};


