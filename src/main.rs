#![feature(proc_macro)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate guardian;
#[macro_use]
extern crate serde_derive;
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
extern crate phf;
extern crate datetime;
extern crate clap;
extern crate separator;


/* rust lib imports */

use std::panic::{self, AssertUnwindSafe};
use std::io;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

/* 3rd-party imports */

use hyper::server::{Server, Request, Response};
use clap::{Arg, App};
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
#[allow(non_snake_case)]
mod components;
mod timestamp;
#[allow(non_snake_case)]
mod timezones;


use context::{Context};
use log_entry::LogEntry;
use api::{user};
use route::parse_request_uri;
use route::{RenderResponse, render_response};

/* ////////////////////////////////////////////////////////////////////////// */

fn main() {

    /* parse command line args */

    let cmd_matches = App::new("grokdb")
        .version("0.1 (semver)") // semver semantics
        .author("Alberto Leal <mailforalberto@gmail.com> (github.com/dashed)")
        .about("flashcard app to help you grok better")
        .arg(
            Arg::with_name("port")
            .short("p")
            .long("port")
            .help("Port number to serve")
            .takes_value(true)
            .required(false)
            .validator(|port| {
                let port = port.trim();
                if port.len() <= 0 {
                    return Err(String::from("invalid port number"));
                }
                match port.parse::<u16>() {
                    Ok(_) => {
                        return Ok(());
                    },
                    _ => {
                        return Err(String::from("invalid port number"));
                    }
                };
            })
        )
        .arg(
            Arg::with_name("database_name")
            .help("Database name to store your flashcards")
            .required(true)
            .index(1)
            .validator(|database_name| {
                let database_name = database_name.trim();
                if database_name.len() <= 0 {
                    return Err(String::from("invalid database name"));
                } else {
                    return Ok(());
                }
            })
        ).get_matches();

    // fetch database name

    let mut database_name: String = cmd_matches.value_of("database_name")
                                                .unwrap()
                                                .trim()
                                                .to_string();

    if !database_name.to_lowercase().ends_with(".db") {
        database_name = format!("{}.db", database_name);

        println!("Using database: {}", database_name);
    }

    let database_name = database_name;

    // fetch port number to serve at
    let server_port: u16 = match cmd_matches.value_of("port") {
        // Binding with a port number of 0 will request that the OS assigns
        // a port to this listener.
        None => 0u16,
        Some(ref port_str) => {

            let port_str = port_str.trim();

            match port_str.parse::<u16>() {
                Ok(port) => port,
                Err(_) => {
                     // should already be validated to be u16
                    // unreachable!();
                    0u16
                }

            }
        }
    };

    /* database */

    let global_lock = {
        let db_connection = database::get_database(database_name);
        let global_lock = Arc::new(RwLock::new(db_connection));
        global_lock
    };

    /* database bootstrap */

    {
        let bootstrap_context = Rc::new(RefCell::new(Context::new(global_lock.clone())));

        let _read_guard = context::write_lock(bootstrap_context.clone());

        user::set_up_user(bootstrap_context.clone()).unwrap();

    };

    // TODO: debug
    // {
            // use api::{decks, cards, user};

    //     let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));

    //     let _read_guard = context::write_lock(context.clone());

    //     let root_deck_id = user::get_root_deck(context.clone()).unwrap().unwrap();

    //     for deck_num in 1..245 {

    //         let request = decks::CreateDeck {
    //             name: format!("deck {}", deck_num),
    //             description: format!("description for deck {}", deck_num),
    //         };

    //         match decks::create_deck(context.clone(), request) {
    //             Ok(deck) => {

    //                 match decks::connect_decks(context.clone(), deck.id, root_deck_id) {
    //                     Ok(_) => assert!(true),
    //                     Err(_) => assert!(false)
    //                 }
    //             },
    //             Err(_) => assert!(false),
    //         }
    //     }
    // };

    // {
        // use api::{decks, cards, user};

    //     let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));

    //     let _read_guard = context::write_lock(context.clone());

    //     for card_num in 1..245 {

    //         let request = cards::CreateCard {
    //             title: format!("card num {}", card_num),
    //             description: format!("card description {}", card_num),
    //             question: format!("card question {}", card_num),
    //             answer: format!("card answer {}", card_num),
    //             is_active: card_num % 2 == 0
    //         };

    //         match cards::create_card(context.clone(), 1, request) {
    //             Ok(_) => {
    //             },
    //             Err(_) => assert!(false),
    //         }
    //     }
    // };

    /* server */

    let address = ("0.0.0.0", server_port);

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

            user_id: 42,
            request_uri: format!("{}", request.uri),

            /* caching */

            should_cache: true,

            decks: HashMap::new(),
            deck_children_count: HashMap::new(),
            cards: HashMap::new(),
        };

        let context = Rc::new(RefCell::new(context));

        // TODO: this needs to be removed!!!
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

    });

    let (host, port) = address;
    println!("Listening on http://{}:{}", host, port);

}
