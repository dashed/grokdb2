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
use types::{APIStatus, DeckID, CardID, DecksPageQuery, CardsPageQuery, Search, JSONResponse, MoveDecksPageQuery};
use context::{self, Context};
use components::{AppComponent, view_route_to_link, generate_move_to};
use api::decks::{self, CreateDeck, DeckCreateResponse, UpdateDeckDescription, UpdateDeckName};
use api::cards;
use api::user;
use api::review::{self,
    CachedReviewProcedure, RawDeckReviewRequest, RawCardReviewRequest, Reviewable, ReviewResponse};

/* ////////////////////////////////////////////////////////////////////////// */

/* statics */

lazy_static! {
    static ref MIME_TYPES: mime_types::Types = mime_types::Types::new().unwrap();

    // TODO: internationalization for these strings below
    static ref INTERNAL_SERVER_ERROR_STRING: String = "An internal server error occurred.".to_string();

    // NOTE: This is for API responses
    static ref API_METHOD_NOT_ALLOWED_STRING: String =
        "An invalid request was sent. (ERROR: Method not allowed)".to_string();
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

    // TODO: remove
    // Card(CardID, CardRoute),
    // CardInDeck(DeckID, CardID, CardRoute),
}

#[derive(Debug, Clone)]
pub enum CardRoute {
    Contents,
    Review,
    Stats,
    Settings(CardSettings)
}

#[derive(Debug, Clone)]
pub enum CardSettings {
    Main,
    Move(MoveDecksPageQuery, Search)
}

impl Default for CardRoute {
    fn default() -> Self {
        CardRoute::Contents
    }
}

#[derive(Debug, Clone)]
pub enum DeckRoute {

    NewCard,
    NewDeck,

    Description,

    Decks(DecksPageQuery, Search), // list
    Cards(CardsPageQuery, Search), // list

    Settings(DeckSettings),

    Stats,
    Review(Option<(CardID, Option<CachedReviewProcedure>)>),

    CardProfile(CardID, CardRoute)

    // Create,
    // Read,
    // Update,
    // http://stackoverflow.com/a/26897298/412627
    // http://programmers.stackexchange.com/questions/114156/why-are-there-are-no-put-and-delete-methods-on-html-forms
    // Delete
}

impl Default for DeckRoute {
    fn default() -> Self {
        DeckRoute::Decks(Default::default(), Default::default())
    }
}

#[derive(Debug, Clone)]
pub enum DeckSettings {
    Main,
    Move
}

#[derive(Debug)]
pub enum RenderResponse {

    Component(AppRoute),

    // NOTE: these lead to html content
    NotFound,
    BadRequest,
    InternalServerError,

    // API response
    JSON(APIStatus, JSONResponse),

    Asset(ContentType, Vec<u8>),

    MethodNotAllowed
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

            // TODO: complete
            // /card/*
            // parse_route_card() <|>

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
        return RenderResponse::MethodNotAllowed;
    }

    // TODO: inlined resources here

    // URL decode
    let decoded_req_path = Path::new(&path).iter().map(decode_percents);

    let starts_with = Path::new("./assets/").to_path_buf().canonicalize().unwrap();

    let mut req_path = starts_with.clone();
    req_path.extend(decoded_req_path);
    let req_path: PathBuf = req_path;

    // TODO: this is a security bottle-neck
    let req_path = match req_path.canonicalize() {
        Err(_) => {
            return RenderResponse::NotFound;
        },
        Ok(req_path) => {

            if !req_path.starts_with(starts_with.as_path()) {
                return RenderResponse::NotFound;
            }

            req_path
        }
    };


    match fs::metadata(&req_path) {
        Ok(metadata) => {

            if !metadata.is_file() {
                return RenderResponse::NotFound;
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

            RenderResponse::Asset(content_type, content)

        },
        Err(e) => {
            return RenderResponse::NotFound;
        },
    }

}

#[inline]
fn parse_assets<'a>(input: Input<'a, u8>, request: Rc<RefCell<Request>>) -> U8Result<'a, RenderResponse> {
    parse!{input;

        string_ignore_case(b"assets");
        string_ignore_case(b"/");

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

                eof();

                ret {
                    __parse_route_root(context, request, query_string)
                }
            },
            |i| parse!{i;

                // null parser

                ret {
                    RenderResponse::NotFound
                }
            }
        );

        ret result
    }
}

#[inline]
fn __parse_route_root(
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    query_string: Option<QueryString>) -> RenderResponse {

    let root_deck_id = {
        let _guard = context::read_lock(context.clone());
        let result = handle_api_result_html!(user::get_root_deck(context.clone()));

        match result {
            None => {

                // TODO: fix; this is fatal error, since root deck should exists

                return RenderResponse::InternalServerError;
            },
            Some(root_deck_id) => root_deck_id
        }
    };

    route_deck_decks(context, request, root_deck_id, query_string)

}

#[inline]
fn parse_route_api<'a>(input: Input<'a, u8>, context: Rc<RefCell<Context>>, request: Rc<RefCell<Request>>)
-> U8Result<'a, RenderResponse> {
    parse!{input;

        string_ignore_case(b"api");

        string_ignore_case(b"/");

        let render_response = parse_route_api_deck(context.clone(), request.clone()) <|>
            parse_route_api_card(context.clone(), request.clone()) <|>
            invalid_route_api(context.clone());

        ret render_response

    }
}

#[inline]
fn invalid_route_api<'a>(input: Input<'a, u8>, context: Rc<RefCell<Context>>)
-> U8Result<'a, RenderResponse> {
    parse!{input;

        ret {

            // TODO: statically allocate this?

            let reason = "Invalid API Path.".to_string();

            respond_json_with_error!(APIStatus::BadRequest; reason; None)
        }
    }
}

#[inline]
fn parse_route_api_card<'a>(input: Input<'a, u8>, context: Rc<RefCell<Context>>, request: Rc<RefCell<Request>>)
-> U8Result<'a, RenderResponse> {
    (parse!{input;

        string_ignore_case(b"card");
        string_ignore_case(b"/");
        let card_id: CardID = decimal();

        ret card_id

    }).bind(|i, card_id| {

        let _guard = context::write_lock(context.clone());

        match cards::card_exists(context.clone(), card_id) {
            Ok(exists) => {

                if exists {
                    __parse_route_api_card(i, context.clone(), request, card_id)
                } else {

                    i.ret({
                        let reason = "Card does not exist.".to_string();
                        respond_json_with_error!(APIStatus::BadRequest; reason; None)
                    })

                }

            },
            // TODO: internal error logging
            Err(_) => i.ret({
                // TODO: internal error logging
                respond_json_with_error!(APIStatus::ServerError; INTERNAL_SERVER_ERROR_STRING.clone(); None)
            })
        }
    })

}

