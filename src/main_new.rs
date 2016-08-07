#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

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
use std::ascii::{AsciiExt};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex, LockResult, MutexGuard, RwLock};
use std::fs::{self, File};
use std::path::{PathBuf, Path};
use std::ffi::OsStr;
use std::collections::HashMap;

/* 3rd-party imports */

use rusqlite::{Connection};
use rusqlite::types::ToSql;
use rusqlite::{Error as SqliteError};

use url::percent_encoding::{percent_decode};

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

mod types;
#[macro_use]
mod macros;
mod parsers;
mod tables;
mod context;
mod api;
#[macro_use]
mod errors;
mod log_entry;
mod route;
mod components;


use context::Context;
use log_entry::LogEntry;
use errors::{RawAPIError};
use api::configs;
use route::parse_request_uri;
use components::render_response;
use route::RenderResponse;

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
// main
////////////////////////////////////////////////////////////////////////////////

fn main() {

    /* database */

    let db_connection = match Connection::open("test.db") {
        Err(why) => {
            // TODO: fix
            panic!("{}", why);
        },
        Ok(db_conn) => Arc::new(RwLock::new(Mutex::new(db_conn)))
    };

    /* table setup */

    match tables::setup_database(db_connection.clone()) {
        Ok(_) => {},
        Err(why) => {
            handle_raw_api_error!(why);
            return;
        }
    }

    /* database bootstrap */

    configs::api_get_config(db_connection.clone(), configs::CONFIG_ROOT_DECK_ID_KEY);

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

        let context = Context {
            database: db_connection.clone()
        };

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
