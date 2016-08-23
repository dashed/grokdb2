/* rust lib imports */

use std::panic::{self, AssertUnwindSafe};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::{PathBuf, Path};
use std::ffi::OsStr;
use std::rc::Rc;
use std::cell::RefCell;

/* 3rd-party imports */

use url::percent_encoding::percent_decode;

use horrorshow::{RenderOnce, TemplateBuffer, Template, FnRenderer};

use hyper;
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

use serde_json;
use serde;

/* local imports */

use parsers::{parse_then_value, string_till, string_ignore_case, parse_byte_limit};
use types::{DeckID, CardID, DecksPageQuery, Search};
use context::{self, Context};
use components::{AppComponent, view_route_to_link};
use api::decks::{self, CreateDeck, DeckResponse};

/* ////////////////////////////////////////////////////////////////////////// */

/* statics */

lazy_static! {
    static ref MIME_TYPES: mime_types::Types = mime_types::Types::new().unwrap();
}

/* router types */

#[derive(Debug)]
pub enum AppRoute {

    // TODO: remove
    // Home,

    // user settings
    Preferences,

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
    Review,

    // CardProfile(CardID, CardRoute)
    // Create,
    // Read,
    // Update,
    // http://stackoverflow.com/a/26897298/412627
    // http://programmers.stackexchange.com/questions/114156/why-are-there-are-no-put-and-delete-methods-on-html-forms
    // Delete
}

#[derive(Debug)]
pub enum RenderResponse {
    RenderComponent(AppRoute),
    RenderJSON(String),

    RenderNotFound,
    RenderBadRequest,
    RenderInternalServerError,

    StatusCode(StatusCode),

    RenderAsset(ContentType, Vec<u8>),
}

/* route parser */

#[inline]
pub fn parse_request_uri<'a>(input: Input<'a, u8>, context: Rc<RefCell<Context>>, request: Rc<RefCell<Request>>)
-> U8Result<'a, RenderResponse> {
    parse!{input;

        // path must begin with /
        // see: https://en.wikipedia.org/wiki/Uniform_Resource_Locator#Syntax
        parse_byte_limit(b'/', 5);

        // NOTE: order matters
        let render_response =

            // /assets/*
            parse_assets(request.clone()) <|>

            // /api/*
            parse_route_api(context.clone(), request.clone()) <|>

            // /card/*
            // parse_route_cards() <|>

            // /deck/*
            parse_route_deck(context.clone(), request.clone()) <|>

            // /stashes/*
            // parse_route_stashes() <|>

            // /settings
            // parse_route_settings() <|>

            // /
            parse_route_root(context.clone(), request.clone());

            // NOTE: don't put query string parser or eof parser here

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
fn fetch_assets(request: Rc<RefCell<Request>>, path: String) -> RenderResponse {

    // Allow only GET requests

    if request.borrow().method != Method::Get {
        return RenderResponse::StatusCode(StatusCode::MethodNotAllowed);
    }

    // URL decode
    let decoded_req_path = Path::new(&path).iter().map(decode_percents);

    let starts_with = Path::new("./assets/").to_path_buf().canonicalize().unwrap();

    let mut req_path = starts_with.clone();
    req_path.extend(decoded_req_path);
    let req_path: PathBuf = req_path;

    let req_path = match req_path.canonicalize() {
        Err(_) => {
            return RenderResponse::RenderNotFound;
        },
        Ok(req_path) => {

            if !req_path.starts_with(starts_with.as_path()) {
                return RenderResponse::RenderNotFound;
            }

            req_path
        }
    };


    match fs::metadata(&req_path) {
        Ok(metadata) => {

            if !metadata.is_file() {
                return RenderResponse::RenderNotFound;
            }

            // TODO: better way?
            let path_str = format!("{}", &req_path.to_string_lossy());

            // println!("req_path.as_path() = {:?}", req_path.as_path().clone());

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

        },
        Err(e) => {
            return RenderResponse::RenderNotFound;
        },
    }

}

#[inline]
fn parse_assets<'a>(input: Input<'a, u8>, request: Rc<RefCell<Request>>) -> U8Result<'a, RenderResponse> {
    parse!{input;

        string_ignore_case(b"assets");

        parse_byte_limit(b'/', 5);

        // TODO: query string cache bust
        let path = string_till(|i| or(i, |i| parse_then_value(i, |i| token(i, b'?'), ()), eof));

        ret {
            fetch_assets(request, path)
        }
    }
}

