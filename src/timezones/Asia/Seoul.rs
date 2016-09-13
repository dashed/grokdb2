
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Asia/Seoul",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 30472,  // UTC offset 30472, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1948782472, FixedTimespan {  // 1908-03-31T15:32:08.000 UTC
            offset: 30600,  // UTC offset 30600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KST"),
        }),
        (-1830414600, FixedTimespan {  // 1911-12-31T15:30:00.000 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("JCST"),
        }),
        (-1017824400, FixedTimespan {  // 1937-09-30T15:00:00.000 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("JST"),
        }),
        (-767350800, FixedTimespan {  // 1945-09-07T15:00:00.000 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KST"),
        }),
        (-498128400, FixedTimespan {  // 1954-03-20T15:00:00.000 UTC
            offset: 30600,  // UTC offset 30600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KST"),
        }),
        (-462702600, FixedTimespan {  // 1955-05-04T15:30:00.000 UTC
            offset: 34200,  // UTC offset 30600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KDT"),
        }),
        (-451733400, FixedTimespan {  // 1955-09-08T14:30:00.000 UTC
            offset: 30600,  // UTC offset 30600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KST"),
        }),
        (-429784200, FixedTimespan {  // 1956-05-19T15:30:00.000 UTC
            offset: 34200,  // UTC offset 30600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KDT"),
        }),
        (-418296600, FixedTimespan {  // 1956-09-29T14:30:00.000 UTC
            offset: 30600,  // UTC offset 30600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KST"),
        }),
        (-399544200, FixedTimespan {  // 1957-05-04T15:30:00.000 UTC
            offset: 34200,  // UTC offset 30600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KDT"),
        }),
        (-387451800, FixedTimespan {  // 1957-09-21T14:30:00.000 UTC
            offset: 30600,  // UTC offset 30600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KST"),
        }),
        (-368094600, FixedTimespan {  // 1958-05-03T15:30:00.000 UTC
            offset: 34200,  // UTC offset 30600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KDT"),
        }),
        (-356002200, FixedTimespan {  // 1958-09-20T14:30:00.000 UTC
            offset: 30600,  // UTC offset 30600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KST"),
        }),
        (-336645000, FixedTimespan {  // 1959-05-02T15:30:00.000 UTC
            offset: 34200,  // UTC offset 30600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KDT"),
        }),
        (-324552600, FixedTimespan {  // 1959-09-19T14:30:00.000 UTC
            offset: 30600,  // UTC offset 30600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KST"),
        }),
        (-305195400, FixedTimespan {  // 1960-04-30T15:30:00.000 UTC
            offset: 34200,  // UTC offset 30600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KDT"),
        }),
        (-293103000, FixedTimespan {  // 1960-09-17T14:30:00.000 UTC
            offset: 30600,  // UTC offset 30600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KST"),
        }),
        (-264933000, FixedTimespan {  // 1961-08-09T15:30:00.000 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KST"),
        }),
        (547578000, FixedTimespan {  // 1987-05-09T17:00:00.000 UTC
            offset: 36000,  // UTC offset 32400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KDT"),
        }),
        (560883600, FixedTimespan {  // 1987-10-10T17:00:00.000 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KST"),
        }),
        (579027600, FixedTimespan {  // 1988-05-07T17:00:00.000 UTC
            offset: 36000,  // UTC offset 32400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("KDT"),
        }),
        (592333200, FixedTimespan {  // 1988-10-08T17:00:00.000 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KST"),
        }),
    ]},
};