#[inline]
fn __parse_route_api_card<'a>(
    input: Input<'a, u8>,
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    card_id: CardID)
-> U8Result<'a, RenderResponse> {

    parse!{input;

        let render_response = or(|i| parse!{i;

            string_ignore_case(b"/");

            // TODO: reorder for micro optimization
            let render_response = parse_route_api_card_update(context.clone(), request.clone(), card_id) <|>
                parse_route_api_card_review(context.clone(), request.clone(), card_id) <|>
                parse_route_api_card_move(context.clone(), request.clone(), card_id);

            ret render_response

        }, |i| parse!{i;

            eof();

            ret {
                parse_route_api_card_root(context.clone(), request.clone(), card_id)
            }

        });

        ret render_response
    }

}

#[inline]
fn parse_route_api_card_root(
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    card_id: DeckID) -> RenderResponse {

    let method = {
        let request = request.borrow();
        request.method.clone()
    };

    if method != Method::Delete {
        return RenderResponse::MethodNotAllowed;
    }

    // handle DELETE request to delete this card

    let card = handle_api_result_json!(cards::get_card(context.clone(), card_id));

    // delete card
    handle_api_result_json!(cards::delete_card(context.clone(), card_id));

    let card_delete_response = cards::DeleteCardResponse {
        redirect_to: view_route_to_link(context, AppRoute::Deck(card.deck_id,
            DeckRoute::Cards(Default::default(), Default::default())))
    };

    return respond_json!(Some(card_delete_response));
}

#[inline]
fn parse_route_api_card_move<'a>(
    input: Input<'a, u8>,
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    card_id: CardID) -> U8Result<'a, RenderResponse> {
    parse!{input;

        string_ignore_case(b"move");
        eof();

        ret {
            __parse_route_api_card_move(context, request, card_id)
        }
    }
}

#[inline]
fn __parse_route_api_card_move(
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    card_id: CardID) -> RenderResponse {

    // invariant: card exists

    let mut request = request.borrow_mut();

    if request.method != Method::Post {
        return RenderResponse::MethodNotAllowed;
    }

    // POST /api/card/:id/move

    let mut buffer = String::new();

    match request.read_to_string(&mut buffer) {
        Err(err) => {
            // internal reason: invalid utf8 input

            // TODO: internal error logging
            let err = "Unable to move this card. Please try again.".to_string();
            return respond_json_with_error!(APIStatus::BadRequest; err; None);

        },
        Ok(_num_bytes_parsed) => {

            let request: cards::MoveCardRequest = match serde_json::from_str(&buffer) {
                Ok(request) => request,
                Err(err) => {

                    // TODO: internal error logging
                    let err = "Unable to move this card. Please try again.".to_string();
                    return respond_json_with_error!(APIStatus::BadRequest; err; None);
                }
            };

            // ensure deck exists
            match decks::deck_exists(context.clone(), request.deck_id) {
                Ok(true) => {},
                Ok(false) => {
                    let err = "This deck does not exist.".to_string();
                    return respond_json_with_error!(APIStatus::BadRequest; err; None);
                },
                Err(err) => {
                    // TODO: internal error logging
                    return respond_json_with_error!(APIStatus::ServerError;
                        INTERNAL_SERVER_ERROR_STRING.clone(); None);
                }
            }

            match cards::move_card(context.clone(), card_id, request.deck_id) {
                Ok(_card) => {

                    let response = cards::MoveCardResponse {
                        redirect_to: view_route_to_link(context.clone(), AppRoute::Deck(request.deck_id,
                            DeckRoute::CardProfile(card_id,
                                CardRoute::Settings(CardSettings::Move(
                                    MoveDecksPageQuery::default(context.clone(), card_id),
                                    Default::default())))))
                    };

                    return respond_json!(Some(response));
                },
                Err(err) => {
                    // TODO: internal error logging
                    return respond_json_with_error!(APIStatus::ServerError;
                        INTERNAL_SERVER_ERROR_STRING.clone(); None);
                }
            }
        }
    }
}

#[inline]
fn parse_route_api_card_review<'a>(
    input: Input<'a, u8>,
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    card_id: CardID) -> U8Result<'a, RenderResponse> {
    parse!{input;

        string_ignore_case(b"review");
        eof();

        ret {
            __parse_route_api_card_review(context, request, card_id)
        }
    }
}

#[inline]
fn __parse_route_api_card_review(
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    card_id: CardID) -> RenderResponse {

    // invariant: card exists

    let mut request = request.borrow_mut();

    if request.method == Method::Get {

        // GET /api/card/:id/review

        let card = handle_api_result_json!(cards::get_card(context.clone(), card_id));
        let card_score = handle_api_result_json!(review::get_card_score(context.clone(), card_id));

        let response = cards::ReviewCardResponse {
            card: card,
            card_score: card_score
        };

        return respond_json!(Some(response));

    }

    if request.method != Method::Post {
        return RenderResponse::MethodNotAllowed;
    }

    // POST /api/card/:id/review

    let mut buffer = String::new();

    match request.read_to_string(&mut buffer) {
        Ok(_num_bytes_parsed) => {

            let request: RawCardReviewRequest = match serde_json::from_str(&buffer) {
                Ok(request) => request,
                Err(err) => {

                    // TODO: internal error logging
                    let err = "Unable to review this card. Please try again.".to_string();
                    return respond_json_with_error!(APIStatus::BadRequest; err; None);
                }
            };

            let request = request.normalize(card_id);

            let _guard = context::write_lock(context.clone());

            // commit review request
            handle_api_result_json!(request.commit(context.clone()));

            let card = handle_api_result_json!(cards::get_card(context.clone(), card_id));
            let card_score = handle_api_result_json!(review::get_card_score(context.clone(), card_id));

            let response = cards::ReviewCardResponse {
                card: card,
                card_score: card_score
            };

            return respond_json!(Some(response));

        },
        Err(err) => {
            // internal reason: invalid utf8 input

            // TODO: internal error logging
            let err = "Unable to review this card. Please try again.".to_string();
            return respond_json_with_error!(APIStatus::BadRequest; err; None);

        }
    }
}


#[inline]
fn parse_route_api_card_update<'a>(
    input: Input<'a, u8>,
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    card_id: CardID) -> U8Result<'a, RenderResponse> {
    parse!{input;

        string_ignore_case(b"update");
        eof();

        ret {
            __parse_route_api_card_update(context, request, card_id)
        }
    }
}

