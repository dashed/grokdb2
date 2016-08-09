#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate serde;
extern crate serde_json;
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
extern crate rusqlite;
#[macro_use(quick_error)]
extern crate quick_error;

/* rust lib imports */

use std::io;
use std::ascii::AsciiExt;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex, LockResult, MutexGuard, RwLock};
use std::fs::{self, File};
use std::path::{PathBuf, Path};
use std::ffi::OsStr;
use std::collections::HashMap;

/* 3rd-party imports */

use rusqlite::Connection;
use rusqlite::types::ToSql;
use rusqlite::Error as SqliteError;

use url::percent_encoding::percent_decode;

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

/* local imports */

mod types;
// TODO: remove; cleanup
// #[macro_use]
// mod macros;
#[macro_use]
mod errors;
#[macro_use]
mod database;
mod parsers;
mod tables;
mod context;
mod api;
mod log_entry;
mod route;
mod components;


use context::Context;
use log_entry::LogEntry;
use errors::RawAPIError;
use api::{configs, decks};
use route::parse_request_uri;
use components::render_response;
use route::RenderResponse;

/// /////////////////////////////////////////////////////////////////////////////

/// /////////////////////////////////////////////////////////////////////////////
/// main
/// /////////////////////////////////////////////////////////////////////////////

fn main() {

    /* database */

    let db_connection = database::get_database("test.db".to_string());

    /* database bootstrap */

    let bootstrap_context = Context::new(db_connection.clone());

    let mut root_deck_id = 1;

    let should_create_root_deck = match configs::get_config(bootstrap_context.clone(),
                                                            configs::CONFIG_ROOT_DECK_ID_KEY.to_string())
        .unwrap() {
        Some(config) => {
            let deck_id = config.value;

            match deck_id.parse::<i64>() {
                Ok(deck_id) => {
                    root_deck_id = deck_id;
                    false
                }
                Err(_) => true,
            }

        }
        None => true,
    };

    if should_create_root_deck {
        // root deck not found.
        // create a root deck.
        let request = decks::CreateDeck {
            name: "Library".to_string(),
            description: "".to_string(),
        };

        let root_deck = decks::create_deck(bootstrap_context.clone(), request).unwrap();

        configs::set_config(bootstrap_context.clone(),
                            configs::CONFIG_ROOT_DECK_ID_KEY.to_string(),
                            format!("{}", root_deck.id))
            .unwrap();
    }

    drop(bootstrap_context);

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

        let context = Context { database: db_connection.clone() };

        // middleware/logging
        // TODO: complete

        /* middleware/router */

        let uri = format!("{}", request.uri);
        let request: Rc<RefCell<_>> = Rc::new(RefCell::new(request));

        let render = match parse_only(|i| parse_request_uri(i, request.clone()), uri.as_bytes()) {
            Ok(render_response) => {
                // url has a match
                render_response
            }
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
