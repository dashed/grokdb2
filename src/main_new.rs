#[macro_use]
extern crate mime;
#[macro_use]
extern crate horrorshow;
extern crate hyper;
extern crate time;
#[macro_use]
extern crate chomp;
#[macro_use]
extern crate lazy_static;
extern crate url;
extern crate conduit_mime_types as mime_types;

/* rust lib imports */

use std::io;
use std::io::{Write, Read};
use std::thread;
use std::ascii::{AsciiExt};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex, LockResult, MutexGuard, RwLock};
use std::panic::{self, AssertUnwindSafe};
use std::fs::{self, File};
use std::path::{PathBuf, Path};
use std::ffi::OsStr;
use std::collections::HashMap;

/* 3rd-party imports */

use url::percent_encoding::{percent_decode};

use horrorshow::{RenderOnce, TemplateBuffer, Template, FnRenderer};

use hyper::method::Method;
use hyper::server::{Server, Handler, Request, Response};
use hyper::header::{Headers, ContentType, TransferEncoding};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::uri::RequestUri;
use hyper::status::StatusCode;
use hyper::header::{Header, HeaderFormat};

use chomp::{SimpleResult, Error, ParseResult};
use chomp::primitives::{InputBuffer};
use chomp::{Input, U8Result, parse_only};
use chomp::buffer::{Source, Stream, StreamError};

use chomp::{token};
use chomp::parsers::{string, eof, any, satisfy};
use chomp::combinators::{or, many_till, many, many1, skip_many, skip_many1, look_ahead, option};
use chomp::ascii::{is_whitespace, decimal, digit};

/* local imports */


////////////////////////////////////////////////////////////////////////////////

lazy_static! {
    static ref MIME_TYPES: mime_types::Types = mime_types::Types::new().unwrap();
}

/* Types */

// TODO: change to u64? it's i64 b/c sqlite requires it.
pub type DeckID = i64;
pub type CardID = i64;
pub type StashID = i64;

// pagination
// TODO: need compile-time check to ensure >= 1 constraint (rust doesn't support this yet)
pub type Page = u64;
pub type PerPage = u64;

#[derive(Debug)]
pub enum Search {
    NoQuery,
    Query(String)
}

#[derive(Debug)]
pub enum SortOrder {
    Ascending,
    Descending
}

#[derive(Debug)]
pub enum DecksPageQuery {
    NoQuery,
    Query(Page, DecksPageSort)
}

#[derive(Debug)]
pub enum DecksPageSort {

    DeckTitle(SortOrder),
    CreatedAt(SortOrder),
    UpdatedAt(SortOrder),

    // TODO: number of cards
    // TODO: number of decks

    // last time user reviewed this deck;
    // not based on the cards the deck contains
    // ReviewedAt(SortOrder)
}

/* database */

type Database = Arc<RwLock<Mutex<i32>>>;

/* App API */

struct AppAPI {
    database: Database
}

/* router */

#[derive(Debug)]
pub enum AppRoute {

    Home,

    // user settings
    Settings,

    Stashes,

    Deck(DeckID, DeckRoute),

    Card(CardID, CardRoute),
    CardInDeck(DeckID, CardID, CardRoute)
}

#[derive(Debug)]
pub enum CardRoute {
    Profile,
    // Settings,
    // Meta,

    Review
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
enum RenderResponse {

    RenderComponent(AppRoute),
    RenderJSON(String),

    RenderNotFound,
    RenderBadRequest,
    RenderInternalServerError,

    RenderAsset(ContentType, Vec<u8>)
}


#[inline]
fn parse_request_uri<'a>(input: Input<'a, u8>, request: Rc<RefCell<Request>>)
    -> U8Result<'a, RenderResponse> {
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
 */
/*
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

type QueryString = HashMap<String, Option<String>>;
enum QueryStringKeyType {
    Value(String),
    NoValue(String)
}

#[inline]
fn parse_query_string(input: Input<u8>) -> U8Result<QueryString> {

    let mut result = parse!{input;
        token(b'?');
        skip_many(|i| token(i, b'&'));
        ret {()}
    };

    let mut query_string: QueryString = HashMap::new();
    let mut should_break = false;

    loop {

        result = result.then(|i| parse!{i;

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
        }).bind(|i, key_type| {
            match key_type {
                QueryStringKeyType::NoValue(key) => {
                    query_string.insert(key, None);

                    i.ret(())
                },
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
                        skip_many(i, |i| token(i, b'&'));

                        ret {
                            query_string.insert(key, Some(value));
                            ()
                        }
                    }


                }
            }
        });

        if should_break {
            break;
        }
    }

    return result.bind(|i, _| i.ret(query_string));
}

#[test]
fn parse_query_string_test() {
    // TODO: complete
}

/* segment parser */