#[inline]
fn __parse_route_api_card_update(
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    card_id: CardID) -> RenderResponse {

    let mut request = request.borrow_mut();

    if request.method != Method::Post {
        return RenderResponse::MethodNotAllowed;
    }

    // POST /api/card/:id/update

    let mut buffer = String::new();

    match request.read_to_string(&mut buffer) {
        Ok(_num_bytes_parsed) => {

            let request: cards::UpdateCard = match serde_json::from_str(&buffer) {
                Ok(request) => request,
                Err(err) => {
                    // TODO: error logging
                    // println!("{:?}", err);
                    return RenderResponse::BadRequest;
                }
            };

            match request.validate() {
                None => {},
                Some(reason) => {
                    return respond_json_with_error!(APIStatus::BadRequest; reason; None);
                }
            }

            // TODO: refactor
            let mut request = request;
            if request.title.trim().len() <= 0 {
                // invariant: request.question is not empty

                // if card has question content, fetch first 140 characters as title

                let len = if request.question.len() < 140 {
                    request.question.len()
                } else {
                    140
                };

                let new_title = &(request.question)[..len];
                let new_title = new_title.trim().to_string();

                request.title = new_title;
            };
            let request = request;

            let _guard = context::write_lock(context.clone());

            let updated_card = handle_api_result_json!(cards::update_card(context.clone(), card_id, request));

            let card_score = handle_api_result_json!(review::get_card_score(context.clone(), card_id));

            let response = cards::UpdatedCardResponse {
                card: updated_card,
                card_score: card_score
            };

            return respond_json!(Some(response));

        },
        Err(err) => {
            // invalid utf8 input
            // TODO: error logging
            return RenderResponse::BadRequest;
        }
    }
}

#[inline]
fn parse_route_api_deck<'a>(input: Input<'a, u8>, context: Rc<RefCell<Context>>, request: Rc<RefCell<Request>>)
-> U8Result<'a, RenderResponse> {
    (parse!{input;

        string_ignore_case(b"deck");
        string_ignore_case(b"/");
        let deck_id: DeckID = decimal();

        ret deck_id

    }).bind(|i, deck_id| {

        let _guard = context::write_lock(context.clone());

        match decks::deck_exists(context.clone(), deck_id) {
            Ok(exists) => {

                if exists {
                    __parse_route_api_deck(i, context.clone(), request, deck_id)
                } else {
                    i.ret({
                        let reason = "Deck does not exist.".to_string();
                        respond_json_with_error!(APIStatus::BadRequest; reason; None)
                    })
                }

            },
            Err(_) => i.ret({
                // TODO: internal error logging
                respond_json_with_error!(APIStatus::ServerError; INTERNAL_SERVER_ERROR_STRING.clone(); None)
            })
        }
    })

}

#[inline]
fn __parse_route_api_deck<'a>(
    input: Input<'a, u8>,
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID)
-> U8Result<'a, RenderResponse> {

    parse!{input;

        let render_response = or(|i| parse!{i;

            string_ignore_case(b"/");

            // TODO: reorder for micro optimization
            let render_response =
                parse_route_api_deck_description(context.clone(), request.clone(), deck_id) <|>
                parse_route_api_deck_settings(context.clone(), request.clone(), deck_id) <|>
                parse_route_api_deck_new_deck(context.clone(), request.clone(), deck_id) <|>
                parse_route_api_deck_new_card(context.clone(), request.clone(), deck_id) <|>
                parse_route_api_deck_review(context.clone(), request.clone(), deck_id) <|>
                parse_route_api_deck_move(context.clone(), request.clone(), deck_id);

            ret render_response

        }, |i| parse!{i;

            eof();

            ret {
                parse_route_api_deck_root(context.clone(), request.clone(), deck_id)
            }

        });

        ret render_response
    }

}

#[inline]
fn parse_route_api_deck_root(
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID) -> RenderResponse {

    let method = {
        let request = request.borrow();
        request.method.clone()
    };

    if method != Method::Delete {
        return RenderResponse::MethodNotAllowed;
    }

    // handle DELETE request to delete this deck

    // check if this deck is root deck
    if handle_api_result_json!(user::is_root_deck(context.clone(), deck_id)) {

        // TODO: review and revise phrasing as necessary
        let err = "You are not allowed to delete the top-most deck.".to_string();
        return respond_json_with_error!(APIStatus::BadRequest; err; None);
    }

    // get parent deck
    let parent_deck_id = match handle_api_result_json!(decks::get_parent_id_of_deck(context.clone(), deck_id)) {
        None => {

            // TODO: error: parent_deck_id should exist

            // TODO: internal error logging
            return respond_json_with_error!(APIStatus::ServerError;
                "An internal server error occurred.".to_string(); None);

        },
        Some(parent_deck_id) => parent_deck_id
    };

    // delete deck
    handle_api_result_json!(decks::delete_deck(context.clone(), deck_id));

    let deck_delete_response = decks::DeleteDeckResponse {
        redirect_to: view_route_to_link(context, AppRoute::Deck(parent_deck_id, Default::default()))
    };

    return respond_json!(Some(deck_delete_response));
}

#[inline]
fn parse_route_api_deck_move<'a>(
    input: Input<'a, u8>,
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID) -> U8Result<'a, RenderResponse> {

    parse!{input;

        string_ignore_case(b"move");

        eof();

        ret {
            __parse_route_api_deck_move(context, request, deck_id)
        }

    }

}

#[inline]
fn __parse_route_api_deck_move(
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    child_deck_id: DeckID) -> RenderResponse {

    // invariant: deck exists

    let mut request = request.borrow_mut();

    if request.method != Method::Post {
        return RenderResponse::MethodNotAllowed;
    }

    // POST /api/deck/:id/move

    let mut buffer = String::new();

    match request.read_to_string(&mut buffer) {
        Err(err) => {
            // internal reason: invalid utf8 input

            // TODO: internal error logging
            let err = "Unable to move this deck. Please try again.".to_string();
            return respond_json_with_error!(APIStatus::BadRequest; err; None);

        },
        Ok(_num_bytes_parsed) => {

            let request: decks::MoveDeckRequest = match serde_json::from_str(&buffer) {
                Ok(request) => request,
                Err(err) => {

                    // TODO: internal error logging
                    let err = "Unable to move this deck. Please try again.".to_string();
                    return respond_json_with_error!(APIStatus::BadRequest; err; None);
                }
            };

            // ensure deck exists
            match decks::deck_exists(context.clone(), request.deck_id) {
                Ok(true) => {},
                Ok(false) => {
                    let err = "This deck does not exist.".to_string();
                    return respond_json_with_error!(APIStatus::BadRequest; err; None);
                },
                Err(err) => {
                    // TODO: internal error logging
                    return respond_json_with_error!(APIStatus::ServerError;
                        INTERNAL_SERVER_ERROR_STRING.clone(); None);
                }
            }

            match decks::connect_decks(context.clone(), child_deck_id, request.deck_id) {
                Err(err) => {
                    // TODO: internal error logging
                    return respond_json_with_error!(APIStatus::ServerError;
                        INTERNAL_SERVER_ERROR_STRING.clone(); None);
                },
                Ok(()) => {

                    let response = decks::MoveDeckResponse {
                        redirect_to: view_route_to_link(context.clone(), AppRoute::Deck(child_deck_id,
                            DeckRoute::Settings(DeckSettings::Move)))
                    };

                    return respond_json!(Some(response));

                }
            }

        }
    }
}

