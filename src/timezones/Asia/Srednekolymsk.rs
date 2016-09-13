
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Asia/Srednekolymsk",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 36892,  // UTC offset 36892, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1441188892, FixedTimespan {  // 1924-05-01T13:45:08.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (-1247565600, FixedTimespan {  // 1930-06-20T14:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (354891600, FixedTimespan {  // 1981-03-31T13:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (370699200, FixedTimespan {  // 1981-09-30T12:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (386427600, FixedTimespan {  // 1982-03-31T13:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (402235200, FixedTimespan {  // 1982-09-30T12:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (417963600, FixedTimespan {  // 1983-03-31T13:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (433771200, FixedTimespan {  // 1983-09-30T12:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (449586000, FixedTimespan {  // 1984-03-31T13:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (465318000, FixedTimespan {  // 1984-09-29T15:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (481042800, FixedTimespan {  // 1985-03-30T15:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (496767600, FixedTimespan {  // 1985-09-28T15:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (512492400, FixedTimespan {  // 1986-03-29T15:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (528217200, FixedTimespan {  // 1986-09-27T15:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (543942000, FixedTimespan {  // 1987-03-28T15:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (559666800, FixedTimespan {  // 1987-09-26T15:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (575391600, FixedTimespan {  // 1988-03-26T15:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (591116400, FixedTimespan {  // 1988-09-24T15:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (606841200, FixedTimespan {  // 1989-03-25T15:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (622566000, FixedTimespan {  // 1989-09-23T15:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (638290800, FixedTimespan {  // 1990-03-24T15:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (654620400, FixedTimespan {  // 1990-09-29T15:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (670345200, FixedTimespan {  // 1991-03-30T15:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (686073600, FixedTimespan {  // 1991-09-28T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (695750400, FixedTimespan {  // 1992-01-18T16:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (701794800, FixedTimespan {  // 1992-03-28T15:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (717519600, FixedTimespan {  // 1992-09-26T15:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (733244400, FixedTimespan {  // 1993-03-27T15:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (748969200, FixedTimespan {  // 1993-09-25T15:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (764694000, FixedTimespan {  // 1994-03-26T15:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (780418800, FixedTimespan {  // 1994-09-24T15:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (796143600, FixedTimespan {  // 1995-03-25T15:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (811868400, FixedTimespan {  // 1995-09-23T15:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (828198000, FixedTimespan {  // 1996-03-30T15:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (846342000, FixedTimespan {  // 1996-10-26T15:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (859647600, FixedTimespan {  // 1997-03-29T15:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (877791600, FixedTimespan {  // 1997-10-25T15:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (891097200, FixedTimespan {  // 1998-03-28T15:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (909241200, FixedTimespan {  // 1998-10-24T15:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (922546800, FixedTimespan {  // 1999-03-27T15:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (941295600, FixedTimespan {  // 1999-10-30T15:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (953996400, FixedTimespan {  // 2000-03-25T15:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (972745200, FixedTimespan {  // 2000-10-28T15:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (985446000, FixedTimespan {  // 2001-03-24T15:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (1004194800, FixedTimespan {  // 2001-10-27T15:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (1017500400, FixedTimespan {  // 2002-03-30T15:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (1035644400, FixedTimespan {  // 2002-10-26T15:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (1048950000, FixedTimespan {  // 2003-03-29T15:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (1067094000, FixedTimespan {  // 2003-10-25T15:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (1080399600, FixedTimespan {  // 2004-03-27T15:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (1099148400, FixedTimespan {  // 2004-10-30T15:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (1111849200, FixedTimespan {  // 2005-03-26T15:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (1130598000, FixedTimespan {  // 2005-10-29T15:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (1143298800, FixedTimespan {  // 2006-03-25T15:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (1162047600, FixedTimespan {  // 2006-10-28T15:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (1174748400, FixedTimespan {  // 2007-03-24T15:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (1193497200, FixedTimespan {  // 2007-10-27T15:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (1206802800, FixedTimespan {  // 2008-03-29T15:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (1224946800, FixedTimespan {  // 2008-10-25T15:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (1238252400, FixedTimespan {  // 2009-03-28T15:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (1256396400, FixedTimespan {  // 2009-10-24T15:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (1269702000, FixedTimespan {  // 2010-03-27T15:00:00.000 UTC
            offset: 43200,  // UTC offset 39600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MAGST"),
        }),
        (1288450800, FixedTimespan {  // 2010-10-30T15:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (1301151600, FixedTimespan {  // 2011-03-26T15:00:00.000 UTC
            offset: 43200,  // UTC offset 43200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MAGT"),
        }),
        (1414245600, FixedTimespan {  // 2014-10-25T14:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("SRET"),
        }),
    ]},
};