#[inline]
fn parse_route_root<'a>(input: Input<'a, u8>, context: Rc<RefCell<Context>>, request: Rc<RefCell<Request>>)
-> U8Result<'a, RenderResponse> {
    parse!{input;

        let result = or(
            |i| parse!{i;

                option(|i| parse_byte_limit(i, b'/', 5), ());

                let query_string = option(|i| parse!{i;
                    let query_string = parse_query_string();

                    ret Some(query_string)
                }, None);

                // TODO: remove
                // or(
                //     |i| parse!{i;
                //         token(b'?');
                //         ret {()}
                //     },
                //     eof
                // );

                ret {

                    let root_deck_id = context.borrow().root_deck_id;

                    route_deck_decks(context, request, root_deck_id, query_string)

                    // TODO: remove

                    // Allow only GET requests

                    // if request.borrow().method != Method::Get {
                    //     RenderResponse::StatusCode(StatusCode::MethodNotAllowed)
                    // } else {

                    //     let deck_route = DeckRoute::Decks(Default::default(), Default::default());

                    //     let default_home = AppRoute::Deck(context.borrow().root_deck_id, deck_route);

                    //     RenderResponse::RenderComponent(default_home)
                    // }
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
fn parse_route_api<'a>(input: Input<'a, u8>, context: Rc<RefCell<Context>>, request: Rc<RefCell<Request>>)
-> U8Result<'a, RenderResponse> {
    parse!{input;

        string_ignore_case(b"api");

        parse_byte_limit(b'/', 5);

        let render_response = parse_route_api_deck(context.clone(), request.clone());

        ret render_response

    }
}

#[inline]
fn __parse_route_api_deck(context: Rc<RefCell<Context>>, request: Rc<RefCell<Request>>, parent_deck_id: DeckID)
-> RenderResponse {

    let mut request = request.borrow_mut();

    if request.method == Method::Post {

        // POST /api/deck/:id => create a new deck within this deck

        let mut buffer = String::new();

        match request.read_to_string(&mut buffer) {
            Ok(_num_bytes_parsed) => {

                let request: CreateDeck = match serde_json::from_str(&buffer) {
                    Ok(request) => request,
                    Err(err) => {
                        // TODO: error logging
                        // println!("{:?}", err);
                        return RenderResponse::RenderBadRequest;
                    }
                };

                let _guard = context::write_lock(context.clone());

                match decks::create_deck(context.clone(), request) {
                    Ok(new_deck) => {

                        // connect new deck as child of parent deck

                        match decks::connect_decks(context.clone(), new_deck.id, parent_deck_id) {
                            Ok(_) => {

                                let deck_route = DeckRoute::Decks(Default::default(), Default::default());
                                let app_route = AppRoute::Deck(new_deck.id, deck_route);

                                let response = DeckResponse {
                                    profile_url: view_route_to_link(context, app_route),
                                    deck: new_deck,
                                    has_parent: true,
                                    parent_id: Some(parent_deck_id)
                                };

                                return respond_json!(response);
                            },
                            Err(_) => {
                                // TODO: error logging
                                return RenderResponse::RenderInternalServerError;
                            }
                        }

                    },
                    Err(_) => {
                        // TODO: error logging
                        return RenderResponse::RenderInternalServerError;
                    }
                }

                // println!("{:?}", request);

                // TODO: change

                return RenderResponse::StatusCode(StatusCode::MethodNotAllowed);

            },
            Err(err) => {
                // invalid utf8 input
                // TODO: error logging
                return RenderResponse::RenderBadRequest;
            }
        }

    }

    return RenderResponse::StatusCode(StatusCode::MethodNotAllowed);
}

#[inline]
fn parse_route_api_deck<'a>(input: Input<'a, u8>, context: Rc<RefCell<Context>>, request: Rc<RefCell<Request>>)
-> U8Result<'a, RenderResponse> {
    parse!{input;

        string_ignore_case(b"deck");

        parse_byte_limit(b'/', 5);

        let deck_id: DeckID = decimal();

        ret {

            let _guard = context::write_lock(context.clone());

            match decks::deck_exists(context.clone(), deck_id) {
                Ok(exists) => {

                    if exists {
                        __parse_route_api_deck(context, request, deck_id)
                    } else {
                        RenderResponse::RenderBadRequest
                    }

                },
                // TODO: internal error logging
                Err(_) => RenderResponse::RenderInternalServerError
            }

        }

    }
}