#[inline]
fn parse_route_api_deck_review<'a>(
    input: Input<'a, u8>,
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    parent_deck_id: DeckID) -> U8Result<'a, RenderResponse> {

    parse!{input;

        string_ignore_case(b"review");

        eof();

        ret {
            __parse_route_api_deck_review(context, request, parent_deck_id)
        }

    }
}

#[inline]
fn __parse_route_api_deck_review(
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    parent_deck_id: DeckID)
-> RenderResponse {

    let method = {
        let request = request.borrow();
        request.method.clone()
    };

    match method {
        Method::Get => {
            // GET /api/deck/:id/review
            return __parse_route_api_deck_review_get(context, request, parent_deck_id);
        },
        Method::Post => {
            // POST /api/deck/:id/review
            return __parse_route_api_deck_review_post(context, request, parent_deck_id);
        },
        _ => {

            // TODO: internal error logging
            return respond_json_with_error!(APIStatus::MethodNotAllowed;
                API_METHOD_NOT_ALLOWED_STRING.clone(); None);
        }
    }

}

#[inline]
fn __parse_route_api_deck_review_get(
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    parent_deck_id: DeckID) -> RenderResponse {

    // invariant: deck exists
    let deck = match decks::get_deck(context.clone(), parent_deck_id) {
        Ok(deck) => deck,
        Err(_) => {

            // TODO: internal error logging
            return respond_json_with_error!(APIStatus::ServerError;
                INTERNAL_SERVER_ERROR_STRING.clone(); None);

        }
    };

    // fetch card for review
    match ReviewResponse::new(context.clone(), deck) {
        Ok(review_response) => {

            return respond_json!(Some(review_response));
        },
        Err(_) => {

            // TODO: internal error logging
            return respond_json_with_error!(APIStatus::ServerError;
                INTERNAL_SERVER_ERROR_STRING.clone(); None);

        }
    }

}

#[inline]
fn __parse_route_api_deck_review_post(
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    parent_deck_id: DeckID) -> RenderResponse {

    let mut request = request.borrow_mut();

    let mut buffer = String::new();

    match request.read_to_string(&mut buffer) {
        Ok(_num_bytes_parsed) => {

            let request: RawDeckReviewRequest = match serde_json::from_str(&buffer) {
                Ok(request) => request,
                Err(err) => {

                    // TODO: internal error logging
                    let err = "Unable to review the card in this deck. Please try again.".to_string();
                    return respond_json_with_error!(APIStatus::BadRequest; err; None);
                }
            };

            let request = request.normalize();

            let _guard = context::write_lock(context.clone());

            // invariant: deck exists
            let deck = handle_api_result_json!(decks::get_deck(context.clone(), parent_deck_id));

            let exists = handle_api_result_json!(cards::card_exists(context.clone(), request.card_id));

            if !exists {

                // TODO: code repeat
                let review_response = handle_api_result_json!(ReviewResponse::new(context.clone(), deck));

                let err = "This card does not appear to exist anymore.".to_string();
                return respond_json_with_error!(APIStatus::BadRequest; err; Some(review_response));

            }

            let in_deck = handle_api_result_json!(cards::is_card_in_deck(context.clone(),
                request.card_id, parent_deck_id));

            if !in_deck {

                // commit review request
                handle_api_result_json!(request.commit(context.clone()));

                // TODO: code repeat
                let review_response = handle_api_result_json!(ReviewResponse::new(context.clone(), deck));

                // TODO: internal error logging
                let err = "This card does not appear to be in this deck. \
                    However, the card will still be reviewed.".to_string();
                return respond_json_with_error!(APIStatus::Ok; err; Some(review_response));

            }

            let card_id = match handle_api_result_json!(deck.get_cached_card(context.clone())) {
                None => {

                    // commit review request
                    handle_api_result_json!(request.commit(context.clone()));

                    // TODO: code repeat
                    let review_response = handle_api_result_json!(ReviewResponse::new(context.clone(), deck));

                    // TODO: internal error logging
                    let err = "This card does not appear to be chosen for review for this deck. \
                        However, the card will still be reviewed.".to_string();
                    return respond_json_with_error!(APIStatus::Ok; err; Some(review_response));

                },
                Some((card_id, _)) => {
                    card_id
                }
            };

            if card_id != request.card_id {

                // commit review request
                handle_api_result_json!(request.commit(context.clone()));

                // TODO: code repeat
                let review_response = handle_api_result_json!(ReviewResponse::new(context.clone(), deck));

                // TODO: internal error logging
                let err = "This card does not appear to be chosen for review for this deck. \
                    However, the card will still be considered reviewed.".to_string();
                return respond_json_with_error!(APIStatus::Ok; err; None);

            }

            // commit review request
            handle_api_result_json!(request.commit(context.clone()));

            // fetch next card for review
            let review_response = handle_api_result_json!(ReviewResponse::new(context.clone(), deck));

            return respond_json!(Some(review_response));

        },
        Err(err) => {
            // internal reason: invalid utf8 input

            // TODO: internal error logging
            let err = "Unable to review the card in this deck. Please try again.".to_string();
            return respond_json_with_error!(APIStatus::BadRequest; err; None);

        }
    }
}

#[inline]
fn parse_route_api_deck_new_deck<'a>(
    input: Input<'a, u8>,
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    parent_deck_id: DeckID)
-> U8Result<'a, RenderResponse> {

    parse!{input;

        string_ignore_case(b"new");
        string_ignore_case(b"/");
        string_ignore_case(b"deck");

        eof();

        ret {
            __parse_route_api_deck_new_deck(context, request, parent_deck_id)
        }

    }
}

