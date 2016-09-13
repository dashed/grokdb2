
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Australia/Brisbane",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 36728,  // UTC offset 36728, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2366791928, FixedTimespan {  // 1894-12-31T13:47:52.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (-1672567140, FixedTimespan {  // 1916-12-31T14:01:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (-1665392400, FixedTimespan {  // 1917-03-24T15:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (-883641600, FixedTimespan {  // 1941-12-31T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (-876128400, FixedTimespan {  // 1942-03-28T15:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (-860400000, FixedTimespan {  // 1942-09-26T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (-844678800, FixedTimespan {  // 1943-03-27T15:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (-828345600, FixedTimespan {  // 1943-10-02T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (-813229200, FixedTimespan {  // 1944-03-25T15:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (57686400, FixedTimespan {  // 1971-10-30T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (67968000, FixedTimespan {  // 1972-02-26T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (625593600, FixedTimespan {  // 1989-10-28T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (636480000, FixedTimespan {  // 1990-03-03T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (657043200, FixedTimespan {  // 1990-10-27T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (667929600, FixedTimespan {  // 1991-03-02T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (688492800, FixedTimespan {  // 1991-10-26T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (699465600, FixedTimespan {  // 1992-03-01T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
    ]},
};