#[inline]
fn parse_route_deck<'a>(input: Input<'a, u8>, context: Rc<RefCell<Context>>, request: Rc<RefCell<Request>>)
-> U8Result<'a, RenderResponse> {
    parse!{input;

        string_ignore_case(b"deck");
        parse_byte_limit(b'/', 5);
        let deck_id = decimal();
        parse_byte_limit(b'/', 5);

        let render_response = parse_route_deck_new_deck(context.clone(), request.clone(), deck_id) <|>
            parse_route_deck_decks(context.clone(), request.clone(), deck_id);

        ret render_response
    }
}

#[inline]
fn route_deck_decks(
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID,
    query_string: Option<QueryString>) -> RenderResponse {

    if request.borrow().method != Method::Get {
        return RenderResponse::StatusCode(StatusCode::MethodNotAllowed);
    }

    let deck_route = match query_string {
        None => DeckRoute::Decks(Default::default(), Default::default()),
        Some(query_string) => {

            let page_query = DecksPageQuery::parse(&query_string, context.clone(), deck_id);
            let search = Search::parse(&query_string);

            DeckRoute::Decks(page_query, search)
        }
    };

    let _guard = context::read_lock(context.clone());

    match decks::deck_exists(context, deck_id) {
        Ok(exists) => {

            if exists {

                // TODO: fix

                let decks = AppRoute::Deck(deck_id, deck_route);
                return RenderResponse::RenderComponent(decks);
            } else {
                return RenderResponse::RenderNotFound;
            }
        },
        Err(_) => {
            return RenderResponse::RenderInternalServerError;
        }
    }

}

#[inline]
fn parse_route_deck_decks<'a>(input: Input<'a, u8>, context: Rc<RefCell<Context>>, request: Rc<RefCell<Request>>, deck_id: DeckID)
-> U8Result<'a, RenderResponse> {
    parse!{input;

        string_ignore_case(b"decks");

        let query_string = option(|i| parse!{i;
            let query_string = parse_query_string();

            ret Some(query_string)
        }, None);

        ret {
            route_deck_decks(context, request, deck_id, query_string)
        }
    }
}

#[inline]
fn parse_route_deck_new_deck<'a>(input: Input<'a, u8>, context: Rc<RefCell<Context>>, request: Rc<RefCell<Request>>, deck_id: DeckID)
-> U8Result<'a, RenderResponse> {
    parse!{input;

        string_ignore_case(b"new");
        parse_byte_limit(b'/', 5);
        string_ignore_case(b"deck");

        ret {
            // Allow only GET requests

            if request.borrow().method != Method::Get {
                RenderResponse::StatusCode(StatusCode::MethodNotAllowed)
            } else {
                let route = AppRoute::Deck(deck_id, DeckRoute::NewDeck);
                RenderResponse::RenderComponent(route)
            }
        }
    }
}

/*

Original:
E --> P {or P}
P --> leaf

Expanded:
query_string = leaf rest | leaf
rest = & leaf rest | & leaf

Cases:
?foo=42
?foo=42&bar=9000
?foo=42&&&&&
?empty

query_string :=
    skip_many(&) field_value query_string |
    skip_many(&) field_value

field_value :=
    segment(=) segment(&) |
    segment(=) segment(eof) |
    segment(&) |
    segment(&) |
    segment(eof)

*/