// parse to string till stop_at parser is satisfied. input satisfying stop_at wont be consumed.
#[inline]
fn string_till<'a, F>(input: Input<'a, u8>, mut stop_at: F) -> U8Result<'a, String>
    where F: FnMut(Input<'a, u8>) -> U8Result<'a, ()>  {

    many_till(input, any, |i| look_ahead(i, &mut stop_at))
        .bind(|i, line: Vec<u8>| {
            let string: String = String::from_utf8_lossy(line.as_slice()).into_owned();
            i.ret(string)
        })

}

// TODO: remove
// enum Segment {
//     Empty,
//     NonEmpty(Vec<u8>),
//     NonEmptyEof(Vec<u8>)
// }

// fn parse_segment_before_string(i: Input<u8>, ends_with: String) -> U8Result<Segment> {

//     or(i,
//         |i| parse!{i;
//             eof();
//             ret Segment::Empty
//         },
//         |i| parse!{i;

//             let line: Vec<u8> = many_till(any, |i| string_ignore_case(i, ends_with.as_bytes()));

//             ret Segment::NonEmpty(line)
//         }
//     )

// }

// fn parse_segment_before_eof(i: Input<u8>) -> U8Result<Segment> {

//     or(i,
//         |i| parse!{i;
//             eof();
//             ret Segment::Empty
//         },
//         |i| parse!{i;

//             let line: Vec<u8> = many_till(any, eof);

//             ret Segment::NonEmpty(line)
//         }
//     )

// }

/* misc parsers */

// if parser sucessfully consumes, then return value instead of whatever parser returns
#[inline]
fn parse_then_value<'a, I, T, E, F, U: 'a>(i: Input<'a, I>, mut parser: F, value: T) -> ParseResult<'a, I, T, E>
    where F: FnMut(Input<'a, I>) -> ParseResult<'a, I, U, E>
{
    parse!{i;
        parser();
        ret value
    }
}

#[inline]
fn string_ignore_case<'a>(i: Input<'a, u8>, s: &[u8])
    -> SimpleResult<'a, u8, &'a [u8]> {
    let b = i.buffer();

    if s.len() > b.len() {
        return i.incomplete(s.len() - b.len());
    }

    let d = &b[..s.len()];

    for j in 0..s.len() {

        if !(s[j]).eq_ignore_ascii_case(&(d[j])) {
            return i.replace(&b[j..]).err(Error::expected(d[j]))
        }
    }

    i.replace(&b[s.len()..]).ret(d)
}


#[inline]
fn parse_byte_limit(input: Input<u8>, delim: u8, max_reoccurance: u8) -> U8Result<()> {

    let mut result = parse!{input;
        token(delim);
        ret {()}
    };

    let mut idx = 0;

    let not_delim = {
        if delim == b'-' {
            b'/'
        } else {
            b'-'
        }
    };

    loop {

        if idx >= max_reoccurance {
            break;
        }

        result = result.then(|i| {
            parse!{i;
                let result = option(|i| token(i, delim), not_delim);
                ret {

                    // early bail
                    if result == not_delim {
                        idx = max_reoccurance;
                    }

                    ()
                }
            }
        });

        idx = idx + 1;
    }

    return result;
}

/* misc routes */

fn route_internal_server_error(request: Request, mut response: Response) {

    // let mut context = context;

    let message = format!("Internal server error for {}", request.uri);

    // 500 status code
    *response.status_mut() = StatusCode::InternalServerError;

    response.send(message.as_bytes()).unwrap();
}

/* rendering */

#[inline]
fn render_response(render: RenderResponse, mut response: Response) {

    match render {
        RenderResponse::RenderComponent(app_route) => {
            render_components(app_route, response);
        },
        RenderResponse::RenderNotFound => {

            // let ref url = request.borrow().uri;

            // let message = format!("No route handler found for {}", url);
            let message = format!("Not Found 404");

            // 404 status code
            *response.status_mut() = StatusCode::NotFound;

            response.send(message.as_bytes()).unwrap();

        },
        RenderResponse::RenderAsset(header, content) => {
            response.headers_mut().set((header));
            response.send(&content).unwrap();
        },
        _ => {
            panic!("fix me");
        }
    }


}

#[inline]
fn render_components(app_route: AppRoute, mut response: Response) {

    let app_component = {
        FnRenderer::new(|tmpl| {
            AppComponent(tmpl, app_route);
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
    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        app_component.into_string()
    }));


    if result.is_err() {

        println!("TEMPLATE RENDERING PANIC: {:?}", result.err().unwrap());

        // TODO: fix... route_internal_server_error
        // super::routes::internal_server_error(request, response);

        return;
    }

    match result.ok().unwrap() {
        Err(why) => {
            println!("ERROR RENDERING: {:?}", why);

            // TODO: fix
            // super::routes::internal_server_error(request, response);

            return;
        },
        Ok(rendered) => {

            response.headers_mut().set((ContentType(
                mime!(Text/Html)
            )));

            response.send(rendered.as_bytes()).unwrap();

            return;
        }
    };

}

/* components */

#[inline]
pub fn AppComponent(tmpl: &mut TemplateBuffer, app_route: AppRoute) {

    tmpl << html! {
        : raw!("<!DOCTYPE html>");
        html {
            head {
                title {
                    : "title"
                }
                link (
                    rel="stylesheet",
                    href="/assets/bulma.css"
                );

                |tmpl| {
                    tmpl << html!{
                        style {

                            // sticky footer
                            // source: https://philipwalton.github.io/solved-by-flexbox/demos/sticky-footer/

                            : raw!("\
                                body {\
                                    display: flex;\
                                    min-height: 100vh;\
                                    flex-direction: column;\
                                }\
                            ");
                            : raw!("\
                                #grokdb {\
                                    flex: 1;\
                                }\
                            ");

                            // custom styles
                            // TODO: merge back into bulma css

                            : raw!("\
                                .is-side-paddingless {\
                                    padding-left: 0;\
                                    padding-right: 0;\
                                }\
                            ");

                            : raw!("\
                                .is-bold{\
                                    font-weight: bold;\
                                }\
                            ")
                        }
                    }
                }

                // TODO:  custom stylesheet for specific views
            }
            body {

                div(id="grokdb") {
                    div(class="container", style="max-width:960px;margin-top:10px;margin-bottom:10px;") {
                        nav(class="nav") {
                            div(class="nav-left") {
                                a(class="nav-item", href="#") {
                                    h1(class="title", style="font-weight:bold;") {
                                        : raw!("grokdb")
                                    }

                                }
                            }
                            span(class="nav-toggle") {
                                span {}
                                span {}
                                span {}
                            }
                            div(class="nav-right nav-menu") {
                                a(class="nav-item is-bold", href="#") {
                                    : raw!("Decks")
                                }
                                a(class="nav-item is-bold", href="#") {
                                    : raw!("Stashes")
                                }
                                a(class="nav-item is-bold", href="#") {
                                    : raw!("Preferences")
                                }
                            }
                        }
                    }

                    div(class="container", style="max-width:960px;") {
                        div(class="columns") {

                            div(class="column") {

                                div(class="columns") {
                                    div(class="column") {
                                        span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                            : raw!("/ ");
                                        }
                                        span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                            a(href="#") {
                                                : raw!("Library");
                                            }
                                        }
                                        span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                            : raw!(" / ");
                                        }
                                        span(class="title is-5 is-marginless is-bold") {
                                            a(href="#") {
                                                : raw!("Math");
                                            }
                                        }
                                    }
                                }

                                div(class="columns") {
                                    div(class="column") {
                                        a(class="button is-bold is-success") {
                                            : raw!("New Deck")
                                        }
                                    }
                                }

                                // TODO: hide
                                div(class="columns") {
                                    div(class="column") {
                                        |tmpl| PaginationComponent(tmpl);
                                    }
                                }

                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, true);

                                // TODO: hide
                                div(class="columns", style="margin-top:10px;") {
                                    div(class="column") {
                                        |tmpl| PaginationComponent(tmpl);
                                    }
                                }

                            }

                            div(class="column is-one-quarter") {
                                nav(class="panel") {
                                    p(class="panel-heading", style="font-weight: normal;") {
                                        : raw!("Deck #123")
                                    }
                                    div(class="panel-block") {

                                        aside(class="menu") {
                                            // p(class="menu-label is-bold") {
                                            //     : raw!("Deck #123")
                                            // }
                                            ul(class="menu-list") {
                                                li {
                                                    a(href="#", class="is-bold") {
                                                        : "Description"
                                                    }
                                                }
                                                li {
                                                    a(href="#", class="is-active is-bold") {
                                                        : "Decks"
                                                    }
                                                }
                                                li {
                                                    a(href="#", class="is-bold") {
                                                        : "Cards"
                                                    }
                                                }
                                                li {
                                                    a(href="#", class="is-bold") {
                                                        : "Stats"
                                                    }
                                                }
                                                li {
                                                    a(href="#", class="is-bold") {
                                                        : "Settings"
                                                    }
                                                }
                                            }
                                        }

                                    }
                                }


                            }
                        }


                    }
                }

                footer(class="footer container", style="max-width:960px;padding-bottom:20px;") {
                    div(class="content has-text-centered") {
                        p {
                            : raw!("grokdb v");
                            : env!("CARGO_PKG_VERSION");

                            // TODO: use build.rs to generate:
                            // - date/time of compiled binary
                            // - server hostname
                        }
                    }
                }

                // TODO: uncomment
                // |tmpl| {
                //     match context.view_route {
                //         AppRoute::Deck(_, DeckRoute::Review) =>  {
                //             tmpl << html! {
                //                 script(type="text/javascript", src="https://cdnjs.cloudflare.com/ajax/libs/babel-polyfill/6.9.1/polyfill.min.js") {}
                //                 script(type="text/javascript", src="https://cdnjs.cloudflare.com/ajax/libs/react/15.1.0/react.js") {}
                //                 script(type="text/javascript", src="https://cdnjs.cloudflare.com/ajax/libs/react/15.1.0/react-dom.js") {}

                //                 // script(type="text/javascript", src="/assets/vendor.js") {}
                //                 script(type="text/javascript", src="/assets/deck_review.js") {}
                //             };

                //         },
                //         AppRoute::Deck(_, DeckRoute::NewDeck) =>  {
                //             tmpl << html! {
                //                 script(type="text/javascript", src="https://cdnjs.cloudflare.com/ajax/libs/babel-polyfill/6.9.1/polyfill.min.js") {}
                //                 script(type="text/javascript", src="https://cdnjs.cloudflare.com/ajax/libs/react/15.1.0/react.js") {}
                //                 script(type="text/javascript", src="https://cdnjs.cloudflare.com/ajax/libs/react/15.1.0/react-dom.js") {}

                //                 // script(type="text/javascript", src="/assets/vendor.js") {}
                //                 script(type="text/javascript") {
                //                     // needs to be raw b/c of html escaping
                //                     : raw!(format!("window.__PRE_RENDER_STATE__ = {};",
                //                         view_route_to_pre_render_state(context.view_route.clone(), context)))
                //                 }
                //                 script(type="text/javascript", src="/assets/new_deck.js") {}
                //             };

                //         },
                //         _ => {}
                //     };
                // }

            }
        }

    };
}

