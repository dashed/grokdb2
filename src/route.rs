#[macro_use]
pub mod helpers {

    /* 3rd-party imports */

    use hyper::server::{Server, Handler, Request, Response};
    use hyper::header::{Headers, ContentType, TransferEncoding};

    use templates::{RenderOnce, TemplateBuffer, Template, FnRenderer};

    /* local imports */

    use contexts::Context;
    use super::constants::{AppRoute, DeckRoute, CardRoute};
    use components::{AppComponent};
    use super::manager::{RouterFn, LinkGenerator};

    ////////////////////////////////////////////////////////////////////////////

    pub fn render_app_component(
        context: Context,
        app_component_title: String,
        request: Request,
        response: Response) {



        let mut response = response;

        response.headers_mut().set((ContentType(
            mime!(Text/Html)
        )));

        // We lock the API for reads only
        api_read_lock!(_api_guard; context.global_context.db_connection);

        let app_component = FnRenderer::new(|tmpl| {
            AppComponent(tmpl, &context, app_component_title);
        });

        match app_component.into_string() {
            Err(why) => {
                println!("ERROR RENDERING: {:?}", why);

                // TODO: fix
                super::routes::internal_server_error(&context, request, response);

                return;
            },
            Ok(rendered) => {
                response.send(rendered.as_bytes()).unwrap();

                return;
            }
        };


        /////

        // let mut stream = response.start().unwrap();
        // app_component.write_to_io(&mut stream)
        //     .unwrap();

        // stream.end();

        ///////

        // // We lock the API for reads only
        // api_read_lock!(_api_guard; context.global_context.db_connection);

        // let app_component = FnRenderer::new(|tmpl| {
        //     AppComponent(tmpl, &context, app_component_title);
        // });

        // let mut stream = response.start().unwrap();
        // app_component.write_to_io(&mut stream)
        //     .unwrap();

        // stream.end();

        /////

        // let opts = ParseOpts {
        //     tree_builder: TreeBuilderOpts {
        //         drop_doctype: true,
        //         ..Default::default()
        //     },
        //     ..Default::default()
        // };

        // // let stdin = io::stdin();

        // let foo = app_component.into_string().unwrap();
        // let mut foo = foo.as_bytes();

        // let dom = parse_document(RcDom::default(), opts)
        //     .from_utf8()
        //     .read_from(&mut foo)
        //     // .read_from(&mut stdin.lock())
        //     .unwrap();

        // let mut stream = response.start().unwrap();

        // // The validator.nu HTML2HTML always prints a doctype at the very beginning.
        // io::stdout().write_all(b"<!DOCTYPE html>\n")
        //     .ok().expect("writing DOCTYPE failed");
        // serialize(&mut stream, &dom.document, Default::default())
        //     .ok().expect("serialization failed");

    }


