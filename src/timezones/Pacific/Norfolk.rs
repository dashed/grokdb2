
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Pacific/Norfolk",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 40312,  // UTC offset 40312, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2177493112, FixedTimespan {  // 1900-12-31T12:48:08.000 UTC
            offset: 40320,  // UTC offset 40320, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("NMT"),
        }),
        (-599656320, FixedTimespan {  // 1950-12-31T12:48:00.000 UTC
            offset: 41400,  // UTC offset 41400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("NFT"),
        }),
        (152029800, FixedTimespan {  // 1974-10-26T14:30:00.000 UTC
            offset: 45000,  // UTC offset 41400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("NFST"),
        }),
        (162912600, FixedTimespan {  // 1975-03-01T13:30:00.000 UTC
            offset: 41400,  // UTC offset 41400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("NFT"),
        }),
        (1443882600, FixedTimespan {  // 2015-10-03T14:30:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("NFT"),
        }),
    ]},
};

