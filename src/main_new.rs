#[macro_use]
extern crate mime;
#[macro_use]
extern crate horrorshow;
extern crate hyper;
extern crate time;
#[macro_use]
extern crate chomp;

/* rust lib imports */

use std::io;
use std::io::{Write};
use std::thread;
use std::ascii::{AsciiExt};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex, LockResult, MutexGuard, RwLock};
use std::panic::{self, AssertUnwindSafe};

/* 3rd-party imports */

use horrorshow::{RenderOnce, TemplateBuffer, Template, FnRenderer};

use hyper::method::Method;
use hyper::server::{Server, Handler, Request, Response};
use hyper::header::{Headers, ContentType, TransferEncoding};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::uri::RequestUri;
use hyper::status::StatusCode;

use chomp::{SimpleResult, Error, ParseResult};
use chomp::primitives::{InputBuffer};
use chomp::{Input, U8Result, parse_only};
use chomp::buffer::{Source, Stream, StreamError};

use chomp::{token};
use chomp::parsers::{string, eof, any, satisfy};
use chomp::combinators::{or, many_till, many, many1, skip_many, skip_many1, look_ahead};
use chomp::ascii::{is_whitespace, decimal, digit};

/* local imports */


////////////////////////////////////////////////////////////////////////////////

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

#[derive(Debug, Clone)]
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
    RenderInternalServerError
}