#[inline]
fn DeckListItemComponent(tmpl: &mut TemplateBuffer, is_bottom: bool) {
    tmpl << html!{
        div(class="columns is-marginless",
            style=labels!(
                "border-bottom:1px dotted #d3d6db;" => !is_bottom)) {
            div(class="column is-side-paddingless") {
                h5(class="title is-5 is-marginless is-bold") {
                    a(href="#") {
                        : "What does the fox say?"
                    }
                }
                span(style="font-size:12px;") {
                    : "Deck #123";
                    : raw!(" ");
                    a(href="#") {
                        : raw!("View Cards")
                    }
                }
            }
        }
    }
}

#[inline]
fn PaginationComponent(tmpl: &mut TemplateBuffer) {
    tmpl << html!{
        nav(class="pagination") {
            a(class="button is-bold") {
                : raw!("Previous")
            }
            a(class="button is-bold") {
                : raw!("Next")
            }
            ul {
                li {
                    a(class="button is-bold") {
                        : raw!("1")
                    }
                }
                li {
                    a(class="button is-bold") {
                        : raw!("2")
                    }
                }
                li {
                    a(class="button is-bold") {
                        : raw!("3")
                    }
                }
                li {
                    span(class="is-bold") {
                        : "..."
                    }
                }
                li {
                    a(class="button is-bold") {
                        : raw!("44")
                    }
                }
                li {
                    a(class="button is-bold") {
                        : raw!("45")
                    }
                }
                li {
                    a(class="button is-primary is-bold") {
                        : raw!("46")
                    }
                }
                li {
                    a(class="button is-bold") {
                        : raw!("47")
                    }
                }
                li {
                    a(class="button is-bold") {
                        : raw!("48")
                    }
                }
                li {
                    span(class="is-bold") {
                        : "..."
                    }
                }
                li {
                    a(class="button is-bold") {
                        : raw!("100")
                    }
                }
                li {
                    a(class="button is-bold") {
                        : raw!("101")
                    }
                }
                li {
                    a(class="button is-bold") {
                        : raw!("102")
                    }
                }
            }
        }
    }
}

