
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Asia/Kabul",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 16608,  // UTC offset 16608, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2524538208, FixedTimespan {  // 1889-12-31T19:23:12.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AFT"),
        }),
        (-788932800, FixedTimespan {  // 1944-12-31T20:00:00.000 UTC
            offset: 16200,  // UTC offset 16200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AFT"),
        }),
    ]},
};

