
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Atlantic/Stanley",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -7716,  // UTC offset -7716, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2524513884, FixedTimespan {  // 1890-01-01T02:08:36.000 UTC
            offset: -7716,  // UTC offset -7716, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("SMT"),
        }),
        (-1824241884, FixedTimespan {  // 1912-03-12T02:08:36.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (-1018209600, FixedTimespan {  // 1937-09-26T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (-1003093200, FixedTimespan {  // 1938-03-20T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (-986760000, FixedTimespan {  // 1938-09-25T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (-971643600, FixedTimespan {  // 1939-03-19T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (-954705600, FixedTimespan {  // 1939-10-01T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (-939589200, FixedTimespan {  // 1940-03-24T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (-923256000, FixedTimespan {  // 1940-09-29T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (-908139600, FixedTimespan {  // 1941-03-23T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (-891806400, FixedTimespan {  // 1941-09-28T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (-876690000, FixedTimespan {  // 1942-03-22T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (-860356800, FixedTimespan {  // 1942-09-27T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (-852066000, FixedTimespan {  // 1943-01-01T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (420609600, FixedTimespan {  // 1983-05-01T04:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (433306800, FixedTimespan {  // 1983-09-25T03:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (452052000, FixedTimespan {  // 1984-04-29T02:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (464151600, FixedTimespan {  // 1984-09-16T03:00:00.000 UTC
            offset: -7200,  // UTC offset -10800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (483501600, FixedTimespan {  // 1985-04-28T02:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (495601200, FixedTimespan {  // 1985-09-15T03:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (514350000, FixedTimespan {  // 1986-04-20T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (527054400, FixedTimespan {  // 1986-09-14T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (545799600, FixedTimespan {  // 1987-04-19T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (558504000, FixedTimespan {  // 1987-09-13T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (577249200, FixedTimespan {  // 1988-04-17T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (589953600, FixedTimespan {  // 1988-09-11T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (608698800, FixedTimespan {  // 1989-04-16T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (621403200, FixedTimespan {  // 1989-09-10T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (640753200, FixedTimespan {  // 1990-04-22T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (652852800, FixedTimespan {  // 1990-09-09T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (672202800, FixedTimespan {  // 1991-04-21T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (684907200, FixedTimespan {  // 1991-09-15T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (703652400, FixedTimespan {  // 1992-04-19T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (716356800, FixedTimespan {  // 1992-09-13T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (735102000, FixedTimespan {  // 1993-04-18T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (747806400, FixedTimespan {  // 1993-09-12T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (766551600, FixedTimespan {  // 1994-04-17T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (779256000, FixedTimespan {  // 1994-09-11T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (798001200, FixedTimespan {  // 1995-04-16T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (810705600, FixedTimespan {  // 1995-09-10T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (830055600, FixedTimespan {  // 1996-04-21T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (842760000, FixedTimespan {  // 1996-09-15T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (861505200, FixedTimespan {  // 1997-04-20T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (874209600, FixedTimespan {  // 1997-09-14T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (892954800, FixedTimespan {  // 1998-04-19T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (905659200, FixedTimespan {  // 1998-09-13T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (924404400, FixedTimespan {  // 1999-04-18T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (937108800, FixedTimespan {  // 1999-09-12T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (955854000, FixedTimespan {  // 2000-04-16T03:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (968558400, FixedTimespan {  // 2000-09-10T04:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (987310800, FixedTimespan {  // 2001-04-15T05:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (999410400, FixedTimespan {  // 2001-09-02T06:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (1019365200, FixedTimespan {  // 2002-04-21T05:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (1030860000, FixedTimespan {  // 2002-09-01T06:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (1050814800, FixedTimespan {  // 2003-04-20T05:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (1062914400, FixedTimespan {  // 2003-09-07T06:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (1082264400, FixedTimespan {  // 2004-04-18T05:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (1094364000, FixedTimespan {  // 2004-09-05T06:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (1113714000, FixedTimespan {  // 2005-04-17T05:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (1125813600, FixedTimespan {  // 2005-09-04T06:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (1145163600, FixedTimespan {  // 2006-04-16T05:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (1157263200, FixedTimespan {  // 2006-09-03T06:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (1176613200, FixedTimespan {  // 2007-04-15T05:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (1188712800, FixedTimespan {  // 2007-09-02T06:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (1208667600, FixedTimespan {  // 2008-04-20T05:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (1220767200, FixedTimespan {  // 2008-09-07T06:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (1240117200, FixedTimespan {  // 2009-04-19T05:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (1252216800, FixedTimespan {  // 2009-09-06T06:00:00.000 UTC
            offset: -10800,  // UTC offset -14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FKST"),
        }),
        (1271566800, FixedTimespan {  // 2010-04-18T05:00:00.000 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKT"),
        }),
        (1283666400, FixedTimespan {  // 2010-09-05T06:00:00.000 UTC
            offset: -10800,  // UTC offset -10800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FKST"),
        }),
    ]},
};

