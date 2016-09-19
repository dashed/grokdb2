/* rust lib imports */

use std::ascii::AsciiExt;

/* 3rd-party imports */

use chomp::{SimpleResult, Error, ParseResult};
use chomp::primitives::InputBuffer;
use chomp::{Input, U8Result};

use chomp::token;
use chomp::parsers::{any};
use chomp::combinators::{many_till, look_ahead, option};

/* ////////////////////////////////////////////////////////////////////////// */

// segment parser

// parse to string till stop_at parser is satisfied. input satisfying stop_at wont be consumed.
#[inline]
pub fn string_till<'a, F>(input: Input<'a, u8>, mut stop_at: F) -> U8Result<'a, String>
    where F: FnMut(Input<'a, u8>) -> U8Result<'a, ()>
{

    many_till(input, any, |i| look_ahead(i, &mut stop_at)).bind(|i, line: Vec<u8>| {
        let string: String = String::from_utf8_lossy(line.as_slice()).into_owned();
        i.ret(string)
    })

}

/* misc parsers */

// if parser sucessfully consumes, then return value instead of whatever parser returns
#[inline]
pub fn parse_then_value<'a, I, T, E, F, U: 'a>(i: Input<'a, I>, mut parser: F, value: T) -> ParseResult<'a, I, T, E>
    where F: FnMut(Input<'a, I>) -> ParseResult<'a, I, U, E>
{
    parse!{i;
        parser();
        ret value
    }
}

#[inline]
pub fn string_ignore_case<'a>(i: Input<'a, u8>, s: &[u8]) -> SimpleResult<'a, u8, &'a [u8]> {
    let b = i.buffer();

    if s.len() > b.len() {
        return i.incomplete(s.len() - b.len());
    }

    let d = &b[..s.len()];

    for j in 0..s.len() {

        if !(s[j]).eq_ignore_ascii_case(&(d[j])) {
            return i.replace(&b[j..]).err(Error::expected(d[j]));
        }
    }

    i.replace(&b[s.len()..]).ret(d)
}

// parse delim 1-time, and then optionally max_reoccurance more times.
#[inline]
pub fn parse_byte_limit(input: Input<u8>, delim: u8, max_reoccurance: u8) -> U8Result<()> {

    let mut result = parse!{input;
        token(delim);
        ret {()}
    };

    let mut idx = 0;

    let not_delim = {
        if delim == b'a' { b'b' } else { b'a' }
    };

    loop {

        if idx >= max_reoccurance {
            break;
        }

        let mut looped = false;

        result = result.then(|i| {
            parse!{i;
                let result = option(|i| token(i, delim), not_delim);
                ret {

                    // early bail
                    if result == not_delim {
                        idx = max_reoccurance;
                    }

                    looped = true;

                    ()
                }
            }
        });

        idx = idx + 1;

        if !looped {
            break;
        }
    }

    return result;
}
