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

/* 3rd-party imports */

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

/* Router */

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

        // Rc<RefCell<T>>

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

        let uri = format!("{}", request.uri);

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

        // middleware/router

        let request: Rc<RefCell<_>> = Rc::new(RefCell::new(request));
        // let response: Rc<RefCell<_>> = Rc::new(RefCell::new(response));

        // let request: Rc<RefCell<_>> = Rc::new(RefCell::new(request));
        // let response: Rc<RefCell<_>> = Rc::new(RefCell::new(response));

        let render = match parse_only(|i| parse_request_uri(i, request.clone()), uri.as_bytes()) {
            Ok(render_response) => {
                // url parsing successfully
                render_response
            },
            Err(_why) => {

                // 404 error

                // panic!("{:?}", e);
                RenderResponse::RenderNotFound
            }
        };

        let result = render_response(render, response);

        // route_not_found(request.clone(), response);

        return ();

    });

    let (host, port) = address;
    println!("Listening on http://{}:{}", host, port);

}