#[inline]
fn __parse_route_api_deck_new_deck(
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    parent_deck_id: DeckID)
-> RenderResponse {

    let mut request = request.borrow_mut();

    if request.method != Method::Post {

        // TODO: internal error logging
        return respond_json_with_error!(APIStatus::MethodNotAllowed;
            API_METHOD_NOT_ALLOWED_STRING.clone(); None);

    }

    // POST /api/deck/:id/new/deck => create a new deck within this deck

    let mut buffer = String::new();

    match request.read_to_string(&mut buffer) {
        Ok(_num_bytes_parsed) => {

            let request: CreateDeck = match serde_json::from_str(&buffer) {
                Ok(request) => request,
                Err(err) => {

                    // TODO: internal error logging
                    let err = "Unable to interpret your request to create a deck. \
                        Please try again.".to_string();
                    return respond_json_with_error!(APIStatus::BadRequest; err; None);

                }
            };

            // NOTE: deck name must not be empty
            if request.name.trim().len() <= 0 {
                // TODO: internal error logging
                let err = "Deck name is required.".to_string();
                return respond_json_with_error!(APIStatus::BadRequest; err; None);
            }

            let _guard = context::write_lock(context.clone());

            match decks::create_deck(context.clone(), request) {
                Ok(new_deck) => {

                    // connect new deck as child of parent deck

                    match decks::connect_decks(context.clone(), new_deck.id, parent_deck_id) {
                        Ok(_) => {

                            let deck_route = DeckRoute::Decks(Default::default(), Default::default());
                            let app_route = AppRoute::Deck(new_deck.id, deck_route);

                            let response = DeckCreateResponse {
                                profile_url: view_route_to_link(context, app_route)
                            };

                            // TODO: remove
                            // let response = DeckResponse {
                            //     profile_url: view_route_to_link(context, app_route),
                            //     deck: new_deck,
                            //     has_parent: true,
                            //     parent_id: Some(parent_deck_id)
                            // };

                            return respond_json!(Some(response));
                        },
                        Err(_) => {

                            // TODO: internal error logging
                            return respond_json_with_error!(APIStatus::ServerError;
                                INTERNAL_SERVER_ERROR_STRING.clone(); None);
                        }
                    }

                },
                Err(_) => {

                    // TODO: internal error logging
                    return respond_json_with_error!(APIStatus::ServerError;
                        INTERNAL_SERVER_ERROR_STRING.clone(); None);

                }
            }

        },
        Err(err) => {
            // internal reason: invalid utf8 input

            // TODO: internal error logging
            let err = "Unable to interpret your request to create a deck. \
                Please try again.".to_string();
            return respond_json_with_error!(APIStatus::BadRequest; err; None);

        }
    }
}

#[inline]
fn parse_route_api_deck_new_card<'a>(
    input: Input<'a, u8>,
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    parent_deck_id: DeckID)
-> U8Result<'a, RenderResponse> {

    parse!{input;

        string_ignore_case(b"new");
        string_ignore_case(b"/");
        string_ignore_case(b"card");

        eof();

        ret {
            __parse_route_api_deck_new_card(context, request, parent_deck_id)
        }

    }
}

#[inline]
fn __parse_route_api_deck_new_card(
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    parent_deck_id: DeckID)
-> RenderResponse {

    let mut request = request.borrow_mut();

    if request.method != Method::Post {

        // TODO: internal error logging
        return respond_json_with_error!(APIStatus::MethodNotAllowed;
            API_METHOD_NOT_ALLOWED_STRING.clone(); None);
    }

    // POST /api/deck/:id/new/card => create a new card within this deck

    let mut buffer = String::new();

    match request.read_to_string(&mut buffer) {
        Ok(_num_bytes_parsed) => {

            let request: cards::CreateCard = match serde_json::from_str(&buffer) {
                Ok(request) => request,
                Err(err) => {
                    // TODO: internal error logging
                    return respond_json_with_error!(APIStatus::BadRequest;
                        "Unable to create a new card. Please try again.".to_string(); None);
                }
            };

            match request.validate() {
                None => {},
                Some(reason) => {

                    // TODO: code review this

                    // TODO: internal error logging
                    return respond_json_with_error!(APIStatus::BadRequest; reason; None);

                }
            }

            // TODO: refactor
            let mut request = request;
            if request.title.trim().len() <= 0 {
                // invariant: request.question is not empty

                // if card has question content, fetch first 140 characters as title

                let len = if request.question.len() < 140 {
                    request.question.len()
                } else {
                    140
                };

                let new_title = &(request.question)[..len];
                let new_title = new_title.trim().to_string();

                request.title = new_title;
            };
            let request = request;

            let _guard = context::write_lock(context.clone());

            match cards::create_card(context.clone(), parent_deck_id, request) {
                Ok(new_card) => {

                    let card_route = CardRoute::Contents;
                    let app_route = AppRoute::Deck(parent_deck_id,
                        DeckRoute::CardProfile(new_card.id, card_route));

                    let response = cards::CardCreateResponse {
                        profile_url: view_route_to_link(context, app_route)
                    };

                    return respond_json!(Some(response));

                },
                Err(_) => {
                    // TODO: internal error logging
                    return respond_json_with_error!(APIStatus::ServerError;
                        INTERNAL_SERVER_ERROR_STRING.clone(); None);
                }
            }

        },
        Err(err) => {
            // invalid utf8 input

            // TODO: internal error logging
            return respond_json_with_error!(APIStatus::BadRequest;
                "Unable to create a new card. Please try again.".to_string(); None);
        }
    }
}

#[inline]
fn parse_route_api_deck_description<'a>(
    input: Input<'a, u8>,
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID)
-> U8Result<'a, RenderResponse> {
    parse!{input;

        string_ignore_case(b"description");

        eof();

        ret {
            __parse_route_api_deck_description(context, request, deck_id)
        }
    }
}

#[inline]
fn __parse_route_api_deck_description(
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID) -> RenderResponse {

    let mut request = request.borrow_mut();

    if request.method != Method::Post {

        // TODO: internal error logging
        return respond_json_with_error!(APIStatus::MethodNotAllowed;
            API_METHOD_NOT_ALLOWED_STRING.clone(); None);
    }

    let mut buffer = String::new();

    match request.read_to_string(&mut buffer) {
        Err(err) => {
            // invalid utf8 input

            // TODO: internal error logging
            return respond_json_with_error!(APIStatus::BadRequest;
                "Unable to update deck description. Please try again.".to_string(); None);
        },
        Ok(_num_bytes_parsed) => {

            let request: UpdateDeckDescription = match serde_json::from_str(&buffer) {
                Ok(request) => request,
                Err(_err) => {

                    // TODO: internal error logging
                    return respond_json_with_error!(APIStatus::BadRequest;
                        "Unable to update deck description. Please try again.".to_string(); None);
                }
            };

            let _guard = context::write_lock(context.clone());

            match decks::update_deck_description(context.clone(), deck_id, request) {
                Err(_) => {
                    // TODO: internal error logging
                    return respond_json_with_error!(APIStatus::ServerError;
                        INTERNAL_SERVER_ERROR_STRING.clone(); None);
                },
                Ok(_) => {
                    return respond_json!(None);
                }
            }
        }
    }
}

#[inline]
fn parse_route_api_deck_settings<'a>(
    input: Input<'a, u8>,
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID)
-> U8Result<'a, RenderResponse> {
    parse!{input;

        string_ignore_case(b"settings");
        string_ignore_case(b"/");

        // TODO: other settings???
        let response = parse_route_api_deck_settings_name(context.clone(), request.clone(), deck_id);

        ret response
    }
}