    pub fn get_route_tuple(view_route: AppRoute) -> (&'static str, RouterFn, LinkGenerator) {

        match view_route {

            AppRoute::Home => (r"^/$", super::routes::root, super::link::root),

            AppRoute::Settings => (r"^/settings$", super::routes::settings, super::link::settings),

            AppRoute::Stashes => (r"^/stashes$", super::routes::stashes, super::link::stashes),

            AppRoute::Card(_, card_route) => {

                match card_route {
                    CardRoute::Profile => {
                        (
                            r"^/card/(?P<card_id>[1-9][0-9]*)$",
                            super::routes::card_profile,
                            super::link::card_profile
                        )
                    },
                    CardRoute::Review => {
                        (
                            r"^/card/(?P<card_id>[1-9][0-9]*)/review$",
                            super::routes::card_profile_review,
                            super::link::card_profile_review
                        )
                    },
                }
            },

            AppRoute::CardInDeck(_, _, card_route) => {

                match card_route {
                    CardRoute::Profile => {
                        (
                            r"^/deck/(?P<deck_id>[1-9][0-9]*)/card/(?P<card_id>[1-9][0-9]*)$",
                            super::routes::deck_card_profile,
                            super::link::deck_card_profile
                        )
                    },
                    CardRoute::Review => {
                        (
                            r"^/deck/(?P<deck_id>[1-9][0-9]*)/card/(?P<card_id>[1-9][0-9]*)/review$",
                            super::routes::deck_card_profile_review,
                            super::link::deck_card_profile_review
                        )
                    },
                }
            },

            AppRoute::Deck(_, deck_route) => {

                match deck_route {
                    DeckRoute::NewCard=> (
                        r"^/deck/(?P<deck_id>[1-9][0-9]*)/new/card$",
                        super::routes::new_card,
                        super::link::new_card),

                    DeckRoute::NewDeck=> (
                        r"^/deck/(?P<deck_id>[1-9][0-9]*)/new/deck$",
                        super::routes::new_deck,
                        super::link::new_deck),

                    DeckRoute::Description => (
                        r"^/deck/(?P<deck_id>[1-9][0-9]*)/description$",
                        super::routes::deck_description,
                        super::link::deck_description),

                    DeckRoute::Decks => (
                        r"^/deck/(?P<deck_id>[1-9][0-9]*)/decks$",
                        super::routes::deck_decks,
                        super::link::deck_decks),

                    DeckRoute::Cards => (
                        r"^/deck/(?P<deck_id>[1-9][0-9]*)/cards$",
                        super::routes::deck_cards,
                        super::link::deck_cards),

                    DeckRoute::Meta => (
                        r"^/deck/(?P<deck_id>[1-9][0-9]*)/meta$",
                        super::routes::deck_meta,
                        super::link::deck_meta),

                    DeckRoute::Settings => (
                        r"^/deck/(?P<deck_id>[1-9][0-9]*)/settings$",
                        super::routes::deck_settings,
                        super::link::deck_settings),

                    DeckRoute::Review => (
                        r"^/deck/(?P<deck_id>[1-9][0-9]*)/review$",
                        super::routes::deck_review,
                        super::link::deck_review),
                }
            }
        }

    }

    fn get_route_regex(view_route: AppRoute) -> &'static str {
        let (regex_matcher, _, _) = get_route_tuple(view_route);
        regex_matcher
    }

    pub fn view_route_to_link(view_route_destination: AppRoute, context: &Context) -> String {
        let (_, _, link_generator) = get_route_tuple(view_route_destination);
        link_generator(view_route_destination, context)
    }

    // helper macro to attach get_route_tuple(...) to route manager
    macro_rules! route(
        ($router: expr, $method: ident, $view_route: expr) => (
            let (regex_matcher, route_handler, _) = route::helpers::get_route_tuple($view_route);
            $router.add_route(Method::$method, regex_matcher, route_handler);
        )
    );

    // utility macro for extracting named capture and parsing it into the appropriate type
    macro_rules! parse_capture(
        ($captures: expr, $capture_name: expr, $to_type: ident) => (
            $captures
                .as_ref() // prevent move
                .unwrap()
                .name($capture_name)
                .unwrap()
                .parse::<$to_type>()
                .unwrap() // provided regex should ensure the captured group should convert to $to_type seamlessly
        )
    );

}



pub mod constants {
    /* component route constants */

    // TODO: change to u64. it's i64 b/c sqlite requires it.
    pub type DeckID = i64;
    pub type CardID = i64;
    pub type StashID = i64;

    #[derive(Debug, Copy, Clone)]
    pub enum AppRoute {

        Home,

        // user settings
        Settings,

        Stashes,

        Deck(DeckID, DeckRoute),

        Card(CardID, CardRoute),
        CardInDeck(DeckID, CardID, CardRoute)
    }

    #[derive(Debug, Copy, Clone)]
    pub enum CardRoute {
        Profile,
        // Settings,
        // Meta,

        Review
    }

    #[derive(Debug, Copy, Clone)]
    pub enum DeckRoute {

        NewCard,
        NewDeck,
        Description,
        Decks, // list
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
}

mod link {

    /* local imports */

    use contexts::Context;
    use super::constants::{AppRoute, CardRoute};

    ////////////////////////////////////////////////////////////////////////////

    pub fn root(view_route_destination: AppRoute, context: &Context) -> String {
        format!("/")
    }

    pub fn settings(view_route_destination: AppRoute, context: &Context) -> String {
        format!("/settings")
    }

    pub fn stashes(view_route_destination: AppRoute, context: &Context) -> String {
        format!("/stashes")
    }

    pub fn new_deck(view_route_destination: AppRoute, _context: &Context) -> String {

        match view_route_destination {
            AppRoute::Deck(deck_id, DeckRoute::NewDeck) => {
                return format!("/deck/{}/new/deck", deck_id);
            },
            _ => {
                unreachable!();
            }
        }

    }

