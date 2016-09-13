
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Asia/Shanghai",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 29143,  // UTC offset 29143, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2177481943, FixedTimespan {  // 1900-12-31T15:54:17.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (-933494400, FixedTimespan {  // 1940-06-02T16:00:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (-923130000, FixedTimespan {  // 1940-09-30T15:00:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (-908784000, FixedTimespan {  // 1941-03-15T16:00:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (-891594000, FixedTimespan {  // 1941-09-30T15:00:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (515520000, FixedTimespan {  // 1986-05-03T16:00:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (527007600, FixedTimespan {  // 1986-09-13T15:00:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (545155200, FixedTimespan {  // 1987-04-11T16:00:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (558457200, FixedTimespan {  // 1987-09-12T15:00:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (576604800, FixedTimespan {  // 1988-04-09T16:00:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (589906800, FixedTimespan {  // 1988-09-10T15:00:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (608659200, FixedTimespan {  // 1989-04-15T16:00:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (621961200, FixedTimespan {  // 1989-09-16T15:00:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (640108800, FixedTimespan {  // 1990-04-14T16:00:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (653410800, FixedTimespan {  // 1990-09-15T15:00:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (671558400, FixedTimespan {  // 1991-04-13T16:00:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (684860400, FixedTimespan {  // 1991-09-14T15:00:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
    ]},
};