pub type QueryString = HashMap<String, Option<String>>;

// NOTE: this is for parsing
enum QueryStringKeyType {
    Value(String),
    NoValue(String),
}

#[inline]
fn parse_query_string(input: Input<u8>) -> U8Result<QueryString> {

    // TODO: ; delimeter

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

/* rendering */

#[inline]
pub fn render_response(context: Rc<RefCell<Context>>, render: RenderResponse, mut response: Response) {

    match render {
        RenderResponse::RenderComponent(app_route) => {
            render_components(context, app_route, response);
        },
        RenderResponse::RenderJSON(json_response) => {

            *response.status_mut() = StatusCode::Ok;

            response.headers_mut().set((hyper::header::ContentType(
                mime!(Application/Json)
            )));

            response.send(json_response.as_bytes()).unwrap();

        },
        RenderResponse::RenderBadRequest => {
            let message = format!("400 Bad Request");
            *response.status_mut() = StatusCode::BadRequest;
            response.send(message.as_bytes()).unwrap();
        },
        RenderResponse::RenderNotFound => {
            // TODO: better 404 page
            let message = format!("404 Not Found");
            *response.status_mut() = StatusCode::NotFound;
            response.send(message.as_bytes()).unwrap();
        },
        RenderResponse::RenderInternalServerError => {
            // TODO: better 404 page
            let message = format!("500 Internal Server Error");
            *response.status_mut() = StatusCode::NotFound;
            response.send(message.as_bytes()).unwrap();
        },
        RenderResponse::RenderAsset(header, content) => {
            response.headers_mut().set((header));
            response.send(&content).unwrap();
        },
        RenderResponse::StatusCode(status_code) => {
            *response.status_mut() = status_code;
            let message = format!("{}", status_code);
            response.send(message.as_bytes()).unwrap();
        }
    }


}

#[inline]
fn render_components(context: Rc<RefCell<Context>>, app_route: AppRoute, mut response: Response) {

    assert!(!context.borrow().is_read_locked());
    assert!(!context.borrow().is_write_locked());

    let app_component = {
        FnRenderer::new(|tmpl| {

            // NOTE: only API reads are allowed
            let _guard = context::read_lock(context.clone());

            AppComponent(tmpl, context.clone(), &app_route);
        })
    };

    // Panic capture semantics:
    // - horroshow-rs does not provide a convenient way to abort template rendering.
    // - Template abortion can only be done via panic!(...) macro. (the convention)
    // - If panic!(...) macro is only used for panics, then AssertUnwindSafe can be safely used.
    // - Cargo.toml must be configured to enforce panic strategy is 'unwind' for it to be guaranteed
    //   to be caught by the panic::catch_unwind function.
    //
    // see: https://github.com/rust-lang/rfcs/blob/master/text/1513-less-unwinding.md
    // see: http://doc.crates.io/manifest.html
    let result = panic::catch_unwind(AssertUnwindSafe(|| app_component.into_string()));

    if result.is_err() {

        // TODO: internal error logging

        let why = result.err().unwrap();

        let reason: String = if let Some(why) = why.downcast_ref::<String>() {
            format!("{}", why)
        } else if let Some(why) = why.downcast_ref::<&str>() {
            format!("{}", why)
        } else {
            format!("{:?}", why)
        };

        println!("TEMPLATE RENDERING PANIC: {}", reason);

        render_response(context.clone(), RenderResponse::RenderInternalServerError, response);
        return;
    }

    match result.ok().unwrap() {
        Err(why) => {
            println!("ERROR RENDERING: {:?}", why);

            // TODO: internal error logging

            render_response(context.clone(), RenderResponse::RenderInternalServerError, response);
            return;
        }
        Ok(rendered) => {

            *response.status_mut() = StatusCode::Ok;

            response.headers_mut().set((ContentType(mime!(Text / Html))));

            response.send(rendered.as_bytes()).unwrap();

            return;
        }
    };

}
