
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Australia/Perth",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 27804,  // UTC offset 27804, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2337925404, FixedTimespan {  // 1895-11-30T16:16:36.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AWST"),
        }),
        (-1672559940, FixedTimespan {  // 1916-12-31T16:01:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AWDT"),
        }),
        (-1665385200, FixedTimespan {  // 1917-03-24T17:00:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AWST"),
        }),
        (-883634400, FixedTimespan {  // 1941-12-31T18:00:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AWDT"),
        }),
        (-876121200, FixedTimespan {  // 1942-03-28T17:00:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AWST"),
        }),
        (-860392800, FixedTimespan {  // 1942-09-26T18:00:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AWDT"),
        }),
        (-844671600, FixedTimespan {  // 1943-03-27T17:00:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AWST"),
        }),
        (152042400, FixedTimespan {  // 1974-10-26T18:00:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AWDT"),
        }),
        (162928800, FixedTimespan {  // 1975-03-01T18:00:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AWST"),
        }),
        (436298400, FixedTimespan {  // 1983-10-29T18:00:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AWDT"),
        }),
        (447184800, FixedTimespan {  // 1984-03-03T18:00:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AWST"),
        }),
        (690314400, FixedTimespan {  // 1991-11-16T18:00:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AWDT"),
        }),
        (699472800, FixedTimespan {  // 1992-03-01T18:00:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AWST"),
        }),
        (1165082400, FixedTimespan {  // 2006-12-02T18:00:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AWDT"),
        }),
        (1174759200, FixedTimespan {  // 2007-03-24T18:00:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AWST"),
        }),
        (1193508000, FixedTimespan {  // 2007-10-27T18:00:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AWDT"),
        }),
        (1206813600, FixedTimespan {  // 2008-03-29T18:00:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AWST"),
        }),
        (1224957600, FixedTimespan {  // 2008-10-25T18:00:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AWDT"),
        }),
        (1238263200, FixedTimespan {  // 2009-03-28T18:00:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AWST"),
        }),
    ]},
};