fn parse_request_uri<'a>(input: Input<'a, u8>, request: Rc<RefCell<Request>>)
    -> U8Result<'a, RenderResponse> {
    parse!{input;

        // path must begin with /
        // see: https://en.wikipedia.org/wiki/Uniform_Resource_Locator#Syntax
        token(b'/');

        skip_many(|i| token(i, b'/'));

        // NOTE: order matters
        let render_response =

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

fn parse_route_root(input: Input<u8>) -> U8Result<RenderResponse> {
    parse!{input;

        // parse nothing

        // token(b'?');




        ret {

            // TODO: wrong verb... 405 Method Not Allowed

            let home = AppRoute::Home;

            RenderResponse::RenderComponent(home)
        }
    }
}

fn parse_route_decks(input: Input<u8>) -> U8Result<RenderResponse> {
    parse!{input;

        string_ignore_case("decks".as_bytes());

        // TODO: necess?
        // skip_many(token(b'/'));


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
fn parse_query_string(input: Input<u8>) {


}

/* segment parser */

enum Segment {
    Empty,
    NonEmpty(Vec<u8>),
    NonEmptyEof(Vec<u8>)
}

fn parse_segment_before_string(i: Input<u8>, ends_with: String) -> U8Result<Segment> {

    or(i,
        |i| parse!{i;
            eof();
            ret Segment::Empty
        },
        |i| parse!{i;

            let line: Vec<u8> = many_till(any, |i| string_ignore_case(i, ends_with.as_bytes()));

            ret Segment::NonEmpty(line)
        }
    )

}

fn parse_segment_before_eof(i: Input<u8>) -> U8Result<Segment> {

    or(i,
        |i| parse!{i;
            eof();
            ret Segment::Empty
        },
        |i| parse!{i;

            let line: Vec<u8> = many_till(any, eof);

            ret Segment::NonEmpty(line)
        }
    )

}

/* misc parsers */

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

/* misc routes */

fn route_internal_server_error(request: Request, mut response: Response) {

    // let mut context = context;

    let message = format!("Internal server error for {}", request.uri);

    // 500 status code
    *response.status_mut() = StatusCode::InternalServerError;

    response.send(message.as_bytes()).unwrap();
}

fn route_not_found(request: Rc<RefCell<Request>>, mut response: Response) {

    // let mut context = context;

    let ref url = request.borrow().uri;
    let message = format!("No route handler found for {}", url);

    // 404 status code
    *response.status_mut() = StatusCode::NotFound;

    response.send(message.as_bytes()).unwrap();
}

/* rendering */

fn render_response(render: RenderResponse, mut response: Response) {

    match render {
        RenderResponse::RenderComponent(app_route) => {
            render_components(app_route, response);
        },
        _ => {
            panic!("fix me");
        }
    }


}

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

pub fn AppComponent(tmpl: &mut TemplateBuffer, app_route: AppRoute) {

    tmpl << html! {
        : raw!("<!DOCTYPE html>");
        html {
            head {
                title { : "title" }
                link (
                    rel="stylesheet",
                    href="/assets/spectre.min.css"
                );

                // custom stylesheet for specific views
                |tmpl| {

                    // TODO: css minify using build.rs

                    tmpl << html! {
                        style {

                            // TODO: uncomment
                            // custom styles for view
                            // |tmpl| {
                            //     match context.view_route {
                            //         AppRoute::Deck(_, DeckRoute::NewCard) |
                            //         AppRoute::Deck(_, DeckRoute::NewDeck) =>  {
                            //             tmpl << html! {

                            //                 : raw!("\
                            //                     .btn-success {\
                            //                         border-color: #30ae40;\
                            //                         color: #32b643;\
                            //                     }\
                            //                     a.btn-success:hover {\
                            //                         background: #32b643;\
                            //                         border-color: #30ae40;\
                            //                         color: #fff;\
                            //                     }\
                            //                 ");

                            //             };

                            //         },
                            //         _ => {}
                            //     };
                            // }

                            : raw!("\
                                body {\
                                    display: flex;\
                                    min-height: 100vh;\
                                    flex-direction: column;\
                                }\
                                #grokdb {\
                                    flex: 1;\
                                }\
                                ");
                            : raw!("\
                                ul.pagination li {\
                                    margin-top: 0;\
                                }\
                                ");
                            : raw!("\
                                .grokdb-menu {\
                                    box-shadow: none;\
                                    border: .1rem solid #c5c5c5;\
                                }\
                                ");
                            : raw!("\
                                a:hover,\
                                a:active,\
                                .menu .menu-item a:hover,\
                                .menu .menu-item a:active\
                                {\
                                    text-decoration: underline;\
                                }\
                                ");
                            : raw!("\
                                hr {\
                                    height: 1px;\
                                    background-color: #c5c5c5;\
                                    border:none;\
                                }\
                                ");
                        }
                    };


                }
            }
            body {
                section(class="container grid-960", id="grokdb") {
                    header(class="navbar") {
                        section(class="navbar-section") {
                            a(href = "#", class="navbar-brand") {
                                : "grokdb"
                            }
                        }

                        // TODO: uncomment
                        // section(class="navbar-section") {
                        //     a(
                        //         href = view_route_to_link(AppRoute::Home, &context),

                        //         // TODO: fix
                        //         style? = stylenames!("font-weight:bold;" => {
                        //             matches!(context.view_route, AppRoute::Deck(_, _)) ||
                        //             matches!(context.view_route, AppRoute::Home) ||
                        //             matches!(context.view_route, AppRoute::Card(_, _)) ||
                        //             matches!(context.view_route, AppRoute::CardInDeck(_, _, _))
                        //         }
                        //         ),
                        //         class? = classnames!("btn btn-link badge", "active" => {
                        //             matches!(context.view_route, AppRoute::Deck(_, _)) ||
                        //             matches!(context.view_route, AppRoute::Home) ||
                        //             matches!(context.view_route, AppRoute::Card(_, _)) ||
                        //             matches!(context.view_route, AppRoute::CardInDeck(_, _, _))
                        //         }
                        //         ),

                        //         data-badge="9"
                        //     ) {
                        //         : "decks"
                        //     }
                        //     a(
                        //         href = view_route_to_link(AppRoute::Stashes, &context),

                        //         style? = stylenames!("font-weight:bold;" =>
                        //             matches!(context.view_route, AppRoute::Stashes)
                        //         ),
                        //         class? = classnames!("btn btn-link badge", "active" =>
                        //             matches!(context.view_route, AppRoute::Stashes)
                        //         )

                        //     ) {
                        //         : "stashes"
                        //     }
                        //     a(
                        //         href = view_route_to_link(AppRoute::Settings, &context),

                        //         style? = stylenames!("font-weight:bold;" =>
                        //             matches!(context.view_route, AppRoute::Settings)
                        //         ),
                        //         class? = classnames!("btn btn-link badge", "active" =>
                        //             matches!(context.view_route, AppRoute::Settings)
                        //         )
                        //     ) {
                        //         : "settings"
                        //     }
                        //     : " ";
                        //     input(type="text", class="form-input input-inline", placeholder="search");
                        //     : " ";
                        //     a(href="#", class="btn btn-primary") {
                        //         : "login"
                        //     }
                        // }
                    }
                    noscript {
                        div(class="toast toast-danger") {
                            : "grokdb requires JavaScript to function properly. Go to ";
                            a(href="http://enable-javascript.com/", target="_blank", style="color: #000000") {
                                : "http://enable-javascript.com/"
                            }
                            : " to enable JavaScript for your browser.";
                        }
                    }

                    // section {
                    // // section(class="container") {
                    //     // : ViewRouteResolver::new(&context)


                    // TODO: uncomment
                    // |tmpl| {
                    //     match context.view_route.clone() {
                    //         AppRoute::Home => {
                    //             // TODO: fix
                    //             // NOTE: goes to DeckDetailComponent
                    //             unreachable!();
                    //             // tmpl << DeckDetailComponent::new(&context)
                    //         }
                    //         AppRoute::Settings => {
                    //             SettingsComponent(tmpl, &mut context);
                    //         }
                    //         AppRoute::Stashes => {
                    //             StashesComponent(tmpl, &context);
                    //         }
                    //         AppRoute::Deck(_deck_id, ref _deck_route) => {

                    //             DeckDetailComponent(tmpl, context);

                    //             // match deck_route {
                    //             //     &DeckRoute::New => tmpl << NewDeckComponent::new(&context)
                    //             // }

                    //         },
                    //         AppRoute::Card(_card_id, ref _card_route) => {
                    //             CardDetailComponent(tmpl, &mut context);
                    //         },
                    //         AppRoute::CardInDeck(_deck_id, _card_id, ref _card_route) => {
                    //             CardDetailComponent(tmpl, &mut context);
                    //         }
                    //     };
                    // }


                }

                footer(class="container grid-960") {
                    : "footer component"
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
    pub fn start(output: W, rq: &Request) -> LogEntry<W> {
        LogEntry {
            line: format!("{} {}", rq.method, rq.uri),
            output: output,
            start_time: time::precise_time_ns(),
        }
    }
}

impl<W> Drop for LogEntry<W> where W: Write {
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
