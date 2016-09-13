
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "America/Argentina/Cordoba",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -13392,  // UTC offset -13392, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2372098608, FixedTimespan {  // 1894-10-31T03:43:12.000 UTC
            offset: -13392,  // UTC offset -13392, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CMT"),
        }),
        (-1567455408, FixedTimespan {  // 1920-05-01T03:43:12.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (-1233432000, FixedTimespan {  // 1930-12-01T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (-1222981200, FixedTimespan {  // 1931-04-01T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (-1205956800, FixedTimespan {  // 1931-10-15T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (-1194037200, FixedTimespan {  // 1932-03-01T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (-1172865600, FixedTimespan {  // 1932-11-01T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (-1162501200, FixedTimespan {  // 1933-03-01T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (-1141329600, FixedTimespan {  // 1933-11-01T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (-1130965200, FixedTimespan {  // 1934-03-01T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (-1109793600, FixedTimespan {  // 1934-11-01T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (-1099429200, FixedTimespan {  // 1935-03-01T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (-1078257600, FixedTimespan {  // 1935-11-01T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (-1067806800, FixedTimespan {  // 1936-03-01T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (-1046635200, FixedTimespan {  // 1936-11-01T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (-1036270800, FixedTimespan {  // 1937-03-01T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (-1015099200, FixedTimespan {  // 1937-11-01T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (-1004734800, FixedTimespan {  // 1938-03-01T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (-983563200, FixedTimespan {  // 1938-11-01T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (-973198800, FixedTimespan {  // 1939-03-01T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (-952027200, FixedTimespan {  // 1939-11-01T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (-941576400, FixedTimespan {  // 1940-03-01T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (-931032000, FixedTimespan {  // 1940-07-01T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (-900882000, FixedTimespan {  // 1941-06-15T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (-890337600, FixedTimespan {  // 1941-10-15T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (-833749200, FixedTimespan {  // 1943-08-01T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (-827265600, FixedTimespan {  // 1943-10-15T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (-752274000, FixedTimespan {  // 1946-03-01T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (-733780800, FixedTimespan {  // 1946-10-01T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (-197326800, FixedTimespan {  // 1963-10-01T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (-190843200, FixedTimespan {  // 1963-12-15T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (-184194000, FixedTimespan {  // 1964-03-01T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (-164491200, FixedTimespan {  // 1964-10-15T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (-152658000, FixedTimespan {  // 1965-03-01T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (-132955200, FixedTimespan {  // 1965-10-15T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (-121122000, FixedTimespan {  // 1966-03-01T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (-101419200, FixedTimespan {  // 1966-10-15T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (-86821200, FixedTimespan {  // 1967-04-02T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (-71092800, FixedTimespan {  // 1967-10-01T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (-54766800, FixedTimespan {  // 1968-04-07T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (-39038400, FixedTimespan {  // 1968-10-06T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (-23317200, FixedTimespan {  // 1969-04-06T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (-7588800, FixedTimespan {  // 1969-10-05T04:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (128142000, FixedTimespan {  // 1974-01-23T03:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (136605600, FixedTimespan {  // 1974-05-01T02:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (596948400, FixedTimespan {  // 1988-12-01T03:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (605066400, FixedTimespan {  // 1989-03-05T02:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (624423600, FixedTimespan {  // 1989-10-15T03:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (636516000, FixedTimespan {  // 1990-03-04T02:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (656478000, FixedTimespan {  // 1990-10-21T03:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (667965600, FixedTimespan {  // 1991-03-03T02:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("WART"),
        }),
        (687931200, FixedTimespan {  // 1991-10-20T04:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (699415200, FixedTimespan {  // 1992-03-01T02:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (719377200, FixedTimespan {  // 1992-10-18T03:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (731469600, FixedTimespan {  // 1993-03-07T02:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (938919600, FixedTimespan {  // 1999-10-03T03:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (952052400, FixedTimespan {  // 2000-03-03T03:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (1198983600, FixedTimespan {  // 2007-12-30T03:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (1205632800, FixedTimespan {  // 2008-03-16T02:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
        (1224385200, FixedTimespan {  // 2008-10-19T03:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ARST"),
        }),
        (1237082400, FixedTimespan {  // 2009-03-15T02:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("ART"),
        }),
    ]},
};


