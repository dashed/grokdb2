
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Pacific/Noumea",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 39948,  // UTC offset 39948, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1829387148, FixedTimespan {  // 1912-01-12T12:54:12.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("NCT"),
        }),
        (250002000, FixedTimespan {  // 1977-12-03T13:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("NCST"),
        }),
        (257342400, FixedTimespan {  // 1978-02-26T12:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("NCT"),
        }),
        (281451600, FixedTimespan {  // 1978-12-02T13:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("NCST"),
        }),
        (288878400, FixedTimespan {  // 1979-02-26T12:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("NCT"),
        }),
        (849366000, FixedTimespan {  // 1996-11-30T15:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("NCST"),
        }),
        (857228400, FixedTimespan {  // 1997-03-01T15:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("NCT"),
        }),
    ]},
};

