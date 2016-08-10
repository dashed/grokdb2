/* rust lib imports */

use std::panic::{self, AssertUnwindSafe};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::{PathBuf, Path};
use std::ffi::OsStr;
use std::rc::Rc;
use std::sync::{Arc, Mutex, LockResult, MutexGuard, RwLock};
use std::cell::RefCell;

/* 3rd-party imports */

use url::percent_encoding::percent_decode;

use horrorshow::{RenderOnce, TemplateBuffer, Template, FnRenderer};

use hyper::method::Method;
use hyper::server::{Server, Handler, Request, Response};
use hyper::header::{Headers, ContentType, TransferEncoding};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::uri::RequestUri;
use hyper::status::StatusCode;
use hyper::header::{Header, HeaderFormat};

use chomp::{SimpleResult, Error, ParseResult};
use chomp::primitives::InputBuffer;
use chomp::{Input, U8Result, parse_only};
use chomp::buffer::{Source, Stream, StreamError};

use chomp::token;
use chomp::parsers::{string, eof, any, satisfy};
use chomp::combinators::{or, many_till, many, many1, skip_many, skip_many1, look_ahead, option};
use chomp::ascii::{is_whitespace, decimal, digit};

use mime_types;

/* local imports */

use parsers::{parse_then_value, string_till, string_ignore_case, parse_byte_limit};
use types::{DeckID, CardID, DecksPageQuery, Search};

/// /////////////////////////////////////////////////////////////////////////////

/* statics */

lazy_static! {
    static ref MIME_TYPES: mime_types::Types = mime_types::Types::new().unwrap();
}

/* router types */

#[derive(Debug)]
pub enum AppRoute {
    Home,

    // user settings
    Settings,

    Stashes,

    Deck(DeckID, DeckRoute),

    Card(CardID, CardRoute),
    CardInDeck(DeckID, CardID, CardRoute),
}

#[derive(Debug)]
pub enum CardRoute {
    Profile,
    // Settings,
    // Meta,
    Review,
}

#[derive(Debug)]
pub enum DeckRoute {
    NewCard,
    NewDeck,
    Description,
    Decks(DecksPageQuery, Search), // list
    Cards, // list
    Settings,
    Meta,
    Review, /* CardProfile(CardID, CardRoute)
             *
             * Create,
             * Read,
             * Update,
             * http://stackoverflow.com/a/26897298/412627
             * http://programmers.stackexchange.com/questions/114156/why-are-there-are-no-put-and-delete-methods-on-html-forms
             * Delete */
}

#[derive(Debug)]
pub enum RenderResponse {
    RenderComponent(AppRoute),
    RenderJSON(String),

    RenderNotFound,
    RenderBadRequest,
    RenderInternalServerError,

    RenderAsset(ContentType, Vec<u8>),
}

/* route parser */

#[inline]
pub fn parse_request_uri<'a>(input: Input<'a, u8>, request: Rc<RefCell<Request>>) -> U8Result<'a, RenderResponse> {
    parse!{input;

        // path must begin with /
        // see: https://en.wikipedia.org/wiki/Uniform_Resource_Locator#Syntax
        parse_byte_limit(b'/', 5);

        // NOTE: order matters
        let render_response =

            // /assets/*
            parse_assets() <|>

            // /cards
            // parse_route_cards() <|>

            // /decks
            parse_route_decks() <|>

            // /stashes
            // parse_route_stashes() <|>

            // /settings
            // parse_route_settings() <|>

            // /
            parse_route_root();

        // NOTE: Thou shalt not put parsers after this line.
        //       Why?
        //       Allow previous parsers to bail early.
        //       e.g. Not having to parse query strings.

        ret render_response
    }
}

#[inline]
fn decode_percents(string: &OsStr) -> String {

    let string = format!("{}", string.to_string_lossy());

    format!("{}", percent_decode(string.as_bytes()).decode_utf8_lossy())

    // String::from_utf8(.if_any().unwrap()).unwrap()
    // OsStr::new(&token)
}

#[inline]
fn parse_assets(input: Input<u8>) -> U8Result<RenderResponse> {
    parse!{input;

        string_ignore_case(b"assets");

        parse_byte_limit(b'/', 5);

        // TODO: query string cache bust
        let path = string_till(eof);
        eof();

        ret {

            // URL decode
            let decoded_req_path = Path::new(&path).iter().map(decode_percents);

            let mut req_path = Path::new("assets/").to_path_buf();
            req_path.extend(decoded_req_path);
            let req_path: PathBuf = req_path;

            match fs::metadata(&req_path) {
                Ok(metadata) => {

                    if !metadata.is_file() {
                        RenderResponse::RenderNotFound
                    } else {

                        let path_str = format!("{}", &req_path.to_string_lossy());

                        // Set the content type based on the file extension
                        let mime_str = MIME_TYPES.mime_for_path(req_path.as_path());

                        let mut content_type = ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![]));

                        let _ = mime_str.parse().map(|mime: Mime| {
                            content_type = ContentType(mime);
                        });

                        let mut file = File::open(req_path)
                            .ok()
                            .expect(&format!("No such file: {:?}", path_str));

                        let mut content = Vec::new();

                        file.read_to_end(&mut content).unwrap();

                        RenderResponse::RenderAsset(content_type, content)
                    }

                },
                Err(e) => {
                    RenderResponse::RenderNotFound
                },
            }
        }

    }
}

