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
#[macro_use]
extern crate matches;
extern crate chrono;

/* rust lib imports */

use std::io;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

/* 3rd-party imports */

use hyper::server::{Server, Request, Response};

use chomp::{parse_only};

/* local imports */

mod types;
#[macro_use]
mod global_macros;
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
use api::{configs, decks};
use route::parse_request_uri;
use route::{RenderResponse, render_response};
use types::DeckID;

/* ////////////////////////////////////////////////////////////////////////// */

fn main() {

    /* global vars */

    let mut root_deck_id = 1;

    /* database */

    let db_connection = database::get_database("test.db".to_string());

    /* database bootstrap */

    {
        let bootstrap_context = Rc::new(RefCell::new(Context::new(db_connection.clone())));

        let should_create_root_deck = match configs::get_config(bootstrap_context.clone(),
                                                                configs::CONFIG_ROOT_DECK_ID_KEY.to_string())
            .unwrap() {
            Some(config) => {
                let deck_id = config.value;

                match deck_id.parse::<DeckID>() {
                    Ok(deck_id) => {

                        match decks::deck_exists(bootstrap_context.clone(), deck_id) {
                            Ok(exists) => {

                                if exists {
                                    root_deck_id = deck_id;
                                }

                                !exists
                            },
                            Err(why) => {
                                handle_raw_api_error!(why);

                                // TODO: fix
                                panic!("decks::deck_exists");
                            }
                        }

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

            root_deck_id = root_deck.id;
        }

    };

    // freeze root_deck_id
    let root_deck_id = root_deck_id;

    println!("Root deck id: {}", root_deck_id);

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
            request_uri: format!("{}", request.uri),
            root_deck_id: root_deck_id,
            database: db_connection.clone(),

            /* caching */

            should_cache: true,

            decks: HashMap::new()
        };

        let context = Rc::new(RefCell::new(context));

        // middleware/logging
        // TODO: complete

        /* middleware/router */

        let uri = format!("{}", request.uri);
        let request: Rc<RefCell<_>> = Rc::new(RefCell::new(request));

        let render = match parse_only(|i| parse_request_uri(i, context.clone(), request.clone()), uri.as_bytes()) {
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

        render_response(context, render, response);

        return ();

    });

    let (host, port) = address;
    println!("Listening on http://{}:{}", host, port);

}
