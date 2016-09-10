#![feature(plugin, custom_derive)]
#![plugin(serde_macros)]
#![plugin(indoc)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate guardian;
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
extern crate random_wheel;
extern crate rand;
extern crate libc;
extern crate pcg;

/* rust lib imports */

use std::panic::{self, AssertUnwindSafe};
use std::io;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

/* 3rd-party imports */

use hyper::server::{Server, Request, Response};

use chomp::{parse_only};

/* local imports */

mod types;
mod constants;
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


use context::{Context};
use log_entry::LogEntry;
use api::{configs, decks, cards};
use route::parse_request_uri;
use route::{RenderResponse, render_response};
use types::DeckID;

/* ////////////////////////////////////////////////////////////////////////// */

fn main() {

    /* global vars */

    let mut root_deck_id = 1;

    /* database */

    let global_lock = {
        let db_connection = database::get_database("test.db".to_string());
        let global_lock = Arc::new(RwLock::new(db_connection));
        global_lock
    };

    /* database bootstrap */

    {
        let bootstrap_context = Rc::new(RefCell::new(Context::new(global_lock.clone())));

        let _read_guard = context::write_lock(bootstrap_context.clone());

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

    // TODO: debug
    {

        let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));

        let _read_guard = context::write_lock(context.clone());

        for deck_num in 1..245 {

            let request = decks::CreateDeck {
                name: format!("deck {}", deck_num),
                description: format!("description for deck {}", deck_num),
            };

            match decks::create_deck(context.clone(), request) {
                Ok(deck) => {

                    match decks::connect_decks(context.clone(), deck.id, root_deck_id) {
                        Ok(_) => assert!(true),
                        Err(_) => assert!(false)
                    }
                },
                Err(_) => assert!(false),
            }
        }
    };

    {

        let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));

        let _read_guard = context::write_lock(context.clone());

        for card_num in 1..245 {

            let request = cards::CreateCard {
                title: format!("card num {}", card_num),
                description: format!("card description {}", card_num),
                question: format!("card question {}", card_num),
                answer: format!("card answer {}", card_num),
                is_active: card_num % 2 == 0
            };

            match cards::create_card(context.clone(), 1, request) {
                Ok(_) => {
                },
                Err(_) => assert!(false),
            }
        }
    };


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

            global_lock: global_lock.clone(),
            lock_state: None,
            lock_state_ref_read_count: 0,
            lock_state_ref_write_count: 0,

            root_deck_id: root_deck_id,
            request_uri: format!("{}", request.uri),

            /* caching */

            should_cache: true,

            decks: HashMap::new(),
            deck_children_count: HashMap::new(),
            cards: HashMap::new(),
        };

        let context = Rc::new(RefCell::new(context));

        let _guard = context::write_lock(context.clone());

        // middleware/logging
        // TODO: complete

        /* middleware/router */

        let uri = format!("{}", request.uri);
        let request: Rc<RefCell<_>> = Rc::new(RefCell::new(request));

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            let render = match parse_only(|i| parse_request_uri(i, context.clone(), request.clone()), uri.as_bytes()) {
                Ok(render_response) => {
                    // url has a match
                    render_response
                }
                Err(_why) => {

                    // 404 error
                    // TODO: internal error logging

                    // panic!("{:?}", e);
                    RenderResponse::NotFound
                }
            };
            render
        }));

        // NOTE: This is a boundary line where the request has been transformed into
        //       RenderResponse type. Anything from request shall be put into the RenderResponse type.

        match result {
            Err(why) => {
                // TODO: internal error logging

                let reason: String = if let Some(why) = why.downcast_ref::<String>() {
                    format!("{}", why)
                } else if let Some(why) = why.downcast_ref::<&str>() {
                    format!("{}", why)
                } else {
                    format!("{:?}", why)
                };

                println!("PANIC REASON: {}", reason);

                render_response(context.clone(), RenderResponse::InternalServerError, response);
                return;
            },
            Ok(render) => {
                render_response(context, render, response);
                return;
            }
        }

        return;

    });

    let (host, port) = address;
    println!("Listening on http://{}:{}", host, port);

}