#[inline]
fn parse_route_api_deck_settings_name<'a>(
    input: Input<'a, u8>,
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID)
-> U8Result<'a, RenderResponse> {
    parse!{input;

        string_ignore_case(b"name");
        eof();

        ret {
            __parse_route_api_deck_settings_name(context, request, deck_id)
        }
    }
}

#[inline]
fn __parse_route_api_deck_settings_name(
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID) -> RenderResponse {

    let mut request = request.borrow_mut();

    if request.method != Method::Post {

        // TODO: internal error logging
        return respond_json_with_error!(APIStatus::MethodNotAllowed;
            API_METHOD_NOT_ALLOWED_STRING.clone(); None);

    }

    let mut buffer = String::new();

    match request.read_to_string(&mut buffer) {
        Err(_why) => {
            // Possible reason: invalid utf8 input

            // TODO: internal error logging
            let err = "Unable to rename deck. Please try again.".to_string();
            return respond_json_with_error!(APIStatus::BadRequest; err; None);

        },
        Ok(_num_bytes_parsed) => {

            let request: UpdateDeckName = match serde_json::from_str(&buffer) {
                Ok(request) => request,
                Err(err) => {
                    // TODO: internal error logging
                    let err = "Unable to rename deck. Please try again.".to_string();
                    return respond_json_with_error!(APIStatus::BadRequest; err; None);
                }
            };

            // NOTE: deck name must not be empty
            if request.name.trim().len() <= 0 {

                // TODO: internal error logging
                let err = "Deck name must not be empty.".to_string();
                return respond_json_with_error!(APIStatus::BadRequest; err; None);

            }

            let _guard = context::write_lock(context.clone());

            match decks::update_deck_name(context.clone(), deck_id, request) {
                Err(_) => {

                    // TODO: internal error logging
                    let err = "An internal server error occurred.".to_string();
                    return respond_json_with_error!(APIStatus::ServerError; err; None);

                },
                Ok(_) => {
                    return respond_json!(None);
                }
            }
        }
    }
}

#[inline]
fn parse_route_deck<'a>(input: Input<'a, u8>, context: Rc<RefCell<Context>>, request: Rc<RefCell<Request>>)
-> U8Result<'a, RenderResponse> {
    (parse!{input;

        string_ignore_case(b"deck");
        parse_byte_limit(b'/', 5);
        let deck_id = decimal();

        ret deck_id

    }).bind(|i, deck_id| {

        let exists = {

            let _guard = context::read_lock(context.clone());

            match decks::deck_exists(context.clone(), deck_id) {
                Ok(exists) => exists,
                Err(_) => {
                    // TODO: internal error logging
                    return i.ret(RenderResponse::InternalServerError);
                }
            }

        };

        if exists {
            __parse_route_deck(i, context.clone(), request, deck_id)
        } else {
            i.ret(RenderResponse::NotFound)
        }


    })
}

#[inline]
fn __parse_route_deck<'a>(
    input: Input<'a, u8>,
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID) -> U8Result<'a, RenderResponse> {

    parse!{input;

        let render_response = or(|i| parse!{i;

            parse_byte_limit(b'/', 5);

            let render_response =
                // /review
                parse_route_deck_review(context.clone(), request.clone(), deck_id) <|>
                // /decks
                parse_route_deck_decks(context.clone(), request.clone(), deck_id) <|>
                // /cards
                parse_route_deck_cards(context.clone(), request.clone(), deck_id) <|>
                // /card/:id/*
                parse_route_deck_card_profile(context.clone(), request.clone(), deck_id) <|>
                // /new/deck
                parse_route_deck_new_deck(request.clone(), deck_id) <|>
                // /new/card
                parse_route_deck_new_card(request.clone(), deck_id) <|>
                // /description
                parse_route_deck_description(request.clone(), deck_id) <|>
                // /stats
                parse_route_deck_stats(request.clone(), deck_id) <|>
                // /settings
                parse_route_deck_settings(request.clone(), deck_id);


            ret render_response

        }, |i| parse!{i;

            option(|i| parse_byte_limit(i, b'/', 5), ());

            let query_string = option(|i| parse!{i;
                let query_string = parse_query_string();

                ret Some(query_string)
            }, None);

            eof();

            ret {
                route_deck_decks(context.clone(), request.clone(), deck_id, query_string)
            }

        });

        ret render_response
    }

}

#[inline]
fn parse_route_deck_stats<'a>(
    input: Input<'a, u8>,
    // NOTE: not needed
    // context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID) -> U8Result<'a, RenderResponse> {
    parse!{input;

        string_ignore_case(b"statistics") <|>
        string_ignore_case(b"stats");

        option(|i| parse_byte_limit(i, b'/', 5), ());

        or(
            |i| parse!{i;
                token(b'?');
                ret {()}
            },
            eof
        );

        ret {
            if request.borrow().method != Method::Get {
                RenderResponse::MethodNotAllowed
            } else {
                let route = AppRoute::Deck(deck_id, DeckRoute::Stats);
                RenderResponse::Component(route)
            }
        }
    }
}

#[inline]
fn parse_route_deck_review<'a>(
    input: Input<'a, u8>,
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID) -> U8Result<'a, RenderResponse> {
    parse!{input;

        string_ignore_case(b"review");

        option(|i| parse_byte_limit(i, b'/', 5), ());

        or(
            |i| parse!{i;
                token(b'?');
                ret {()}
            },
            eof
        );

        ret {
            __parse_route_deck_review(context, request, deck_id)
        }
    }
}

#[inline]
fn __parse_route_deck_review(
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID) -> RenderResponse {

    if request.borrow().method != Method::Get {
        return RenderResponse::MethodNotAllowed;
    }

    let _guard = context::write_lock(context.clone());

    let deck = handle_api_result_html!(decks::get_deck(context.clone(), deck_id));

    let deck_route = {
        let result = handle_api_result_html!(review::get_review_card(context, &deck));

        DeckRoute::Review(result)
    };

    let route = AppRoute::Deck(deck_id, deck_route);
    return RenderResponse::Component(route)

}

#[inline]
fn parse_route_deck_description<'a>(
    input: Input<'a, u8>,
    // NOTE: not needed
    // context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID) -> U8Result<'a, RenderResponse> {
    parse!{input;

        string_ignore_case(b"description");

        option(|i| parse_byte_limit(i, b'/', 5), ());

        or(
            |i| parse!{i;
                token(b'?');
                ret {()}
            },
            eof
        );

        ret {
            if request.borrow().method != Method::Get {
                RenderResponse::MethodNotAllowed
            } else {
                let route = AppRoute::Deck(deck_id, DeckRoute::Description);
                RenderResponse::Component(route)
            }
        }
    }
}