    pub fn new_card(view_route_destination: AppRoute, context: &Context) -> String {

        match view_route_destination {
            AppRoute::Deck(deck_id, DeckRoute::NewCard) => {
                return format!("/deck/{}/new/card", deck_id);
            },
            _ => {
                unreachable!();
            }
        }

    }


    pub fn deck_description(view_route_destination: AppRoute, context: &Context) -> String {

        match view_route_destination {
            AppRoute::Deck(deck_id, DeckRoute::Description) => {
                return format!("/deck/{}/description", deck_id);
            },
            _ => {
                unreachable!();
            }
        }

    }

    pub fn deck_decks(view_route_destination: AppRoute, context: &Context) -> String {

        match view_route_destination {
            AppRoute::Deck(deck_id, DeckRoute::Decks) => {
                return format!("/deck/{}/decks", deck_id);
            },
            _ => {
                unreachable!();
            }
        }

    }

    pub fn deck_cards(view_route_destination: AppRoute, context: &Context) -> String {

        match view_route_destination {
            AppRoute::Deck(deck_id, DeckRoute::Cards) => {
                return format!("/deck/{}/cards", deck_id);
            },
            _ => {
                unreachable!();
            }
        }

    }

    pub fn deck_meta(view_route_destination: AppRoute, context: &Context) -> String {

        match view_route_destination {
            AppRoute::Deck(deck_id, DeckRoute::Meta) => {
                return format!("/deck/{}/meta", deck_id);
            },
            _ => {
                unreachable!();
            }
        }

    }

    pub fn deck_settings(view_route_destination: AppRoute, context: &Context) -> String {

        match view_route_destination {
            AppRoute::Deck(deck_id, DeckRoute::Settings) => {
                return format!("/deck/{}/settings", deck_id);
            },
            _ => {
                unreachable!();
            }
        }

    }


    pub fn deck_review(view_route_destination: AppRoute, context: &Context) -> String {

        match view_route_destination {
            AppRoute::Deck(deck_id, DeckRoute::Review) => {
                return format!("/deck/{}/review", deck_id);
            },
            _ => {
                unreachable!();
            }
        }

    }

    pub fn deck_card_profile(view_route_destination: AppRoute, context: &Context) -> String {

        match view_route_destination {

            AppRoute::Card(card_id, CardRoute::Profile) => {
                return format!("/card/{}", card_id);
            },

            AppRoute::CardInDeck(deck_id, card_id, CardRoute::Profile) => {
                return format!("/deck/{}/card/{}", deck_id, card_id);
            },
            _ => {
                unreachable!();
            }
        }

    }

    pub fn deck_card_profile_review(view_route_destination: AppRoute, context: &Context) -> String {

        // TODO: fetch deck_id

        format!("/deck/1/card/1/review")
    }

    pub fn card_profile(view_route_destination: AppRoute, context: &Context) -> String {

        // TODO: fetch card_Id

        format!("/card/1")
    }

    pub fn card_profile_review(view_route_destination: AppRoute, context: &Context) -> String {

        // TODO: fetch card_Id

        format!("/card/1/review")
    }
}

// These are akin to controllers.
pub mod routes {

    /* rust lib imports */

    use std::panic::{self, AssertUnwindSafe};
    use std::fs;
    use std::fs::{File};
    use std::io;
    use std::path::{PathBuf, Path};
    use std::ffi::OsStr;

    /* 3rd-party imports */

    use hyper;
    use hyper::method::Method;
    use hyper::server::{Server, Handler, Request, Response};
    use hyper::header::{Headers, ContentType, TransferEncoding};
    use hyper::mime::{Mime, TopLevel, SubLevel};
    use hyper::uri::RequestUri;
    use hyper::status::StatusCode;

    use mime_types;

    use url::percent_encoding::percent_decode;

    /* local imports */

    use contexts::Context;
    use super::helpers::render_app_component;
    use super::constants::{AppRoute, DeckRoute, CardRoute, DeckID};

    lazy_static! {
        static ref MIME_TYPES: mime_types::Types = mime_types::Types::new().unwrap();
    }

    ////////////////////////////////////////////////////////////////////////////

