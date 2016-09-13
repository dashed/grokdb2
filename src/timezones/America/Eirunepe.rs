
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "America/Eirunepe",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -12032,  // UTC offset -12032, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1767213568, FixedTimespan {  // 1914-01-01T03:20:32.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACT"),
        }),
        (-1206950400, FixedTimespan {  // 1931-10-03T16:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ACST"),
        }),
        (-1191355200, FixedTimespan {  // 1932-04-01T04:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACT"),
        }),
        (-1175367600, FixedTimespan {  // 1932-10-03T05:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ACST"),
        }),
        (-1159819200, FixedTimespan {  // 1933-04-01T04:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACT"),
        }),
        (-633812400, FixedTimespan {  // 1949-12-01T05:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ACST"),
        }),
        (-622062000, FixedTimespan {  // 1950-04-16T05:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACT"),
        }),
        (-602276400, FixedTimespan {  // 1950-12-01T05:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ACST"),
        }),
        (-591825600, FixedTimespan {  // 1951-04-01T04:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACT"),
        }),
        (-570740400, FixedTimespan {  // 1951-12-01T05:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ACST"),
        }),
        (-560203200, FixedTimespan {  // 1952-04-01T04:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACT"),
        }),
        (-539118000, FixedTimespan {  // 1952-12-01T05:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ACST"),
        }),
        (-531345600, FixedTimespan {  // 1953-03-01T04:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACT"),
        }),
        (-191358000, FixedTimespan {  // 1963-12-09T05:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ACST"),
        }),
        (-184190400, FixedTimespan {  // 1964-03-01T04:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACT"),
        }),
        (-155156400, FixedTimespan {  // 1965-01-31T05:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ACST"),
        }),
        (-150062400, FixedTimespan {  // 1965-03-31T04:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACT"),
        }),
        (-128890800, FixedTimespan {  // 1965-12-01T05:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ACST"),
        }),
        (-121118400, FixedTimespan {  // 1966-03-01T04:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACT"),
        }),
        (-99946800, FixedTimespan {  // 1966-11-01T05:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ACST"),
        }),
        (-89582400, FixedTimespan {  // 1967-03-01T04:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACT"),
        }),
        (-68410800, FixedTimespan {  // 1967-11-01T05:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ACST"),
        }),
        (-57960000, FixedTimespan {  // 1968-03-01T04:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACT"),
        }),
        (499755600, FixedTimespan {  // 1985-11-02T05:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ACST"),
        }),
        (511243200, FixedTimespan {  // 1986-03-15T04:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACT"),
        }),
        (530600400, FixedTimespan {  // 1986-10-25T05:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ACST"),
        }),
        (540273600, FixedTimespan {  // 1987-02-14T04:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACT"),
        }),
        (562136400, FixedTimespan {  // 1987-10-25T05:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ACST"),
        }),
        (571204800, FixedTimespan {  // 1988-02-07T04:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACT"),
        }),
        (750834000, FixedTimespan {  // 1993-10-17T05:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ACST"),
        }),
        (761716800, FixedTimespan {  // 1994-02-20T04:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACT"),
        }),
        (1214283600, FixedTimespan {  // 2008-06-24T05:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AMT"),
        }),
        (1384056000, FixedTimespan {  // 2013-11-10T04:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ACT"),
        }),
    ]},
};

