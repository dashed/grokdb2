
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "America/Recife",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -6024,  // UTC offset -6024, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1767219576, FixedTimespan {  // 1914-01-01T01:40:24.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BRT"),
        }),
        (-1206957600, FixedTimespan {  // 1931-10-03T14:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("BRST"),
        }),
        (-1191362400, FixedTimespan {  // 1932-04-01T02:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BRT"),
        }),
        (-1175374800, FixedTimespan {  // 1932-10-03T03:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("BRST"),
        }),
        (-1159826400, FixedTimespan {  // 1933-04-01T02:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BRT"),
        }),
        (-633819600, FixedTimespan {  // 1949-12-01T03:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("BRST"),
        }),
        (-622069200, FixedTimespan {  // 1950-04-16T03:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BRT"),
        }),
        (-602283600, FixedTimespan {  // 1950-12-01T03:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("BRST"),
        }),
        (-591832800, FixedTimespan {  // 1951-04-01T02:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BRT"),
        }),
        (-570747600, FixedTimespan {  // 1951-12-01T03:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("BRST"),
        }),
        (-560210400, FixedTimespan {  // 1952-04-01T02:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BRT"),
        }),
        (-539125200, FixedTimespan {  // 1952-12-01T03:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("BRST"),
        }),
        (-531352800, FixedTimespan {  // 1953-03-01T02:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BRT"),
        }),
        (-191365200, FixedTimespan {  // 1963-12-09T03:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("BRST"),
        }),
        (-184197600, FixedTimespan {  // 1964-03-01T02:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BRT"),
        }),
        (-155163600, FixedTimespan {  // 1965-01-31T03:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("BRST"),
        }),
        (-150069600, FixedTimespan {  // 1965-03-31T02:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BRT"),
        }),
        (-128898000, FixedTimespan {  // 1965-12-01T03:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("BRST"),
        }),
        (-121125600, FixedTimespan {  // 1966-03-01T02:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BRT"),
        }),
        (-99954000, FixedTimespan {  // 1966-11-01T03:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("BRST"),
        }),
        (-89589600, FixedTimespan {  // 1967-03-01T02:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BRT"),
        }),
        (-68418000, FixedTimespan {  // 1967-11-01T03:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("BRST"),
        }),
        (-57967200, FixedTimespan {  // 1968-03-01T02:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BRT"),
        }),
        (499748400, FixedTimespan {  // 1985-11-02T03:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("BRST"),
        }),
        (511236000, FixedTimespan {  // 1986-03-15T02:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BRT"),
        }),
        (530593200, FixedTimespan {  // 1986-10-25T03:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("BRST"),
        }),
        (540266400, FixedTimespan {  // 1987-02-14T02:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BRT"),
        }),
        (562129200, FixedTimespan {  // 1987-10-25T03:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("BRST"),
        }),
        (571197600, FixedTimespan {  // 1988-02-07T02:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BRT"),
        }),
        (592974000, FixedTimespan {  // 1988-10-16T03:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("BRST"),
        }),
        (602042400, FixedTimespan {  // 1989-01-29T02:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BRT"),
        }),
        (624423600, FixedTimespan {  // 1989-10-15T03:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("BRST"),
        }),
        (634701600, FixedTimespan {  // 1990-02-11T02:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BRT"),
        }),
        (938919600, FixedTimespan {  // 1999-10-03T03:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("BRST"),
        }),
        (951616800, FixedTimespan {  // 2000-02-27T02:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BRT"),
        }),
        (970974000, FixedTimespan {  // 2000-10-08T03:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("BRST"),
        }),
        (971575200, FixedTimespan {  // 2000-10-15T02:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BRT"),
        }),
        (1003028400, FixedTimespan {  // 2001-10-14T03:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("BRST"),
        }),
        (1013911200, FixedTimespan {  // 2002-02-17T02:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BRT"),
        }),
    ]},
};