#[inline]
fn parse_route_root(input: Input<u8>) -> U8Result<RenderResponse> {
    parse!{input;

        let result = or(
            |i| parse!{i;

                or(
                    |i| parse!{i;
                        token(b'?');
                        ret {()}
                    },
                    eof
                );

                ret {
                    // TODO: wrong verb... 405 Method Not Allowed

                    let home = AppRoute::Home;

                    RenderResponse::RenderComponent(home)
                }
            },
            |i| parse!{i;

                // null parser

                ret {
                    RenderResponse::RenderNotFound
                }
            }
        );

        ret result
    }
}

#[inline]
fn parse_route_decks(input: Input<u8>) -> U8Result<RenderResponse> {
    parse!{input;

        string_ignore_case(b"decks");

        let query_string = option(|i| parse!{i;
            let query_string = parse_query_string();

            ret Some(query_string)
        }, None);

        ret {

            let home = AppRoute::Home;

            RenderResponse::RenderComponent(home)
        }
    }
}


/* Original:
 * E --> P {or P}
 * P --> leaf
 *
 * Expanded:
 * query_string = leaf rest | leaf
 * rest = & leaf rest | & leaf
 *
 * Cases:
 * ?foo=42
 * ?foo=42&bar=9000
 * ?foo=42&&&&&
 * ?empty
 * */
/* query_string :=
 * skip_many(&) field_value query_string |
 * skip_many(&) field_value
 *
 * field_value :=
 * segment(=) segment(&) |
 * segment(=) segment(eof) |
 * segment(&) |
 * segment(&) |
 * segment(eof)
 * */

type QueryString = HashMap<String, Option<String>>;
enum QueryStringKeyType {
    Value(String),
    NoValue(String),
}

#[inline]
fn parse_query_string(input: Input<u8>) -> U8Result<QueryString> {

    let mut result = parse!{input;
        token(b'?');
        skip_many(|i| token(i, b'&'));
        ret {()}
    };

    let mut should_break = false;
    let mut query_string: QueryString = HashMap::new();

    loop {

        let mut looped = false;

        result = result.then(|i| {
                parse!{i;

            let key = string_till(|i| parse!{i;
                token(b'&') <|> token(b'=') <|> parse_then_value(eof, b'&');
                ret {()}
            });

            let key_type = or(
                |i| parse!{i;
                    token(b'&');
                    skip_many(|i| token(i, b'&'));
                    ret QueryStringKeyType::NoValue(key.to_lowercase().trim().to_owned())
                },
                |i| or(i,
                    |i| parse!{i;
                        token(b'=');
                        ret QueryStringKeyType::Value(key.to_lowercase().trim().to_owned())
                    },
                    |i| parse!{i;
                        eof();
                        ret {
                            should_break = true;
                            QueryStringKeyType::NoValue(key.to_lowercase().trim().to_owned())
                        }
                    }
                )
            );

            ret key_type
        }
            })
            .bind(|i, key_type| {
                match key_type {
                    QueryStringKeyType::NoValue(key) => {

                        if key.len() > 0 {
                            query_string.insert(key, None);
                        }

                        looped = true;

                        i.ret(())
                    }
                    QueryStringKeyType::Value(key) => {
                        parse!{i;

                        let value = string_till(|i| parse!{i;
                            let res = token(b'&') <|> parse_then_value(eof, b'-');
                            ret {
                                if res == b'-' {
                                    should_break = true;
                                }
                                ()
                            }
                        });

                        token(b'&') <|> parse_then_value(eof, b'-');
                        skip_many(|i| token(i, b'&'));

                        ret {

                            if key.len() > 0 {
                                query_string.insert(key, Some(value));
                            }

                            looped = true;

                            ()
                        }
                    }
                    }
                }
            });

        if should_break || !looped {
            break;
        }
    }

    return result.bind(|i, _| i.ret(query_string));
}

#[test]
fn parse_query_string_test() {

    let fails = vec!["", "&", " ", "\t"];

    for input in fails {
        match parse_only(parse_query_string, input.as_bytes()) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true)
        }
    }

    let inputs = vec![
        "?",
        "?&&&"
    ];

    for input in inputs {
        match parse_only(parse_query_string, input.as_bytes()) {
            Ok(actual) => {
                let expected: QueryString = HashMap::new();
                assert_eq!(actual, expected);
            }
            Err(_) => assert!(false)
        }
    }

    let inputs = vec![
        "?page=1",
        "?page=1&&&",
        "?page=3333&page=1",
        "?page=3333&page=1&&&",
        "?page=3333&&&&&page=1",
        "?page=3333&&&&&page=1&&&",
        "?page&page=1"
    ];

    for input in inputs {
        match parse_only(parse_query_string, input.as_bytes()) {
            Ok(actual) => {

                let mut expected: QueryString = HashMap::new();
                expected.insert(format!("page"), Some(format!("1")));

                assert_eq!(actual, expected);
            }
            Err(_) => assert!(false)
        }
    }

    let inputs = vec![
        "?page=1&page&sort=desc",
    ];

    for input in inputs {
        match parse_only(parse_query_string, input.as_bytes()) {
            Ok(actual) => {

                let mut expected: QueryString = HashMap::new();
                expected.insert(format!("page"), None);
                expected.insert(format!("sort"), Some(format!("desc")));

                assert_eq!(actual, expected);
            }
            Err(_) => assert!(false)
        }
    }

}

/* misc routes */

fn route_internal_server_error(request: Request, mut response: Response) {

    // let mut context = context;

    let message = format!("Internal server error for {}", request.uri);

    // 500 status code
    *response.status_mut() = StatusCode::InternalServerError;

    response.send(message.as_bytes()).unwrap();
}
