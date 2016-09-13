
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Asia/Baghdad",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 10660,  // UTC offset 10660, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2524532260, FixedTimespan {  // 1889-12-31T21:02:20.000 UTC
            offset: 10656,  // UTC offset 10656, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BMT"),
        }),
        (-1641005856, FixedTimespan {  // 1917-12-31T21:02:24.000 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (389048400, FixedTimespan {  // 1982-04-30T21:00:00.000 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (402264000, FixedTimespan {  // 1982-09-30T20:00:00.000 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (417906000, FixedTimespan {  // 1983-03-30T21:00:00.000 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (433800000, FixedTimespan {  // 1983-09-30T20:00:00.000 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (449614800, FixedTimespan {  // 1984-03-31T21:00:00.000 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (465422400, FixedTimespan {  // 1984-09-30T20:00:00.000 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (481150800, FixedTimespan {  // 1985-03-31T21:00:00.000 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (496792800, FixedTimespan {  // 1985-09-28T22:00:00.000 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (512517600, FixedTimespan {  // 1986-03-29T22:00:00.000 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (528242400, FixedTimespan {  // 1986-09-27T22:00:00.000 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (543967200, FixedTimespan {  // 1987-03-28T22:00:00.000 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (559692000, FixedTimespan {  // 1987-09-26T22:00:00.000 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (575416800, FixedTimespan {  // 1988-03-26T22:00:00.000 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (591141600, FixedTimespan {  // 1988-09-24T22:00:00.000 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (606866400, FixedTimespan {  // 1989-03-25T22:00:00.000 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (622591200, FixedTimespan {  // 1989-09-23T22:00:00.000 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (638316000, FixedTimespan {  // 1990-03-24T22:00:00.000 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (654645600, FixedTimespan {  // 1990-09-29T22:00:00.000 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (670464000, FixedTimespan {  // 1991-04-01T00:00:00.000 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (686275200, FixedTimespan {  // 1991-10-01T00:00:00.000 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (702086400, FixedTimespan {  // 1992-04-01T00:00:00.000 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (717897600, FixedTimespan {  // 1992-10-01T00:00:00.000 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (733622400, FixedTimespan {  // 1993-04-01T00:00:00.000 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (749433600, FixedTimespan {  // 1993-10-01T00:00:00.000 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (765158400, FixedTimespan {  // 1994-04-01T00:00:00.000 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (780969600, FixedTimespan {  // 1994-10-01T00:00:00.000 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (796694400, FixedTimespan {  // 1995-04-01T00:00:00.000 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (812505600, FixedTimespan {  // 1995-10-01T00:00:00.000 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (828316800, FixedTimespan {  // 1996-04-01T00:00:00.000 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (844128000, FixedTimespan {  // 1996-10-01T00:00:00.000 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (859852800, FixedTimespan {  // 1997-04-01T00:00:00.000 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (875664000, FixedTimespan {  // 1997-10-01T00:00:00.000 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (891388800, FixedTimespan {  // 1998-04-01T00:00:00.000 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (907200000, FixedTimespan {  // 1998-10-01T00:00:00.000 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (922924800, FixedTimespan {  // 1999-04-01T00:00:00.000 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (938736000, FixedTimespan {  // 1999-10-01T00:00:00.000 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (954547200, FixedTimespan {  // 2000-04-01T00:00:00.000 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (970358400, FixedTimespan {  // 2000-10-01T00:00:00.000 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (986083200, FixedTimespan {  // 2001-04-01T00:00:00.000 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (1001894400, FixedTimespan {  // 2001-10-01T00:00:00.000 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (1017619200, FixedTimespan {  // 2002-04-01T00:00:00.000 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (1033430400, FixedTimespan {  // 2002-10-01T00:00:00.000 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (1049155200, FixedTimespan {  // 2003-04-01T00:00:00.000 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (1064966400, FixedTimespan {  // 2003-10-01T00:00:00.000 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (1080777600, FixedTimespan {  // 2004-04-01T00:00:00.000 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (1096588800, FixedTimespan {  // 2004-10-01T00:00:00.000 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (1112313600, FixedTimespan {  // 2005-04-01T00:00:00.000 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (1128124800, FixedTimespan {  // 2005-10-01T00:00:00.000 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (1143849600, FixedTimespan {  // 2006-04-01T00:00:00.000 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (1159660800, FixedTimespan {  // 2006-10-01T00:00:00.000 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
        (1175385600, FixedTimespan {  // 2007-04-01T00:00:00.000 UTC
            offset: 14400,  // UTC offset 10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("ADT"),
        }),
        (1191196800, FixedTimespan {  // 2007-10-01T00:00:00.000 UTC
            offset: 10800,  // UTC offset 10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AST"),
        }),
    ]},
};

