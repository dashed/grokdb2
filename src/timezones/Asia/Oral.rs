
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Asia/Oral",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 12324,  // UTC offset 12324, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1441164324, FixedTimespan {  // 1924-05-01T20:34:36.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+04"),
        }),
        (-1247544000, FixedTimespan {  // 1930-06-20T20:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+05"),
        }),
        (354913200, FixedTimespan {  // 1981-03-31T19:00:00.000 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+06"),
        }),
        (370720800, FixedTimespan {  // 1981-09-30T18:00:00.000 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+06"),
        }),
        (386445600, FixedTimespan {  // 1982-03-31T18:00:00.000 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+06"),
        }),
        (402256800, FixedTimespan {  // 1982-09-30T18:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+05"),
        }),
        (417985200, FixedTimespan {  // 1983-03-31T19:00:00.000 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+06"),
        }),
        (433792800, FixedTimespan {  // 1983-09-30T18:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+05"),
        }),
        (449607600, FixedTimespan {  // 1984-03-31T19:00:00.000 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+06"),
        }),
        (465339600, FixedTimespan {  // 1984-09-29T21:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+05"),
        }),
        (481064400, FixedTimespan {  // 1985-03-30T21:00:00.000 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+06"),
        }),
        (496789200, FixedTimespan {  // 1985-09-28T21:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+05"),
        }),
        (512514000, FixedTimespan {  // 1986-03-29T21:00:00.000 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+06"),
        }),
        (528238800, FixedTimespan {  // 1986-09-27T21:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+05"),
        }),
        (543963600, FixedTimespan {  // 1987-03-28T21:00:00.000 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+06"),
        }),
        (559688400, FixedTimespan {  // 1987-09-26T21:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+05"),
        }),
        (575413200, FixedTimespan {  // 1988-03-26T21:00:00.000 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+06"),
        }),
        (591138000, FixedTimespan {  // 1988-09-24T21:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+05"),
        }),
        (606862800, FixedTimespan {  // 1989-03-25T21:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+05"),
        }),
        (622591200, FixedTimespan {  // 1989-09-23T22:00:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+04"),
        }),
        (638316000, FixedTimespan {  // 1990-03-24T22:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+05"),
        }),
        (654645600, FixedTimespan {  // 1990-09-29T22:00:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+04"),
        }),
        (670370400, FixedTimespan {  // 1991-03-30T22:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+05"),
        }),
        (686095200, FixedTimespan {  // 1991-09-28T22:00:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+04"),
        }),
        (695772000, FixedTimespan {  // 1992-01-18T22:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+05"),
        }),
        (701816400, FixedTimespan {  // 1992-03-28T21:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+05"),
        }),
        (717544800, FixedTimespan {  // 1992-09-26T22:00:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+04"),
        }),
        (733269600, FixedTimespan {  // 1993-03-27T22:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+05"),
        }),
        (748994400, FixedTimespan {  // 1993-09-25T22:00:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+04"),
        }),
        (764719200, FixedTimespan {  // 1994-03-26T22:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+05"),
        }),
        (780444000, FixedTimespan {  // 1994-09-24T22:00:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+04"),
        }),
        (796168800, FixedTimespan {  // 1995-03-25T22:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+05"),
        }),
        (811893600, FixedTimespan {  // 1995-09-23T22:00:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+04"),
        }),
        (828223200, FixedTimespan {  // 1996-03-30T22:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+05"),
        }),
        (846367200, FixedTimespan {  // 1996-10-26T22:00:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+04"),
        }),
        (859672800, FixedTimespan {  // 1997-03-29T22:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+05"),
        }),
        (877816800, FixedTimespan {  // 1997-10-25T22:00:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+04"),
        }),
        (891122400, FixedTimespan {  // 1998-03-28T22:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+05"),
        }),
        (909266400, FixedTimespan {  // 1998-10-24T22:00:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+04"),
        }),
        (922572000, FixedTimespan {  // 1999-03-27T22:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+05"),
        }),
        (941320800, FixedTimespan {  // 1999-10-30T22:00:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+04"),
        }),
        (954021600, FixedTimespan {  // 2000-03-25T22:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+05"),
        }),
        (972770400, FixedTimespan {  // 2000-10-28T22:00:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+04"),
        }),
        (985471200, FixedTimespan {  // 2001-03-24T22:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+05"),
        }),
        (1004220000, FixedTimespan {  // 2001-10-27T22:00:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+04"),
        }),
        (1017525600, FixedTimespan {  // 2002-03-30T22:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+05"),
        }),
        (1035669600, FixedTimespan {  // 2002-10-26T22:00:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+04"),
        }),
        (1048975200, FixedTimespan {  // 2003-03-29T22:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+05"),
        }),
        (1067119200, FixedTimespan {  // 2003-10-25T22:00:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+04"),
        }),
        (1080424800, FixedTimespan {  // 2004-03-27T22:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("+05"),
        }),
        (1099170000, FixedTimespan {  // 2004-10-30T21:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("+05"),
        }),
    ]},
};