/* LogEntry */

// Shamelessly taken from:
// https://github.com/tomaka/rouille/blob/68b9c65886f8aa75107f0ce3423a790f95ab675a/src/log.rs

/// RAII guard that ensures that a log entry corresponding to a request will be written.
///
/// # Example
///
/// ```no_run
/// rouille::start_server("localhost:80", move |request| {
///     let _entry = rouille::LogEntry::start(std::io::stdout(), request);
///
///     // process the request here
///
/// # panic!()
/// }); // <-- the log entry is written at the end of this block
/// ```
///
pub struct LogEntry<W> where W: Write {
    line: String,
    output: W,
    start_time: u64,
}

impl<'a, W> LogEntry<W> where W: Write {
    /// Starts a `LogEntry`.
    #[inline]
    pub fn start(output: W, rq: &Request) -> LogEntry<W> {
        LogEntry {
            line: format!("{} {}", rq.method, rq.uri),
            output: output,
            start_time: time::precise_time_ns(),
        }
    }
}

impl<W> Drop for LogEntry<W> where W: Write {
    #[inline]
    fn drop(&mut self) {
        write!(self.output, "{} - ", self.line).unwrap();

        if thread::panicking() {
            write!(self.output, " - PANIC!").unwrap();

        } else {
            let elapsed = time::precise_time_ns() - self.start_time;
            format_time(self.output.by_ref(), elapsed);
        }

        writeln!(self.output, "").unwrap();
    }
}

