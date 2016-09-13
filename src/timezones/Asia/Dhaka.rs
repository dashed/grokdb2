
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Asia/Dhaka",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 21700,  // UTC offset 21700, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2524543300, FixedTimespan {  // 1889-12-31T17:58:20.000 UTC
            offset: 21200,  // UTC offset 21200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HMT"),
        }),
        (-891582800, FixedTimespan {  // 1941-09-30T18:06:40.000 UTC
            offset: 23400,  // UTC offset 23400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BURT"),
        }),
        (-872058600, FixedTimespan {  // 1942-05-14T17:30:00.000 UTC
            offset: 19800,  // UTC offset 19800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IST"),
        }),
        (-862637400, FixedTimespan {  // 1942-08-31T18:30:00.000 UTC
            offset: 23400,  // UTC offset 23400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BURT"),
        }),
        (-576138600, FixedTimespan {  // 1951-09-29T17:30:00.000 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("DACT"),
        }),
        (38772000, FixedTimespan {  // 1971-03-25T18:00:00.000 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BDT"),
        }),
        (1245430800, FixedTimespan {  // 2009-06-19T17:00:00.000 UTC
            offset: 25200,  // UTC offset 21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("BDST"),
        }),
        (1262278800, FixedTimespan {  // 2009-12-31T17:00:00.000 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BDT"),
        }),
    ]},
};

