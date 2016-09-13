
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "America/Barbados",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -7291,  // UTC offset -7291, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1451685509, FixedTimespan {  // 1924-01-01T02:01:31.000 UTC
            offset: -7291,  // UTC offset -7291, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BMT"),
        }),
        (-1199224709, FixedTimespan {  // 1932-01-01T02:01:31.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (234943200, FixedTimespan {  // 1977-06-12T06:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (244616400, FixedTimespan {  // 1977-10-02T05:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (261554400, FixedTimespan {  // 1978-04-16T06:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (276066000, FixedTimespan {  // 1978-10-01T05:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (293004000, FixedTimespan {  // 1979-04-15T06:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (307515600, FixedTimespan {  // 1979-09-30T05:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (325058400, FixedTimespan {  // 1980-04-20T06:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (338706000, FixedTimespan {  // 1980-09-25T05:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
    ]},
};

