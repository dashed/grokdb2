
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Asia/Novokuznetsk",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 20928,  // UTC offset 20928, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1441259328, FixedTimespan {  // 1924-04-30T18:11:12.000 UTC
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
        (811882800, FixedTimespan {  // 1995-09-23T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (828212400, FixedTimespan {  // 1996-03-30T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (846356400, FixedTimespan {  // 1996-10-26T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (859662000, FixedTimespan {  // 1997-03-29T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (877806000, FixedTimespan {  // 1997-10-25T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (891111600, FixedTimespan {  // 1998-03-28T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (909255600, FixedTimespan {  // 1998-10-24T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (922561200, FixedTimespan {  // 1999-03-27T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (941310000, FixedTimespan {  // 1999-10-30T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (954010800, FixedTimespan {  // 2000-03-25T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (972759600, FixedTimespan {  // 2000-10-28T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (985460400, FixedTimespan {  // 2001-03-24T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (1004209200, FixedTimespan {  // 2001-10-27T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (1017514800, FixedTimespan {  // 2002-03-30T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (1035658800, FixedTimespan {  // 2002-10-26T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (1048964400, FixedTimespan {  // 2003-03-29T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (1067108400, FixedTimespan {  // 2003-10-25T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (1080414000, FixedTimespan {  // 2004-03-27T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (1099162800, FixedTimespan {  // 2004-10-30T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (1111863600, FixedTimespan {  // 2005-03-26T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (1130612400, FixedTimespan {  // 2005-10-29T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (1143313200, FixedTimespan {  // 2006-03-25T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (1162062000, FixedTimespan {  // 2006-10-28T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (1174762800, FixedTimespan {  // 2007-03-24T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (1193511600, FixedTimespan {  // 2007-10-27T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (1206817200, FixedTimespan {  // 2008-03-29T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (1224961200, FixedTimespan {  // 2008-10-25T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (1238266800, FixedTimespan {  // 2009-03-28T19:00:00.000 UTC
            offset: 28800,  // UTC offset 25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+08"),
        }),
        (1256410800, FixedTimespan {  // 2009-10-24T19:00:00.000 UTC
            offset: 25200,  // UTC offset 25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+07"),
        }),
        (1269716400, FixedTimespan {  // 2010-03-27T19:00:00.000 UTC
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
    ]},
};


