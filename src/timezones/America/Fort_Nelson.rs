
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "America/Fort_Nelson",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -28153,  // UTC offset -28153, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2713882247, FixedTimespan {  // 1884-01-01T07:49:13.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (-1632060000, FixedTimespan {  // 1918-04-14T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (-1615129200, FixedTimespan {  // 1918-10-27T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (-880207200, FixedTimespan {  // 1942-02-09T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PWT"),
        }),
        (-769395600, FixedTimespan {  // 1945-08-14T23:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PPT"),
        }),
        (-765385200, FixedTimespan {  // 1945-09-30T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (-715788000, FixedTimespan {  // 1947-04-27T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (-702486000, FixedTimespan {  // 1947-09-28T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (-684338400, FixedTimespan {  // 1948-04-25T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (-671036400, FixedTimespan {  // 1948-09-26T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (-652888800, FixedTimespan {  // 1949-04-24T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (-639586800, FixedTimespan {  // 1949-09-25T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (-620834400, FixedTimespan {  // 1950-04-30T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (-608137200, FixedTimespan {  // 1950-09-24T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (-589384800, FixedTimespan {  // 1951-04-29T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (-576082800, FixedTimespan {  // 1951-09-30T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (-557935200, FixedTimespan {  // 1952-04-27T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (-544633200, FixedTimespan {  // 1952-09-28T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (-526485600, FixedTimespan {  // 1953-04-26T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (-513183600, FixedTimespan {  // 1953-09-27T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (-495036000, FixedTimespan {  // 1954-04-25T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (-481734000, FixedTimespan {  // 1954-09-26T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (-463586400, FixedTimespan {  // 1955-04-24T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (-450284400, FixedTimespan {  // 1955-09-25T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (-431532000, FixedTimespan {  // 1956-04-29T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (-418230000, FixedTimespan {  // 1956-09-30T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (-400082400, FixedTimespan {  // 1957-04-28T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (-386780400, FixedTimespan {  // 1957-09-29T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (-368632800, FixedTimespan {  // 1958-04-27T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (-355330800, FixedTimespan {  // 1958-09-28T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (-337183200, FixedTimespan {  // 1959-04-26T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (-323881200, FixedTimespan {  // 1959-09-27T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (-305733600, FixedTimespan {  // 1960-04-24T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (-292431600, FixedTimespan {  // 1960-09-25T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (-273679200, FixedTimespan {  // 1961-04-30T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (-260982000, FixedTimespan {  // 1961-09-24T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (-242229600, FixedTimespan {  // 1962-04-29T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (-226508400, FixedTimespan {  // 1962-10-28T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (-210780000, FixedTimespan {  // 1963-04-28T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (-195058800, FixedTimespan {  // 1963-10-27T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (-179330400, FixedTimespan {  // 1964-04-26T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (-163609200, FixedTimespan {  // 1964-10-25T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (-147880800, FixedTimespan {  // 1965-04-25T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (-131554800, FixedTimespan {  // 1965-10-31T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (-116431200, FixedTimespan {  // 1966-04-24T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (-100105200, FixedTimespan {  // 1966-10-30T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (-84376800, FixedTimespan {  // 1967-04-30T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (-68655600, FixedTimespan {  // 1967-10-29T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (-52927200, FixedTimespan {  // 1968-04-28T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (-37206000, FixedTimespan {  // 1968-10-27T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (-21477600, FixedTimespan {  // 1969-04-27T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (-5756400, FixedTimespan {  // 1969-10-26T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (9972000, FixedTimespan {  // 1970-04-26T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (25693200, FixedTimespan {  // 1970-10-25T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (41421600, FixedTimespan {  // 1971-04-25T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (57747600, FixedTimespan {  // 1971-10-31T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (73476000, FixedTimespan {  // 1972-04-30T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (89197200, FixedTimespan {  // 1972-10-29T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (104925600, FixedTimespan {  // 1973-04-29T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (120646800, FixedTimespan {  // 1973-10-28T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (136375200, FixedTimespan {  // 1974-04-28T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (152096400, FixedTimespan {  // 1974-10-27T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (167824800, FixedTimespan {  // 1975-04-27T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (183546000, FixedTimespan {  // 1975-10-26T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (199274400, FixedTimespan {  // 1976-04-25T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (215600400, FixedTimespan {  // 1976-10-31T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (230724000, FixedTimespan {  // 1977-04-24T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (247050000, FixedTimespan {  // 1977-10-30T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (262778400, FixedTimespan {  // 1978-04-30T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (278499600, FixedTimespan {  // 1978-10-29T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (294228000, FixedTimespan {  // 1979-04-29T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (309949200, FixedTimespan {  // 1979-10-28T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (325677600, FixedTimespan {  // 1980-04-27T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (341398800, FixedTimespan {  // 1980-10-26T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (357127200, FixedTimespan {  // 1981-04-26T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (372848400, FixedTimespan {  // 1981-10-25T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (388576800, FixedTimespan {  // 1982-04-25T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (404902800, FixedTimespan {  // 1982-10-31T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (420026400, FixedTimespan {  // 1983-04-24T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (436352400, FixedTimespan {  // 1983-10-30T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (452080800, FixedTimespan {  // 1984-04-29T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (467802000, FixedTimespan {  // 1984-10-28T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (483530400, FixedTimespan {  // 1985-04-28T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (499251600, FixedTimespan {  // 1985-10-27T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (514980000, FixedTimespan {  // 1986-04-27T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (530701200, FixedTimespan {  // 1986-10-26T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (544615200, FixedTimespan {  // 1987-04-05T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (562150800, FixedTimespan {  // 1987-10-25T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (576064800, FixedTimespan {  // 1988-04-03T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (594205200, FixedTimespan {  // 1988-10-30T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (607514400, FixedTimespan {  // 1989-04-02T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (625654800, FixedTimespan {  // 1989-10-29T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (638964000, FixedTimespan {  // 1990-04-01T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (657104400, FixedTimespan {  // 1990-10-28T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (671018400, FixedTimespan {  // 1991-04-07T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (688554000, FixedTimespan {  // 1991-10-27T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (702468000, FixedTimespan {  // 1992-04-05T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (720003600, FixedTimespan {  // 1992-10-25T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (733917600, FixedTimespan {  // 1993-04-04T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (752058000, FixedTimespan {  // 1993-10-31T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (765367200, FixedTimespan {  // 1994-04-03T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (783507600, FixedTimespan {  // 1994-10-30T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (796816800, FixedTimespan {  // 1995-04-02T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (814957200, FixedTimespan {  // 1995-10-29T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (828871200, FixedTimespan {  // 1996-04-07T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (846406800, FixedTimespan {  // 1996-10-27T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (860320800, FixedTimespan {  // 1997-04-06T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (877856400, FixedTimespan {  // 1997-10-26T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (891770400, FixedTimespan {  // 1998-04-05T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (909306000, FixedTimespan {  // 1998-10-25T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (923220000, FixedTimespan {  // 1999-04-04T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (941360400, FixedTimespan {  // 1999-10-31T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (954669600, FixedTimespan {  // 2000-04-02T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (972810000, FixedTimespan {  // 2000-10-29T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (986119200, FixedTimespan {  // 2001-04-01T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (1004259600, FixedTimespan {  // 2001-10-28T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (1018173600, FixedTimespan {  // 2002-04-07T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (1035709200, FixedTimespan {  // 2002-10-27T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (1049623200, FixedTimespan {  // 2003-04-06T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (1067158800, FixedTimespan {  // 2003-10-26T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (1081072800, FixedTimespan {  // 2004-04-04T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (1099213200, FixedTimespan {  // 2004-10-31T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (1112522400, FixedTimespan {  // 2005-04-03T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (1130662800, FixedTimespan {  // 2005-10-30T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (1143972000, FixedTimespan {  // 2006-04-02T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (1162112400, FixedTimespan {  // 2006-10-29T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (1173607200, FixedTimespan {  // 2007-03-11T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (1194166800, FixedTimespan {  // 2007-11-04T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (1205056800, FixedTimespan {  // 2008-03-09T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (1225616400, FixedTimespan {  // 2008-11-02T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (1236506400, FixedTimespan {  // 2009-03-08T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (1257066000, FixedTimespan {  // 2009-11-01T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (1268560800, FixedTimespan {  // 2010-03-14T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (1289120400, FixedTimespan {  // 2010-11-07T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (1300010400, FixedTimespan {  // 2011-03-13T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (1320570000, FixedTimespan {  // 2011-11-06T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (1331460000, FixedTimespan {  // 2012-03-11T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (1352019600, FixedTimespan {  // 2012-11-04T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (1362909600, FixedTimespan {  // 2013-03-10T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (1383469200, FixedTimespan {  // 2013-11-03T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (1394359200, FixedTimespan {  // 2014-03-09T10:00:00.000 UTC
            offset: -25200,  // UTC offset -28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("PDT"),
        }),
        (1414918800, FixedTimespan {  // 2014-11-02T09:00:00.000 UTC
            offset: -28800,  // UTC offset -28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("PST"),
        }),
        (1425808800, FixedTimespan {  // 2015-03-08T10:00:00.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MST"),
        }),
    ]},
};