#[inline]
fn format_time<W>(mut out: W, time: u64) where W: Write {
    if time < 1_000 {
        write!(out, "{}ns", time).unwrap()
    } else if time < 1_000_000 {
        write!(out, "{:.1}us", time as f64 / 1_000.0).unwrap()
    } else if time < 1_000_000_000 {
        write!(out, "{:.1}ms", time as f64 / 1_000_000.0).unwrap()
    } else {
        write!(out, "{:.1}s", time as f64 / 1_000_000_000.0).unwrap()
    }
}

////////////////////////////////////////////////////////////////////////////////
// main
////////////////////////////////////////////////////////////////////////////////

fn main() {

    /* server */

    let address = ("0.0.0.0", 3000);

    let mut server = Server::http(address).unwrap();

    // disable keep-alive for now.
    // delegate keep-alive to reverse proxy (e.g. nginx).
    // keep-alive for upstream servers can be an optimization.
    //
    // see: http://serverfault.com/a/426741
    server.keep_alive(None);


    let _guard = server.handle(move |request: Request, response: Response| {

        // logger
        // TODO: logging microservice?
        // NOTE: this is a RAII guard
        let _entry = LogEntry::start(io::stdout(), &request);



        // let local_context = LocalContext::new();
        // let local_context: Rc<RefCell<_>> = Rc::new(RefCell::new(local_context));

        // TODO: fix
        // let mut context = Context {

        //     global_context: &global_context,

        //     api: APIContext::new(&global_context),

        //     // TODO: remove
        //     // request: request,
        //     // response: response,

        //     // used for error handling logging
        //     // log_entry: &_entry,

        //     // router/regexset
        //     uri: &uri,
        //     captures: None,

        //     // default view route
        //     view_route: AppRoute::Home
        // };

        // middleware/logging
        // TODO: complete

        /* middleware/router */

        let uri = format!("{}", request.uri);
        let request: Rc<RefCell<_>> = Rc::new(RefCell::new(request));

        let render = match parse_only(|i| parse_request_uri(i, request.clone()), uri.as_bytes()) {
            Ok(render_response) => {
                // url has a match
                render_response
            },
            Err(_why) => {

                // 404 error

                // panic!("{:?}", e);
                RenderResponse::RenderNotFound
            }
        };

        // NOTE: This is a boundary line where the request has been transformed into
        //       RenderResponse type. Anything from request shall be put into the RenderResponse type.

        let result = render_response(render, response);

        return ();

    });

    let (host, port) = address;
    println!("Listening on http://{}:{}", host, port);

}