#[inline]
fn parse_route_deck_cards<'a>(
    input: Input<'a, u8>,
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID) -> U8Result<'a, RenderResponse> {

    parse!{input;

        string_ignore_case(b"cards");

        option(|i| parse_byte_limit(i, b'/', 5), ());

        let query_string = option(|i| parse!{i;
            let query_string = parse_query_string();

            ret Some(query_string)
        }, None);

        ret {
            route_deck_cards(context.clone(), request.clone(), deck_id, query_string)
        }

    }

}

#[inline]
fn route_deck_cards(
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID,
    query_string: Option<QueryString>) -> RenderResponse {

    if request.borrow().method != Method::Get {
        return RenderResponse::MethodNotAllowed;
    }

    let cards_route = match query_string {
        None => DeckRoute::Cards(Default::default(), Default::default()),
        Some(query_string) => {

            let page_query = CardsPageQuery::parse(&query_string, context.clone(), deck_id);
            let search = Search::parse(&query_string);

            DeckRoute::Cards(page_query, search)
        }
    };

    let decks = AppRoute::Deck(deck_id, cards_route);
    return RenderResponse::Component(decks);

}

#[inline]
fn parse_route_deck_card_profile<'a>(
    input: Input<'a, u8>,
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID) -> U8Result<'a, RenderResponse> {

    (parse!{input;

        string_ignore_case(b"card");
        parse_byte_limit(b'/', 5);
        let card_id = decimal();

        ret card_id

    }).bind(|i, card_id| {

        let _guard = context::read_lock(context.clone());

        match cards::is_card_in_deck(context.clone(), card_id, deck_id) {
            Ok(exists) => {

                if exists {
                    __parse_route_card(i, context.clone(), request, deck_id, card_id)
                } else {
                    // TODO: redirect to /card/:id
                    i.ret(RenderResponse::NotFound)
                }

            },
            // TODO: internal error logging
            Err(_) => i.ret(RenderResponse::InternalServerError)
        }

    })
}

#[inline]
fn __parse_route_card<'a>(
    input: Input<'a, u8>,
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID,
    card_id: CardID) -> U8Result<'a, RenderResponse> {
    parse!{input;

        let render_response = or(|i| parse!{i;

            parse_byte_limit(b'/', 5);

            let render_response =
                // /contents
                parse_route_card_contents(context.clone(), request.clone(), deck_id, card_id) <|>
                // /review
                parse_route_card_review(context.clone(), request.clone(), deck_id, card_id) <|>
                // /stats
                parse_route_card_stats(context.clone(), request.clone(), deck_id, card_id) <|>
                // /settings
                parse_route_card_settings(context.clone(), request.clone(), deck_id, card_id);


            ret render_response

        }, |i| parse!{i;

            option(|i| parse_byte_limit(i, b'/', 5), ());

            // TODO: simplify if not used
            let query_string = option(|i| parse!{i;
                let query_string = parse_query_string();

                ret Some(query_string)
            }, None);

            eof();

            ret {
                let route = AppRoute::Deck(deck_id, DeckRoute::CardProfile(card_id, CardRoute::Contents));
                RenderResponse::Component(route)
            }

        });

        ret render_response
    }
}

#[inline]
fn parse_route_card_contents<'a>(
    input: Input<'a, u8>,
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID,
    card_id: CardID) -> U8Result<'a, RenderResponse> {
    parse!{input;

        string_ignore_case(b"contents");

        option(|i| parse_byte_limit(i, b'/', 5), ());

        // TODO: simplify is query string is not consumed
        let query_string = option(|i| parse!{i;
            let query_string = parse_query_string();

            ret Some(query_string)
        }, None);


        ret {
            let route = AppRoute::Deck(deck_id, DeckRoute::CardProfile(card_id, CardRoute::Contents));
            RenderResponse::Component(route)
        }
    }
}

#[inline]
fn parse_route_card_review<'a>(
    input: Input<'a, u8>,
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID,
    card_id: CardID) -> U8Result<'a, RenderResponse> {
    parse!{input;

        string_ignore_case(b"review");

        option(|i| parse_byte_limit(i, b'/', 5), ());

        // TODO: simplify is query string is not consumed
        let query_string = option(|i| parse!{i;
            let query_string = parse_query_string();

            ret Some(query_string)
        }, None);

        ret {
            let route = AppRoute::Deck(deck_id, DeckRoute::CardProfile(card_id, CardRoute::Review));
            RenderResponse::Component(route)
        }
    }
}

#[inline]
fn parse_route_card_stats<'a>(
    input: Input<'a, u8>,
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID,
    card_id: CardID) -> U8Result<'a, RenderResponse> {
    parse!{input;

        string_ignore_case(b"stats");

        option(|i| parse_byte_limit(i, b'/', 5), ());

        // TODO: simplify is query string is not consumed
        let query_string = option(|i| parse!{i;
            let query_string = parse_query_string();

            ret Some(query_string)
        }, None);

        ret {
            let route = AppRoute::Deck(deck_id, DeckRoute::CardProfile(card_id, CardRoute::Stats));
            RenderResponse::Component(route)
        }
    }
}

#[inline]
fn parse_route_card_settings<'a>(
    input: Input<'a, u8>,
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID,
    card_id: CardID) -> U8Result<'a, RenderResponse> {
    parse!{input;

        string_ignore_case(b"settings");

        let response = parse_route_card_settings_move(context, request.clone(), deck_id, card_id) <|>
            parse_route_card_settings_main(request.clone(), deck_id, card_id);

        ret response

    }
}

#[inline]
fn parse_route_card_settings_main<'a>(
    input: Input<'a, u8>,
    // NOTE: not needed
    // context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID,
    card_id: CardID) -> U8Result<'a, RenderResponse> {
    parse!{input;

        option(|i| parse_byte_limit(i, b'/', 5), ());

        or(
            |i| parse!{i;
                token(b'?');
                ret {()}
            },
            eof
        );

        ret {
            if request.borrow().method != Method::Get {
                RenderResponse::MethodNotAllowed
            } else {

                let route = AppRoute::Deck(deck_id, DeckRoute::CardProfile(card_id,
                    CardRoute::Settings(CardSettings::Main)));

                RenderResponse::Component(route)
            }
        }
    }
}

#[inline]
fn parse_route_card_settings_move<'a>(
    input: Input<'a, u8>,
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID,
    card_id: CardID) -> U8Result<'a, RenderResponse> {
    parse!{input;

        // TODO: place this here?
        parse_byte_limit(b'/', 5);

        string_ignore_case(b"move");

        option(|i| parse_byte_limit(i, b'/', 5), ());

        let query_string = option(|i| parse!{i;
            let query_string = parse_query_string();

            ret Some(query_string)
        }, None);

        eof();

        ret {
            if request.borrow().method != Method::Get {
                RenderResponse::MethodNotAllowed
            } else {

                let card_settings_move = match query_string {
                    None => CardSettings::Move(MoveDecksPageQuery::default(context.clone(), card_id),
                        Default::default()),
                    Some(ref query_string) => {

                        let page_query = MoveDecksPageQuery::parse(query_string, context.clone(), card_id);
                        let search = Search::parse(query_string);

                        CardSettings::Move(page_query, search)
                    }
                };

                let route = AppRoute::Deck(deck_id, DeckRoute::CardProfile(card_id,
                    CardRoute::Settings(card_settings_move)));

                RenderResponse::Component(route)
            }
        }
    }
}

