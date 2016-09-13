
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Asia/Vladivostok",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 31651,  // UTC offset 31651, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1487321251, FixedTimespan {  // 1922-11-14T15:12:29.000 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (-1247562000, FixedTimespan {  // 1930-06-20T15:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (354895200, FixedTimespan {  // 1981-03-31T14:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (370702800, FixedTimespan {  // 1981-09-30T13:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (386431200, FixedTimespan {  // 1982-03-31T14:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (402238800, FixedTimespan {  // 1982-09-30T13:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (417967200, FixedTimespan {  // 1983-03-31T14:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (433774800, FixedTimespan {  // 1983-09-30T13:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (449589600, FixedTimespan {  // 1984-03-31T14:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (465321600, FixedTimespan {  // 1984-09-29T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (481046400, FixedTimespan {  // 1985-03-30T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (496771200, FixedTimespan {  // 1985-09-28T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (512496000, FixedTimespan {  // 1986-03-29T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (528220800, FixedTimespan {  // 1986-09-27T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (543945600, FixedTimespan {  // 1987-03-28T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (559670400, FixedTimespan {  // 1987-09-26T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (575395200, FixedTimespan {  // 1988-03-26T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (591120000, FixedTimespan {  // 1988-09-24T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (606844800, FixedTimespan {  // 1989-03-25T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (622569600, FixedTimespan {  // 1989-09-23T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (638294400, FixedTimespan {  // 1990-03-24T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (654624000, FixedTimespan {  // 1990-09-29T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (670348800, FixedTimespan {  // 1991-03-30T16:00:00.000 UTC
            offset: 36000,  // UTC offset 32400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (686077200, FixedTimespan {  // 1991-09-28T17:00:00.000 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (695754000, FixedTimespan {  // 1992-01-18T17:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (701798400, FixedTimespan {  // 1992-03-28T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (717523200, FixedTimespan {  // 1992-09-26T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (733248000, FixedTimespan {  // 1993-03-27T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (748972800, FixedTimespan {  // 1993-09-25T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (764697600, FixedTimespan {  // 1994-03-26T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (780422400, FixedTimespan {  // 1994-09-24T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (796147200, FixedTimespan {  // 1995-03-25T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (811872000, FixedTimespan {  // 1995-09-23T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (828201600, FixedTimespan {  // 1996-03-30T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (846345600, FixedTimespan {  // 1996-10-26T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (859651200, FixedTimespan {  // 1997-03-29T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (877795200, FixedTimespan {  // 1997-10-25T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (891100800, FixedTimespan {  // 1998-03-28T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (909244800, FixedTimespan {  // 1998-10-24T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (922550400, FixedTimespan {  // 1999-03-27T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (941299200, FixedTimespan {  // 1999-10-30T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (954000000, FixedTimespan {  // 2000-03-25T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (972748800, FixedTimespan {  // 2000-10-28T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (985449600, FixedTimespan {  // 2001-03-24T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (1004198400, FixedTimespan {  // 2001-10-27T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (1017504000, FixedTimespan {  // 2002-03-30T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (1035648000, FixedTimespan {  // 2002-10-26T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (1048953600, FixedTimespan {  // 2003-03-29T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (1067097600, FixedTimespan {  // 2003-10-25T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (1080403200, FixedTimespan {  // 2004-03-27T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (1099152000, FixedTimespan {  // 2004-10-30T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (1111852800, FixedTimespan {  // 2005-03-26T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (1130601600, FixedTimespan {  // 2005-10-29T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (1143302400, FixedTimespan {  // 2006-03-25T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (1162051200, FixedTimespan {  // 2006-10-28T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (1174752000, FixedTimespan {  // 2007-03-24T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (1193500800, FixedTimespan {  // 2007-10-27T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (1206806400, FixedTimespan {  // 2008-03-29T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (1224950400, FixedTimespan {  // 2008-10-25T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (1238256000, FixedTimespan {  // 2009-03-28T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (1256400000, FixedTimespan {  // 2009-10-24T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (1269705600, FixedTimespan {  // 2010-03-27T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("VLAST"),
        }),
        (1288454400, FixedTimespan {  // 2010-10-30T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (1301155200, FixedTimespan {  // 2011-03-26T16:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
        (1414249200, FixedTimespan {  // 2014-10-25T15:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("VLAT"),
        }),
    ]},
};