    pub fn internal_server_error(mut context: &Context, request: Request, mut response: Response) {

        // let mut context = context;

        let message = format!("Internal server error for {}", request.uri);

        // 500 status code
        *response.status_mut() = StatusCode::InternalServerError;

        response.send(message.as_bytes()).unwrap();
    }


    pub fn not_found(mut context: Context, request: Request, mut response: Response) {

        // let mut context = context;

        let message = format!("No route handler found for {}", request.uri);

        // 404 status code
        *response.status_mut() = StatusCode::NotFound;

        response.send(message.as_bytes()).unwrap();
    }


    pub fn static_assets(context: Context, request: Request, response: Response) {

        // TODO: caching stuff from https://github.com/iron/staticfile/blob/master/src/static_handler.rs

        let req_path_raw = {
            let capture = context.captures.as_ref().unwrap();
            capture.name("path").unwrap().clone()
        };

        // let req_path = Path::new(&req_path);

        // URL decode
        let decoded_req_path = Path::new(&req_path_raw).iter().map(decode_percents);

        let mut req_path = context.global_context.assets_root_path.to_path_buf();
        req_path.extend(decoded_req_path);
        let req_path: PathBuf = req_path;

        let metadata = match fs::metadata(&req_path) {
            Ok(meta) => meta,
            Err(e) => {

                // TODO: flesh out
                not_found(context, request, response);
                return;


                // let status = match e.kind() {
                //     io::ErrorKind::NotFound => status::NotFound,
                //     io::ErrorKind::PermissionDenied => status::Forbidden,
                //     _ => status::InternalServerError,
                // };

                // return Err(IronError::new(e, status))
            },
        };

        if !metadata.is_file() {
            not_found(context, request, response);
            return;
        }

        let path_str = format!("{}", &req_path.to_string_lossy());

        let mut response = response;

        // Set the content type based on the file extension
        let mime_str = MIME_TYPES.mime_for_path(req_path.as_path());
        let _ = mime_str.parse().map(|mime: Mime|
            response.headers_mut().set((ContentType(mime)))
        );

        let mut file = File::open(req_path)
            .ok()
            .expect(&format!("No such file: {:?}", path_str));


        if let Ok(metadata) = file.metadata() {
            response.headers_mut().set(hyper::header::ContentLength(metadata.len()));
        }

        let mut stream = response.start().unwrap();

        // TODO: clean up
        io::copy(&mut file, &mut stream).unwrap();

        stream.end();
    }

    // Path: /
    pub fn root(mut context: Context, request: Request, response: Response) {

        let root_deck_id = context.global_context.root_deck_id;

        // enclose_panicable!(request; response; {
        //     // code
        // });

        // /////

        // let result = panic::catch_unwind(AssertUnwindSafe(|| {
        //     // TODO: debug
        //     let _deck_id = parse_capture!(context.captures, "deck_id", DeckID);
        // }));

        // if result.is_err() {
        //     let message = format!("Internal server error for {}", request.uri);

        //     let mut response = response;

        //     // 500 status code
        //     *response.status_mut() = StatusCode::InternalServerError;

        //     response.send(message.as_bytes()).unwrap();
        //     return;
        // }

        context.view_route = AppRoute::Deck(root_deck_id, DeckRoute::Cards);

        render_app_component(context, format!("grokdb"), request, response);
    }


    pub fn stashes(mut context: Context, request: Request, response: Response) {

        context.view_route = AppRoute::Stashes;

        render_app_component(context, format!("grokdb"), request, response);
    }

    pub fn settings(mut context: Context, request: Request, response: Response) {

        context.view_route = AppRoute::Settings;

        render_app_component(context, format!("grokdb"), request, response);
    }

    pub fn new_card(mut context: Context, request: Request, response: Response) {

        // /deck/:deck_id/new/card

        let deck_id = parse_capture!(&context.captures, "deck_id", DeckID);

        context.view_route = AppRoute::Deck(deck_id, DeckRoute::NewCard);

        render_app_component(context, format!("grokdb"), request, response);
    }

    pub fn new_deck(mut context: Context, request: Request, response: Response) {

        // /deck/:deck_id/new/deck

        let deck_id = parse_capture!(&context.captures, "deck_id", DeckID);

        context.view_route = AppRoute::Deck(deck_id, DeckRoute::NewDeck);

        render_app_component(context, format!("grokdb"), request, response);
    }

