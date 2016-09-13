
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Asia/Barnaul",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 20100,  // UTC offset 20100, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1579844100, FixedTimespan {  // 1919-12-09T18:25:00.000 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+06"),
        }),
        (-1247551200, FixedTimespan {  // 1930-06-20T18:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (354906000, FixedTimespan {  // 1981-03-31T17:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (370713600, FixedTimespan {  // 1981-09-30T16:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (386442000, FixedTimespan {  // 1982-03-31T17:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (402249600, FixedTimespan {  // 1982-09-30T16:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (417978000, FixedTimespan {  // 1983-03-31T17:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (433785600, FixedTimespan {  // 1983-09-30T16:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (449600400, FixedTimespan {  // 1984-03-31T17:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (465332400, FixedTimespan {  // 1984-09-29T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (481057200, FixedTimespan {  // 1985-03-30T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (496782000, FixedTimespan {  // 1985-09-28T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (512506800, FixedTimespan {  // 1986-03-29T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (528231600, FixedTimespan {  // 1986-09-27T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (543956400, FixedTimespan {  // 1987-03-28T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (559681200, FixedTimespan {  // 1987-09-26T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (575406000, FixedTimespan {  // 1988-03-26T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (591130800, FixedTimespan {  // 1988-09-24T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (606855600, FixedTimespan {  // 1989-03-25T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (622580400, FixedTimespan {  // 1989-09-23T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (638305200, FixedTimespan {  // 1990-03-24T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (654634800, FixedTimespan {  // 1990-09-29T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (670359600, FixedTimespan {  // 1991-03-30T19:00:00.000 UTC
            offset: 25200,  // UTC offset 21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+07"),
        }),
        (686088000, FixedTimespan {  // 1991-09-28T20:00:00.000 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+06"),
        }),
        (695764800, FixedTimespan {  // 1992-01-18T20:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (701809200, FixedTimespan {  // 1992-03-28T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (717534000, FixedTimespan {  // 1992-09-26T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (733258800, FixedTimespan {  // 1993-03-27T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (748983600, FixedTimespan {  // 1993-09-25T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (764708400, FixedTimespan {  // 1994-03-26T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (780433200, FixedTimespan {  // 1994-09-24T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (796158000, FixedTimespan {  // 1995-03-25T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (801590400, FixedTimespan {  // 1995-05-27T16:00:00.000 UTC
            offset: 25200,  // UTC offset 21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+07"),
        }),
        (811886400, FixedTimespan {  // 1995-09-23T20:00:00.000 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+06"),
        }),
        (828216000, FixedTimespan {  // 1996-03-30T20:00:00.000 UTC
            offset: 25200,  // UTC offset 21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+07"),
        }),
        (846360000, FixedTimespan {  // 1996-10-26T20:00:00.000 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+06"),
        }),
        (859665600, FixedTimespan {  // 1997-03-29T20:00:00.000 UTC
            offset: 25200,  // UTC offset 21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+07"),
        }),
        (877809600, FixedTimespan {  // 1997-10-25T20:00:00.000 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+06"),
        }),
        (891115200, FixedTimespan {  // 1998-03-28T20:00:00.000 UTC
            offset: 25200,  // UTC offset 21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+07"),
        }),
        (909259200, FixedTimespan {  // 1998-10-24T20:00:00.000 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+06"),
        }),
        (922564800, FixedTimespan {  // 1999-03-27T20:00:00.000 UTC
            offset: 25200,  // UTC offset 21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+07"),
        }),
        (941313600, FixedTimespan {  // 1999-10-30T20:00:00.000 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+06"),
        }),
        (954014400, FixedTimespan {  // 2000-03-25T20:00:00.000 UTC
            offset: 25200,  // UTC offset 21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+07"),
        }),
        (972763200, FixedTimespan {  // 2000-10-28T20:00:00.000 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+06"),
        }),
        (985464000, FixedTimespan {  // 2001-03-24T20:00:00.000 UTC
            offset: 25200,  // UTC offset 21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+07"),
        }),
        (1004212800, FixedTimespan {  // 2001-10-27T20:00:00.000 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+06"),
        }),
        (1017518400, FixedTimespan {  // 2002-03-30T20:00:00.000 UTC
            offset: 25200,  // UTC offset 21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+07"),
        }),
        (1035662400, FixedTimespan {  // 2002-10-26T20:00:00.000 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+06"),
        }),
        (1048968000, FixedTimespan {  // 2003-03-29T20:00:00.000 UTC
            offset: 25200,  // UTC offset 21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+07"),
        }),
        (1067112000, FixedTimespan {  // 2003-10-25T20:00:00.000 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+06"),
        }),
        (1080417600, FixedTimespan {  // 2004-03-27T20:00:00.000 UTC
            offset: 25200,  // UTC offset 21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+07"),
        }),
        (1099166400, FixedTimespan {  // 2004-10-30T20:00:00.000 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+06"),
        }),
        (1111867200, FixedTimespan {  // 2005-03-26T20:00:00.000 UTC
            offset: 25200,  // UTC offset 21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+07"),
        }),
        (1130616000, FixedTimespan {  // 2005-10-29T20:00:00.000 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+06"),
        }),
        (1143316800, FixedTimespan {  // 2006-03-25T20:00:00.000 UTC
            offset: 25200,  // UTC offset 21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+07"),
        }),
        (1162065600, FixedTimespan {  // 2006-10-28T20:00:00.000 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+06"),
        }),
        (1174766400, FixedTimespan {  // 2007-03-24T20:00:00.000 UTC
            offset: 25200,  // UTC offset 21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+07"),
        }),
        (1193515200, FixedTimespan {  // 2007-10-27T20:00:00.000 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+06"),
        }),
        (1206820800, FixedTimespan {  // 2008-03-29T20:00:00.000 UTC
            offset: 25200,  // UTC offset 21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+07"),
        }),
        (1224964800, FixedTimespan {  // 2008-10-25T20:00:00.000 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+06"),
        }),
        (1238270400, FixedTimespan {  // 2009-03-28T20:00:00.000 UTC
            offset: 25200,  // UTC offset 21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+07"),
        }),
        (1256414400, FixedTimespan {  // 2009-10-24T20:00:00.000 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+06"),
        }),
        (1269720000, FixedTimespan {  // 2010-03-27T20:00:00.000 UTC
            offset: 25200,  // UTC offset 21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+07"),
        }),
        (1288468800, FixedTimespan {  // 2010-10-30T20:00:00.000 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+06"),
        }),
        (1301169600, FixedTimespan {  // 2011-03-26T20:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (1414263600, FixedTimespan {  // 2014-10-25T19:00:00.000 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+06"),
        }),
        (1459022400, FixedTimespan {  // 2016-03-26T20:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
    ]},
};


