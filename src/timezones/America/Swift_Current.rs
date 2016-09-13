
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "America/Swift_Current",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -24520,  // UTC offset -24520, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2030202680, FixedTimespan {  // 1905-09-01T06:48:40.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MST"),
        }),
        (-1632063600, FixedTimespan {  // 1918-04-14T09:00:00.000 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MDT"),
        }),
        (-1615132800, FixedTimespan {  // 1918-10-27T08:00:00.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MST"),
        }),
        (-880210800, FixedTimespan {  // 1942-02-09T09:00:00.000 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MWT"),
        }),
        (-769395600, FixedTimespan {  // 1945-08-14T23:00:00.000 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MPT"),
        }),
        (-765388800, FixedTimespan {  // 1945-09-30T08:00:00.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MST"),
        }),
        (-747241200, FixedTimespan {  // 1946-04-28T09:00:00.000 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MDT"),
        }),
        (-732729600, FixedTimespan {  // 1946-10-13T08:00:00.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MST"),
        }),
        (-715791600, FixedTimespan {  // 1947-04-27T09:00:00.000 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MDT"),
        }),
        (-702489600, FixedTimespan {  // 1947-09-28T08:00:00.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MST"),
        }),
        (-684342000, FixedTimespan {  // 1948-04-25T09:00:00.000 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MDT"),
        }),
        (-671040000, FixedTimespan {  // 1948-09-26T08:00:00.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MST"),
        }),
        (-652892400, FixedTimespan {  // 1949-04-24T09:00:00.000 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MDT"),
        }),
        (-639590400, FixedTimespan {  // 1949-09-25T08:00:00.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MST"),
        }),
        (-400086000, FixedTimespan {  // 1957-04-28T09:00:00.000 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MDT"),
        }),
        (-384364800, FixedTimespan {  // 1957-10-27T08:00:00.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MST"),
        }),
        (-337186800, FixedTimespan {  // 1959-04-26T09:00:00.000 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MDT"),
        }),
        (-321465600, FixedTimespan {  // 1959-10-25T08:00:00.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MST"),
        }),
        (-305737200, FixedTimespan {  // 1960-04-24T09:00:00.000 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MDT"),
        }),
        (-292435200, FixedTimespan {  // 1960-09-25T08:00:00.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MST"),
        }),
        (-273682800, FixedTimespan {  // 1961-04-30T09:00:00.000 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MDT"),
        }),
        (-260985600, FixedTimespan {  // 1961-09-24T08:00:00.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MST"),
        }),
        (73472400, FixedTimespan {  // 1972-04-30T09:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
    ]},
};