    pub fn deck_description(mut context: Context, request: Request, response: Response) {

        // /deck/:deck_id/description

        let deck_id = parse_capture!(context.captures, "deck_id", DeckID);

        context.view_route = AppRoute::Deck(deck_id, DeckRoute::Description);

        render_app_component(context, format!("grokdb"), request, response);
    }

    pub fn deck_decks(mut context: Context, request: Request, response: Response) {

        // /deck/:deck_id/decks

        let deck_id = parse_capture!(context.captures, "deck_id", DeckID);

        context.view_route = AppRoute::Deck(deck_id, DeckRoute::Decks);

        render_app_component(context, format!("grokdb"), request, response);
    }

    pub fn deck_cards(mut context: Context, request: Request, response: Response) {

        // /deck/:deck_id/cards

        let deck_id = parse_capture!(context.captures, "deck_id", DeckID);

        context.view_route = AppRoute::Deck(deck_id, DeckRoute::Cards);

        render_app_component(context, format!("grokdb"), request, response);
    }

    pub fn deck_meta(mut context: Context, request: Request, response: Response) {

        // /deck/:deck_id/meta

        let deck_id = parse_capture!(context.captures, "deck_id", DeckID);

        context.view_route = AppRoute::Deck(deck_id, DeckRoute::Meta);

        render_app_component(context, format!("grokdb"), request, response);
    }

    pub fn deck_settings(mut context: Context, request: Request, response: Response) {

        // /deck/:deck_id/settings

        let deck_id = parse_capture!(context.captures, "deck_id", DeckID);

        context.view_route = AppRoute::Deck(deck_id, DeckRoute::Settings);

        render_app_component(context, format!("grokdb"), request, response);
    }

    pub fn deck_review(mut context: Context, request: Request, response: Response) {

        // /deck/:deck_id/review

        let deck_id = parse_capture!(context.captures, "deck_id", DeckID);

        context.view_route = AppRoute::Deck(deck_id, DeckRoute::Review);

        render_app_component(context, format!("grokdb"), request, response);
    }

    pub fn deck_card_profile(mut context: Context, request: Request, response: Response) {

        // TODO: fetch deck_id and card_id
        context.view_route = AppRoute::CardInDeck(1, 1, CardRoute::Profile);

        render_app_component(context, format!("grokdb"), request, response);
    }

    pub fn deck_card_profile_review(mut context: Context, request: Request, response: Response) {

        // TODO: fetch deck_id and card_id
        context.view_route = AppRoute::CardInDeck(1, 1, CardRoute::Review);

        render_app_component(context, format!("grokdb"), request, response);
    }

    pub fn card_profile(mut context: Context, request: Request, response: Response) {

        // TODO: fetch card_id
        context.view_route = AppRoute::Card(1, CardRoute::Profile);

        render_app_component(context, format!("grokdb"), request, response);
    }

    pub fn card_profile_review(mut context: Context, request: Request, response: Response) {

        // TODO: fetch card_id
        context.view_route = AppRoute::Card(1, CardRoute::Review);

        render_app_component(context, format!("grokdb"), request, response);
    }


    /* helpers */



    #[inline]
    fn decode_percents(string: &OsStr) -> String {

        let string = format!("{}", string.to_string_lossy());

        format!("{}", percent_decode(string.as_bytes()).decode_utf8_lossy())

        // String::from_utf8(.if_any().unwrap()).unwrap()
        // OsStr::new(&token)
    }

    // // Path: /deck/:deck_id/view/cards
    // fn route_deck_cards(mut context: Context, request: Request, response: Response) {



    //     // lock database for read operation
    //     // let db_op_lock = context.global_context.db_connection.read().unwrap();
    //     // let db_lock = db_op_lock.lock().unwrap();
    //     db_read_lock!(context.global_context.db_connection);

    //     // TODO: rendering
    // }

}

pub mod manager {
    // Source: https://github.com/gsquire/reroute

    /* rust lib imports */

    use std::collections::HashMap;

    /* 3rd-party imports */

    use regex::{Regex, RegexSet};
    use regex::{Captures};

    use hyper::method::Method;
    use hyper::server::{Server, Handler, Request, Response};

    /* local imports */

    use contexts::{GlobalContext, Context};
    use super::constants::AppRoute;

    ////////////////////////////////////////////////////////////////////////////

    pub type RouterFn = fn(Context, Request, Response);