#[inline]
fn parse_route_deck_settings<'a>(
    input: Input<'a, u8>,
    // NOTE: not needed
    // context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID) -> U8Result<'a, RenderResponse> {
    parse!{input;

        string_ignore_case(b"settings");

        let response = parse_route_deck_settings_move(request.clone(), deck_id) <|>
            parse_route_deck_settings_main(request.clone(), deck_id);

        ret response

    }
}

#[inline]
fn parse_route_deck_settings_main<'a>(
    input: Input<'a, u8>,
    // NOTE: not needed
    // context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID) -> U8Result<'a, RenderResponse> {
    parse!{input;

        option(|i| parse_byte_limit(i, b'/', 5), ());

        or(
            |i| parse!{i;
                token(b'?');
                ret {()}
            },
            eof
        );

        ret {
            if request.borrow().method != Method::Get {
                RenderResponse::MethodNotAllowed
            } else {
                let route = AppRoute::Deck(deck_id, DeckRoute::Settings(DeckSettings::Main));
                RenderResponse::Component(route)
            }
        }
    }
}

#[inline]
fn parse_route_deck_settings_move<'a>(
    input: Input<'a, u8>,
    // NOTE: not needed
    // context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID) -> U8Result<'a, RenderResponse> {
    parse!{input;

        // TODO: place this here?
        parse_byte_limit(b'/', 5);

        string_ignore_case(b"move");

        option(|i| parse_byte_limit(i, b'/', 5), ());

        let query_string = option(|i| parse!{i;
            let query_string = parse_query_string();

            ret Some(query_string)
        }, None);

        eof();

        ret {
            if request.borrow().method != Method::Get {
                RenderResponse::MethodNotAllowed
            } else {
                let route = AppRoute::Deck(deck_id, DeckRoute::Settings(DeckSettings::Move));
                RenderResponse::Component(route)
            }
        }
    }
}

#[inline]
fn parse_route_deck_decks<'a>(input: Input<'a, u8>, context: Rc<RefCell<Context>>, request: Rc<RefCell<Request>>, deck_id: DeckID)
-> U8Result<'a, RenderResponse> {
    parse!{input;

        string_ignore_case(b"decks");

        option(|i| parse_byte_limit(i, b'/', 5), ());

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
fn route_deck_decks(
    context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID,
    query_string: Option<QueryString>) -> RenderResponse {

    if request.borrow().method != Method::Get {
        return RenderResponse::MethodNotAllowed;
    }

    let decks_route = match query_string {
        None => DeckRoute::Decks(Default::default(), Default::default()),
        Some(query_string) => {

            let page_query = DecksPageQuery::parse(&query_string, context.clone(), deck_id);
            let search = Search::parse(&query_string);

            DeckRoute::Decks(page_query, search)
        }
    };

    let decks = AppRoute::Deck(deck_id, decks_route);
    return RenderResponse::Component(decks);

}

#[inline]
fn parse_route_deck_new_deck<'a>(
    input: Input<'a, u8>,
    // NOTE: not needed
    // context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID) -> U8Result<'a, RenderResponse> {
    parse!{input;

        string_ignore_case(b"new");
        parse_byte_limit(b'/', 5);
        string_ignore_case(b"deck");

        option(|i| parse_byte_limit(i, b'/', 5), ());

        or(
            |i| parse!{i;
                token(b'?');
                ret {()}
            },
            eof
        );

        ret {
            // Allow only GET requests

            if request.borrow().method != Method::Get {
                RenderResponse::MethodNotAllowed
            } else {
                let route = AppRoute::Deck(deck_id, DeckRoute::NewDeck);
                RenderResponse::Component(route)
            }
        }
    }
}

#[inline]
fn parse_route_deck_new_card<'a>(
    input: Input<'a, u8>,
    // NOTE: not needed
    // context: Rc<RefCell<Context>>,
    request: Rc<RefCell<Request>>,
    deck_id: DeckID) -> U8Result<'a, RenderResponse> {
    parse!{input;

        string_ignore_case(b"new");
        parse_byte_limit(b'/', 5);
        string_ignore_case(b"card");

        option(|i| parse_byte_limit(i, b'/', 5), ());

        or(
            |i| parse!{i;
                token(b'?');
                ret {()}
            },
            eof
        );

        ret {
            // Allow only GET requests

            if request.borrow().method != Method::Get {
                RenderResponse::MethodNotAllowed
            } else {
                let route = AppRoute::Deck(deck_id, DeckRoute::NewCard);
                RenderResponse::Component(route)
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
        RenderResponse::Component(app_route) => {
            render_components(context, app_route, response);
        },
        RenderResponse::JSON(api_status, json_response) => {

            *response.status_mut() = api_status.status_code();

            response.headers_mut().set((hyper::header::ContentType(
                mime!(Application/Json)
            )));

            let json_response = serde_json::to_string(&json_response).unwrap();

            response.send(json_response.as_bytes()).unwrap();

        },
        RenderResponse::BadRequest => {
            // TODO: better page
            let message = format!("400 Bad Request");
            *response.status_mut() = StatusCode::BadRequest;
            response.send(message.as_bytes()).unwrap();
        },
        RenderResponse::NotFound => {
            // TODO: better 404 page
            let message = format!("404 Not Found");
            *response.status_mut() = StatusCode::NotFound;
            response.send(message.as_bytes()).unwrap();
        },
        RenderResponse::InternalServerError => {
            // TODO: better 500 page
            let message = format!("500 Internal Server Error");
            *response.status_mut() = StatusCode::NotFound;
            response.send(message.as_bytes()).unwrap();
        },
        RenderResponse::Asset(header, content) => {
            response.headers_mut().set((header));
            response.send(&content).unwrap();
        },
        RenderResponse::MethodNotAllowed => {
            *response.status_mut() = StatusCode::MethodNotAllowed;
            let message = format!("{}", StatusCode::MethodNotAllowed);
            response.send(message.as_bytes()).unwrap();
        }
    }


}

#[inline]
fn render_components(context: Rc<RefCell<Context>>, app_route: AppRoute, mut response: Response) {

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

        render_response(context.clone(), RenderResponse::InternalServerError, response);
        return;
    }

    match result.ok().unwrap() {
        Err(why) => {
            println!("ERROR RENDERING: {:?}", why);

            // TODO: internal error logging

            render_response(context.clone(), RenderResponse::InternalServerError, response);
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
