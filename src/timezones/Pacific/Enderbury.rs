
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Pacific/Enderbury",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -38140,  // UTC offset -38140, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2177414660, FixedTimespan {  // 1901-01-01T10:35:40.000 UTC
            offset: -43200,  // UTC offset -43200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PHOT"),
        }),
        (307627200, FixedTimespan {  // 1979-10-01T12:00:00.000 UTC
            offset: -39600,  // UTC offset -39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PHOT"),
        }),
        (788958000, FixedTimespan {  // 1995-01-01T11:00:00.000 UTC
            offset: 46800,  // UTC offset 46800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PHOT"),
        }),
    ]},
};