    // TODO: flesh out this docs
    // fn(view_route_destination: AppRoute, context: &Context) -> String
    //
    // view_route_destination := TBA
    // context := access to db and env variables
    //
    // returns string for href attribute of anchor tag
    pub type LinkGenerator = fn(AppRoute, &Context) -> String;

    #[derive(PartialEq, Eq, Hash)]
    struct RouteInfo {
        // route: String,

        // fast path
        route_map_idx: usize,
        verb: Method
    }

    /// The Router struct contains the information for your app to route requests
    /// properly based on their HTTP method and matching route. It allows the use
    /// of a custom 404 handler if desired but provides a default as well.
    ///
    /// Under the hood a Router uses a RegexSet to match URI's that come in to the
    /// instance of the hyper server. Because of this, it has the potential to match
    /// multiple patterns that you provide. It will call the first handler that it
    /// matches against so order matters.
    pub struct Router {
        /// A custom 404 handler that you can provide.
        // pub not_found: Option<RouterFn>,

        routes: Option<RegexSet>,
        route_list: Vec<String>,
        compiled_list: Vec<Regex>,
        route_map: HashMap<RouteInfo, RouterFn>
    }

    impl Router {

        pub fn new() -> Self {
            Router {
                routes: None,
                route_list: Vec::new(),
                compiled_list: Vec::new(),
                route_map: HashMap::new(),
            }
        }

        pub fn finalize(&mut self) {
            if self.route_list.len() == 0  {
                panic!("Too few routes");
            }

            let re_routes = RegexSet::new(self.route_list.iter());
            match re_routes {
                Ok(r) => {
                    self.routes = Some(r);
                }
                Err(err) => {
                    panic!("RegexSet::new: {:?}", err);
                }
            }
        }


        // The handle method for the router simply tries to match the URI against
        // the first pattern that it can which in turn calls its associated handle
        // function passing the hyper Request and Response structures.
        //
        // Returning None implies no route handler found.
        pub fn handle<'a, 'b>(&'a self, context: &mut Context<'a, 'b>, request: &Request) -> Option<RouterFn> {

            // TODO: remove
            // let uri = format!("{}", request.uri);

            let matches = self.routes.as_ref().unwrap().matches(&context.uri);
            let index = matches.iter().next();

            match index {
                Some(matched_idx) => {

                    // let route = &self.route_list[i];
                    let route_info = RouteInfo {
                        route_map_idx: matched_idx,
                        verb: request.method.clone()
                    };

                    let handler = self.route_map.get(&route_info);

                    match handler {
                        Some(h) => {

                            let compiled_pattern = &self.compiled_list[matched_idx];

                            // let mut context = context;

                            let captures = compiled_pattern.captures(&(context.uri));

                            // guaranteed to unwrap
                            let captures = captures.unwrap();

                            context.captures = Some(captures);

                            // let captures = self.get_captures(compiled_pattern, &uri).unwrap();

                            return Some(*h);

                            // return Some((*h, captures));
                            // let compiled_pattern = &self.compiled_list[i];
                            // let captures = get_captures(compiled_pattern, &uri);
                            // h(&self.global_context, req, res, captures);
                        }
                        None => {
                            return None;
                            // TODO: remove
                            // not_allowed(req, res);
                        }
                    }
                },
                // There is no point in passing captures to a route handler that
                // wasn't found.
                None => {
                    return None;
                }
            }

            return None;
        }

        /// Add a route to the router and give it a function to call when the route
        /// is matched against. You can call this explicitly or use the convenience
        /// methods defined below.
        pub fn add_route(&mut self, verb: Method, route: &str, handler: RouterFn) {

            let route_info = RouteInfo{
                // route: route.to_string(),
                route_map_idx: self.route_map.len(),
                verb: verb
            };

            match Regex::new(route) {
                Ok(p) => {
                    self.compiled_list.push(p);
                },
                Err(e) => {
                    panic!("Not adding this route due to error: {}", e);
                }
            }

            self.route_list.push(route.to_string());
            self.route_map.insert(route_info, handler);
        }

        // A convenience method for GET requests.
        pub fn get(&mut self, route: &str, handler: RouterFn) {
            self.add_route(Method::Get, route, handler);
        }

        // A convenience method for GET requests.
        pub fn post(&mut self, route: &str, handler: RouterFn) {
            self.add_route(Method::Post, route, handler);
        }


    }

}

