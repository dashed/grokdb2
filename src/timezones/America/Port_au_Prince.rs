
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "America/Port-au-Prince",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -11440,  // UTC offset -11440, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2524510160, FixedTimespan {  // 1890-01-01T03:10:40.000 UTC
            offset: -11460,  // UTC offset -11460, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PPMT"),
        }),
        (-1670489340, FixedTimespan {  // 1917-01-24T15:11:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (421218000, FixedTimespan {  // 1983-05-08T05:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (436334400, FixedTimespan {  // 1983-10-30T04:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (452062800, FixedTimespan {  // 1984-04-29T05:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (467784000, FixedTimespan {  // 1984-10-28T04:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (483512400, FixedTimespan {  // 1985-04-28T05:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (499233600, FixedTimespan {  // 1985-10-27T04:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (514962000, FixedTimespan {  // 1986-04-27T05:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (530683200, FixedTimespan {  // 1986-10-26T04:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (546411600, FixedTimespan {  // 1987-04-26T05:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (562132800, FixedTimespan {  // 1987-10-25T04:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (576050400, FixedTimespan {  // 1988-04-03T06:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (594194400, FixedTimespan {  // 1988-10-30T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (607500000, FixedTimespan {  // 1989-04-02T06:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (625644000, FixedTimespan {  // 1989-10-29T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (638949600, FixedTimespan {  // 1990-04-01T06:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (657093600, FixedTimespan {  // 1990-10-28T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (671004000, FixedTimespan {  // 1991-04-07T06:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (688543200, FixedTimespan {  // 1991-10-27T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (702453600, FixedTimespan {  // 1992-04-05T06:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (719992800, FixedTimespan {  // 1992-10-25T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (733903200, FixedTimespan {  // 1993-04-04T06:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (752047200, FixedTimespan {  // 1993-10-31T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (765352800, FixedTimespan {  // 1994-04-03T06:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (783496800, FixedTimespan {  // 1994-10-30T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (796802400, FixedTimespan {  // 1995-04-02T06:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (814946400, FixedTimespan {  // 1995-10-29T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (828856800, FixedTimespan {  // 1996-04-07T06:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (846396000, FixedTimespan {  // 1996-10-27T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (860306400, FixedTimespan {  // 1997-04-06T06:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (877845600, FixedTimespan {  // 1997-10-26T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (1112504400, FixedTimespan {  // 2005-04-03T05:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (1130644800, FixedTimespan {  // 2005-10-30T04:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (1143954000, FixedTimespan {  // 2006-04-02T05:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (1162094400, FixedTimespan {  // 2006-10-29T04:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (1331449200, FixedTimespan {  // 2012-03-11T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (1352008800, FixedTimespan {  // 2012-11-04T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (1362898800, FixedTimespan {  // 2013-03-10T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (1383458400, FixedTimespan {  // 2013-11-03T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (1394348400, FixedTimespan {  // 2014-03-09T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (1414908000, FixedTimespan {  // 2014-11-02T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (1425798000, FixedTimespan {  // 2015-03-08T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (1446357600, FixedTimespan {  // 2015-11-01T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
    ]},
};


