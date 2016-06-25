#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
// #![deny(warnings)]
#[macro_use]
extern crate horrorshow as templates;
extern crate conduit_mime_types as mime_types;
#[macro_use]
extern crate mime;
extern crate hyper;
extern crate regex;
extern crate url;
extern crate rusqlite;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate matches;
extern crate time;
extern crate serde;
extern crate serde_json;
#[macro_use(quick_error)]
extern crate quick_error;


// TODO: remove; was using it for experiment
// extern crate html5ever;
// extern crate tendril;

/* rust lib imports */

use std::fs;
use std::fs::{File};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{PathBuf, Path};
use std::sync::{Arc, Mutex, LockResult, MutexGuard, RwLock};
use std::io;
use std::io::{Write};
use std::thread;
use std::default::Default;

/* 3rd-party imports */

use url::percent_encoding::percent_decode;

use regex::{Regex, RegexSet};
use regex::{Captures};

use templates::{RenderOnce, TemplateBuffer, Template, FnRenderer};

use hyper::method::Method;
use hyper::server::{Server, Handler, Request, Response};
use hyper::header::{Headers, ContentType, TransferEncoding};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::uri::RequestUri;
use hyper::status::StatusCode;

use rusqlite::{Connection, Error, Result as SqliteResult};

// use html5ever::driver::ParseOpts;
// use html5ever::tree_builder::TreeBuilderOpts;
// use html5ever::{parse_document, serialize};
// use html5ever::rcdom::RcDom;

// use tendril::TendrilSink;


////////////////////////////////////////////////////////////////////////////////

// static PHRASE: &'static [u8] = b"Hello World!";



/**

// TODO: credit, copyright, license, etc
List of projects whose code i've been copying & pasting from:
- https://github.com/iron/iron/pull/291/files
- reroute
- https://github.com/tomaka/rouille

reference: https://github.com/joshbuchea/HEAD#link-elements

**/

/* app errors */
mod errors;

/* global macros */

#[macro_use]
mod global_macros;

/* database */

#[macro_use]
mod database;
mod tables;

/* grokdb api */
mod configs;
mod decks;

/* contexts */

mod contexts;
use contexts::{GlobalContext, Context};

/* route */

#[macro_use]
mod route;
use route::manager::{Router, RouterFn, LinkGenerator};
use route::constants::{AppRoute, DeckRoute, CardRoute};
use route::helpers::{view_route_to_link};

/* components */

mod components;




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

/* grokdb */

fn main() {

    // TODO: clap-rs

    /* database */

    let db_conn = Connection::open("test.db");

    let db_conn = match db_conn {
        Err(why) => {
            panic!("{}", why);
        },
        Ok(db_conn) => {
            db_conn
        }
    };

    let db_connection = Arc::new(RwLock::new(Mutex::new(db_conn)));

    /* table setup */

    tables::setup_database(db_connection.clone());

    /* context setup */

    let global_context = GlobalContext {
        assets_root_path: Path::new("assets/"),
        db_connection: db_connection,
    };

    // preliminary setup.

    {
        // check if root deck exists
        match global_context.get_config("root_deck".to_string()).unwrap() {
            Some(_) => {},
            None => {

                // root deck not found.
                // create a root deck.
                // use decks::CreateDeckRequest;
                let request = decks::CreateDeck {
                    name: "Library".to_string(),
                    description: "".to_string(),
                };

                let root_deck = global_context.create_deck(request).unwrap();

                global_context.set_config("root_deck".to_string(), format!("{}", root_deck.id)).unwrap();
            }
        }
    };

    /* router setup */

    // let route_info = RouteInfo{route: route.to_owned(), verb: verb};

    let mut router = Router::new();

    // TODO: https://webmasters.googleblog.com/2010/04/to-slash-or-not-to-slash.html

    // NOTE: compile-time reminder to add route!
    let _matcher = AppRoute::Home;
    match _matcher {
        AppRoute::Home => {},
        AppRoute::Settings => {},
        AppRoute::Stashes => {},
        AppRoute::Deck(_deck_id, ref _deck_route) => {
            match _deck_route {
                &DeckRoute::NewCard => {},
                &DeckRoute::NewDeck => {},
                &DeckRoute::Description => {},
                &DeckRoute::Decks => {},
                &DeckRoute::Cards => {},
                &DeckRoute::Meta => {},
                &DeckRoute::Settings => {},
                &DeckRoute::Review => {}
            }
        },
        AppRoute::Card(_deck_id, ref _card_route) => {
            match _card_route {
                &CardRoute::Profile => {},
                &CardRoute::Review => {},
            }
        },
        AppRoute::CardInDeck(_deck_id, _card_id, ref _card_route) => {
            match _card_route {
                &CardRoute::Profile => {},
                &CardRoute::Review => {}
            }
        }
    };

    route!(router, Get, AppRoute::Home);
    route!(router, Get, AppRoute::Settings);
    route!(router, Get, AppRoute::Stashes);
    route!(router, Get, AppRoute::Deck(default!(), DeckRoute::NewCard));
    route!(router, Get, AppRoute::Deck(default!(), DeckRoute::NewDeck));
    route!(router, Get, AppRoute::Deck(default!(), DeckRoute::Description));
    route!(router, Get, AppRoute::Deck(default!(), DeckRoute::Decks));
    route!(router, Get, AppRoute::Deck(default!(), DeckRoute::Cards));
    route!(router, Get, AppRoute::Deck(default!(), DeckRoute::Meta));
    route!(router, Get, AppRoute::Deck(default!(), DeckRoute::Settings));
    route!(router, Get, AppRoute::Deck(default!(), DeckRoute::Review));

    route!(router, Get, AppRoute::Card(default!(), CardRoute::Profile));
    route!(router, Get, AppRoute::Card(default!(), CardRoute::Review));
    route!(router, Get, AppRoute::CardInDeck(default!(), default!(), CardRoute::Profile));
    route!(router, Get, AppRoute::CardInDeck(default!(), default!(), CardRoute::Review));

    // API
    router.post(r"^/api/deck$", decks::routes::create_deck);


    // TODO: old; remove
    // router.get(r"^/$", route_root);
    // router.get(r"^/settings$", route_settings);

    // TODO: look into rust cargo env variable to switch between debug/prod
    // hardcode static resources and serve them via RAM
    // TODO: inline_static_asset!(router, "^/assets/deck_review.js$", "path/to/file", mime!(Application/Javascript))

    // TODO: limit path length? [NOTE]: use above approach?
    router.get(r"^/assets/(?P<path>.+)$", route::routes::static_assets);

    // TODO: what is this?
    // router.add_route(Method::Post, r"...", route)

    // Freeze the router. This will validate the router.
    router.finalize();

    let router = router;

    /* server */

    let mut server = Server::http(("0.0.0.0", 3000)).unwrap();

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

        let mut context = Context {

            global_context: &global_context,

            // TODO: remove
            // request: request,
            // response: response,

            // router/regexset
            uri: &uri,
            captures: None,

            // default view route
            view_route: AppRoute::Home
        };

        // middleware/logging
        // TODO: complete

        // middleware/router

        let handler = match router.handle(&mut context, &request) {
            Some(handler) => {
                handler
            },
            None => route::routes::not_found
        };

        handler(context, request, response);

    });

    println!("Listening on http://127.0.0.1:3000");
}
