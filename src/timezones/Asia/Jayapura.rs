
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Asia/Jayapura",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 33768,  // UTC offset 33768, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1172913768, FixedTimespan {  // 1932-10-31T14:37:12.000 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("WIT"),
        }),
        (-799491600, FixedTimespan {  // 1944-08-31T15:00:00.000 UTC
            offset: 34200,  // UTC offset 34200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACST"),
        }),
        (-189423000, FixedTimespan {  // 1963-12-31T14:30:00.000 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("WIT"),
        }),
    ]},
};


