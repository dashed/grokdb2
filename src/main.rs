#![feature(macro_rules)]
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
use std::default::Default;

/* 3rd-party imports */

use url::percent_encoding::percent_decode;

use regex::{Regex, RegexSet};
use regex::{Captures};

use templates::{RenderOnce, TemplateBuffer, Template};

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

static PHRASE: &'static [u8] = b"Hello World!";

lazy_static! {
    static ref MIME_TYPES: mime_types::Types = mime_types::Types::new().unwrap();
}

/**

// TODO: credit, copyright, license, etc
List of projects whose code i've been copying & pasting from:
- https://github.com/iron/iron/pull/291/files
- reroute

reference: https://github.com/joshbuchea/HEAD#link-elements

**/

/* global macros */

// TODO: something here????


/* database */

macro_rules! db_read_lock(
    ($e:expr) => (

        // match $e.global_context.db_connection.read() {
        //     Ok(db_op_lock) => {
        //         db_op_lock
        //         // match db_op_lock.lock() {
        //         //     Ok(db_lock) => db_lock,
        //         //     Err(why) => {
        //         //         panic!("db_read_lock/RwLock/Mutex {:?}", why);
        //         //     }
        //         // }
        //     },
        //     Err(why) => {
        //         panic!("db_read_lock/RwLock {:?}", why);
        //     }
        // }.lock().unwrap()


        // let db_op_lock = $e.global_context.db_connection.read().unwrap();
        // let db_lock = db_op_lock.lock().unwrap();

        {
            // hacky type checking
            let _: Arc<RwLock<Mutex<Connection>>> = $e;
        };

        let db_op_lock = $e.read().unwrap();
        let _ = db_op_lock.lock().unwrap();


        // db_op_lock.lock().unwrap()



        // db_lock
        // match $e {
        //     Ok(v) => v,
        //     Err(e) => { println!("Error: {}", e); return; }
        // }
    )
);

macro_rules! db_write_lock(
    ($e:expr) => (

        {
            // hacky type checking
            let _: Arc<RwLock<Mutex<Connection>>> = $e;
        };

        let db_op_lock = $e.write().unwrap();
        let _ = db_op_lock.lock().unwrap();

    )
);

/* contexts */

struct GlobalContext<'a> {
    assets_root_path: &'a Path,

    // RwLock => ORM operations read/write lock
    // Mutex => database low-level lock
    db_connection: Arc<RwLock<Mutex<Connection>>>
    // db_ops_lock: Arc<RwLock<bool>>
}

// struct Context<'a, 'b, 'c, 'd: 'c> {
struct Context<

    // global context
    'global,

    // router
    'regexset,

    // hyper lifetimes
    // 'request, 'network_stream: 'request,
    // 'response
    > {
    global_context: &'global GlobalContext<'global>,

    uri: &'regexset str,
    captures: Option<Captures<'regexset>>,

    view_route: AppRoute,

    // request: Request<'request, 'network_stream>,
    // response: Response<'response>


}

impl<'a, 'b> Context<'a, 'b> {

    /* deck API */

    fn deck_read(&self, deck_id: u64) {

        // lock database for read operation
        db_read_lock!(self.global_context.db_connection);
        // let db_op_lock = self.global_context.db_connection.read().unwrap();
        // let db_lock = db_op_lock.lock().unwrap();
    }
}

/* router manager */
// Source: https://github.com/gsquire/reroute

type RouterFn = fn(Context, Request, Response);
type LinkGenerator = fn(&Context) -> String;

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
struct Router {
    /// A custom 404 handler that you can provide.
    // pub not_found: Option<RouterFn>,

    routes: Option<RegexSet>,
    route_list: Vec<String>,
    compiled_list: Vec<Regex>,
    route_map: HashMap<RouteInfo, RouterFn>
}

impl Router {

    fn new() -> Self {
        Router {
            routes: None,
            route_list: Vec::new(),
            compiled_list: Vec::new(),
            route_map: HashMap::new(),
        }
    }

    fn finalize(&mut self) {
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
    fn handle<'a, 'b>(&'a self, context: &mut Context<'a, 'b>, request: &Request) -> Option<RouterFn> {

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
    fn add_route(&mut self, verb: Method, route: &str, handler: RouterFn) {

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

    /// A convenience method for GET requests.
    fn get(&mut self, route: &str, handler: RouterFn) {
        self.add_route(Method::Get, route, handler);
    }

}

/* routes */

fn route_not_found(mut context: Context, request: Request, mut response: Response) {

    // let mut context = context;

    let message = format!("No route handler found for {}", request.uri);

    // 404 status code
    *response.status_mut() = StatusCode::NotFound;

    response.send(message.as_bytes()).unwrap();
}

fn route_static_assets(context: Context, request: Request, response: Response) {

    // TODO: caching stuff from https://github.com/iron/staticfile

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
            route_not_found(context, request, response);
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
        route_not_found(context, request, response);
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
}

// Path: /
fn route_root(mut context: Context, request: Request, response: Response) {

    // TODO: fetch deck_id
    context.view_route = AppRoute::Deck(1, DeckRoute::New);

    render_app_component(context, format!("grokdb"), request, response);
}


fn route_settings(mut context: Context, request: Request, response: Response) {

    context.view_route = AppRoute::Settings;

    render_app_component(context, format!("grokdb"), request, response);
}

fn route_new_deck(mut context: Context, request: Request, response: Response) {

    // TODO: fetch deck_id
    context.view_route = AppRoute::Deck(1, DeckRoute::New);

    render_app_component(context, format!("grokdb"), request, response);
}

fn route_deck_description(mut context: Context, request: Request, response: Response) {

    // TODO: fetch deck_id
    context.view_route = AppRoute::Deck(1, DeckRoute::Description);

    render_app_component(context, format!("grokdb"), request, response);
}

fn route_deck_decks(mut context: Context, request: Request, response: Response) {

    // TODO: fetch deck_id
    context.view_route = AppRoute::Deck(1, DeckRoute::Decks);

    render_app_component(context, format!("grokdb"), request, response);
}

fn route_deck_cards(mut context: Context, request: Request, response: Response) {

    db_read_lock!(context.global_context.db_connection);

    // TODO: fetch deck_id
    context.view_route = AppRoute::Deck(1, DeckRoute::Cards);

    render_app_component(context, format!("grokdb"), request, response);
}

fn route_deck_meta(mut context: Context, request: Request, response: Response) {

    // TODO: fetch deck_id
    context.view_route = AppRoute::Deck(1, DeckRoute::Meta);

    render_app_component(context, format!("grokdb"), request, response);
}

fn route_deck_settings(mut context: Context, request: Request, response: Response) {

    // TODO: fetch deck_id
    context.view_route = AppRoute::Deck(1, DeckRoute::Settings);

    render_app_component(context, format!("grokdb"), request, response);
}

// // Path: /deck/:deck_id/view/cards
// fn route_deck_cards(mut context: Context, request: Request, response: Response) {



//     // lock database for read operation
//     // let db_op_lock = context.global_context.db_connection.read().unwrap();
//     // let db_lock = db_op_lock.lock().unwrap();
//     db_read_lock!(context.global_context.db_connection);

//     // TODO: rendering
// }

/* route handler helpers */

fn render_app_component(
    context: Context,
    app_component_title: String,
    request: Request,
    response: Response) {



    let mut response = response;

    response.headers_mut().set((ContentType(
        mime!(Text/Html)
    )));

    let app_component = AppComponent::new(&context, app_component_title);

    // We lock the database for reads
    db_read_lock!(context.global_context.db_connection);

    let mut stream = response.start().unwrap();
    app_component.write_to_io(&mut stream)
        .unwrap();


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

/* component route constants */

type DeckID = u64;

enum AppRoute {

    Home,

    // user settings
    Settings,

    Deck(DeckID, DeckRoute)
}

enum DeckRoute {

    New,
    Description,
    Decks,
    Cards,
    Settings,
    Meta

    // Create,
    // Read,
    // Update,
    // http://stackoverflow.com/a/26897298/412627
// http://programmers.stackexchange.com/questions/114156/why-are-there-are-no-put-and-delete-methods-on-html-forms
    // Delete
}

fn route_root_link(context: &Context) -> String {
    format!("/")
}

fn route_settings_link(context: &Context) -> String {
    format!("/settings")
}

fn route_new_deck_link(context: &Context) -> String {

    // TODO: fetch deck_id

    format!("/deck/1/new/deck")
}

fn route_deck_description_link(context: &Context) -> String {

    // TODO: fetch deck_id

    format!("/deck/1/description")
}

fn route_deck_decks_link(context: &Context) -> String {

    // TODO: fetch deck_id

    format!("/deck/1/decks")
}

fn route_deck_cards_link(context: &Context) -> String {

    // TODO: fetch deck_id

    format!("/deck/1/cards")
}

fn route_deck_meta_link(context: &Context) -> String {

    // TODO: fetch deck_id

    format!("/deck/1/meta")
}

fn route_deck_settings_link(context: &Context) -> String {

    // TODO: fetch deck_id

    format!("/deck/1/settings")
}

fn get_route_tuple(view_route: AppRoute) -> (&'static str, RouterFn, LinkGenerator) {

    match view_route {

        AppRoute::Home => (r"^/$", route_root, route_root_link),

        AppRoute::Settings => (r"^/settings$", route_settings, route_settings_link),

        AppRoute::Deck(_, deck_route) => {

            match deck_route {
                DeckRoute::New => (
                    r"^/deck/(?P<deck_id>\d+)/new/deck$",
                    route_new_deck,
                    route_new_deck_link),

                DeckRoute::Description => (
                    r"^/deck/(?P<deck_id>\d+)/description$",
                    route_deck_description,
                    route_deck_description_link),

                DeckRoute::Decks => (
                    r"^/deck/(?P<deck_id>\d+)/decks$",
                    route_deck_decks,
                    route_deck_decks_link),

                DeckRoute::Cards => (
                    r"^/deck/(?P<deck_id>\d+)/cards$",
                    route_deck_cards,
                    route_deck_cards_link),

                DeckRoute::Meta => (
                    r"^/deck/(?P<deck_id>\d+)/meta$",
                    route_deck_meta,
                    route_deck_meta_link),

                DeckRoute::Settings => (
                    r"^/deck/(?P<deck_id>\d+)/settings$",
                    route_deck_settings,
                    route_deck_settings_link),
            }
        }
    }

}

fn get_route_regex(view_route: AppRoute) -> &'static str {
    let (regex_matcher, _, _) = get_route_tuple(view_route);
    regex_matcher
}

fn view_route_to_link(view_route: AppRoute, context: &Context) -> String {
    let (_, _, link_generator) = get_route_tuple(view_route);
    link_generator(context)
}

// helper macro to attach get_route_tuple(...) to route manager
macro_rules! route(
    ($router: expr, $method: ident, $view_route: expr) => (
        let (regex_matcher, route_handler, _) = get_route_tuple($view_route);
        $router.add_route(Method::$method, regex_matcher, route_handler);
    )
);

/* components (templates) */

// components/AppComponent
struct AppComponent<'component, C: 'component> {
    context: &'component C,
    title: String
}

impl<'component, 'a, 'b> AppComponent<'component, Context<'a, 'b>> {
    fn new(context: &'component Context<'a, 'b>, title: String) -> Self {
        AppComponent {
            context: context,
            title: title
        }
    }
}

impl<'component, 'a, 'b> RenderOnce for AppComponent<'component, Context<'a, 'b>> {

    fn render_once(self, tmpl: &mut TemplateBuffer) {

        let AppComponent {context, title} = self;

        tmpl << html! {
            : raw!("<!DOCTYPE html>");
            html {
                head {
                    title { : &title }
                    link (
                        rel="stylesheet",
                        href="/assets/spectre.min.css"
                    );
                }
                body {
                    section(class="container grid-960") {
                        header(class="navbar") {
                            section(class="navbar-section") {
                                a(href = view_route_to_link(AppRoute::Home, &context), class="navbar-brand") {
                                    : "grokdb"
                                }
                            }
                            section(class="navbar-section") {
                                a(
                                    href = view_route_to_link(AppRoute::Home, &context),
                                    class="btn btn-link badge",
                                    data-badge="9"
                                ) {
                                    : "decks"
                                }
                                a(href="#", class="btn btn-link") {
                                    : "stashes"
                                }
                                a(href = view_route_to_link(AppRoute::Settings, &context), class="btn btn-link") {
                                    : "settings"
                                }
                                a(href="#", class="btn btn-primary") {
                                    : "login"
                                }
                            }
                        }
                        section(class="container") {
                            // : ViewRouteResolver::new(&context)
                            |tmpl| match context.view_route {
                                AppRoute::Home => {
                                    unreachable!();
                                    // tmpl << DeckDetailComponent::new(&context)
                                }
                                AppRoute::Settings => {
                                    tmpl << SettingsComponent::new(&context)
                                }
                                AppRoute::Deck(_deck_id, ref _deck_route) => {

                                    tmpl << DeckDetailComponent::new(&context)

                                    // match deck_route {
                                    //     &DeckRoute::New => tmpl << NewDeckComponent::new(&context)
                                    // }

                                }
                            };

                        }
                        // p : Page::new(format!("boop"))
                    }
                }
            }

        };
    }
}

// components/ViewRouteResolver
// TODO: bikeshed

// struct ViewRouteResolver<'component, C: 'component> {
//     context: &'component C
// }

// impl<'component, 'a, 'b> ViewRouteResolver<'component, Context<'a, 'b>> {
//     fn new(context: &'component Context<'a, 'b>) -> Self {
//         ViewRouteResolver {
//             context: context
//         }
//     }
// }

// macro_rules! render_component(
//     ($tmpl: expr, $component: expr) => (
//         // $tmpl << html! {
//         //     : $component
//         // };
//         $tmpl << $component;
//     )
// );

// impl<'component, 'a, 'b> RenderOnce for ViewRouteResolver<'component, Context<'a, 'b>> {

//     fn render_once(self, tmpl: &mut TemplateBuffer) {

//         let ViewRouteResolver {context} = self;

//         match context.view_route {
//             AppRoute::Home => {
//                 render_component!(tmpl, DeckDetail::new(&context))
//             }
//             AppRoute::Settings => {
//                 render_component!(tmpl, Settings::new(&context))
//             }
//             AppRoute::Deck(_, ref deck_route) => {

//                 match deck_route {
//                     &DeckRoute::New => render_component!(tmpl, Settings::new(&context))
//                 }

//             }
//         }

//     }
// }

// components/SettingsComponent
struct SettingsComponent<'component, C: 'component> {
    context: &'component C
}

impl<'component, 'a, 'b> SettingsComponent<'component, Context<'a, 'b>> {
    fn new(context: &'component Context<'a, 'b>) -> Self {
        SettingsComponent {
            context: context
        }
    }
}

impl<'component, 'a, 'b> RenderOnce for SettingsComponent<'component, Context<'a, 'b>> {

    fn render_once(self, tmpl: &mut TemplateBuffer) {

        let SettingsComponent {context} = self;

        tmpl << html! {
            section {
                : "Settings (work in progress)"
            }
        };
    }
}

// components/DeckDetailComponent
struct DeckDetailComponent<'component, C: 'component> {
    context: &'component C
}

impl<'component, 'a, 'b> DeckDetailComponent<'component, Context<'a, 'b>> {
    fn new(context: &'component Context<'a, 'b>) -> Self {
        DeckDetailComponent {
            context: context
        }
    }
}

impl<'component, 'a, 'b> RenderOnce for DeckDetailComponent<'component, Context<'a, 'b>> {

    fn render_once(self, tmpl: &mut TemplateBuffer) {

        let DeckDetailComponent {context} = self;

        let (deck_id, deck_route) = {
            match context.view_route {
                AppRoute::Deck(deck_id, ref deck_route) => (deck_id, deck_route),
                _ => unreachable!()
            }
        };

        tmpl << html! {

            : BreadCrumbComponent::new(&context);

            section(class="columns") {
                section(class="column col-3") {
                    : DeckNavComponent::new(&context);
                }
                section(class="column col-9") {
                    // : "fuck"
                    |tmpl| match deck_route {
                        &DeckRoute::New => tmpl << NewDeckComponent::new(&context),
                        &DeckRoute::Description => tmpl << DeckDescriptionComponent::new(&context),
                        &DeckRoute::Decks => tmpl << DeckDescriptionComponent::new(&context),
                        &DeckRoute::Cards => tmpl << DeckDescriptionComponent::new(&context),
                        &DeckRoute::Meta => tmpl << DeckDescriptionComponent::new(&context),
                        &DeckRoute::Settings => tmpl << DeckDescriptionComponent::new(&context),
                    };
                }
            }
        };
    }
}

// components/NewDeckComponent
struct NewDeckComponent<'component, C: 'component> {
    context: &'component C
}

impl<'component, 'a, 'b> NewDeckComponent<'component, Context<'a, 'b>> {
    fn new(context: &'component Context<'a, 'b>) -> Self {
        NewDeckComponent {
            context: context
        }
    }
}

impl<'component, 'a, 'b> RenderOnce for NewDeckComponent<'component, Context<'a, 'b>> {

    fn render_once(self, tmpl: &mut TemplateBuffer) {

        let NewDeckComponent {context} = self;

        tmpl << html! {

            : "new deck"
        };
    }
}

// components/DeckDescriptionComponent
struct DeckDescriptionComponent<'component, C: 'component> {
    context: &'component C
}

impl<'component, 'a, 'b> DeckDescriptionComponent<'component, Context<'a, 'b>> {
    fn new(context: &'component Context<'a, 'b>) -> Self {
        DeckDescriptionComponent {
            context: context
        }
    }
}

impl<'component, 'a, 'b> RenderOnce for DeckDescriptionComponent<'component, Context<'a, 'b>> {

    fn render_once(self, tmpl: &mut TemplateBuffer) {

        let DeckDescriptionComponent {context} = self;

        tmpl << html! {

            : "deck description"
        };
    }
}

// components/BreadCrumbComponent
struct BreadCrumbComponent<'component, C: 'component> {
    context: &'component C
}

impl<'component, 'a, 'b> BreadCrumbComponent<'component, Context<'a, 'b>> {
    fn new(context: &'component Context<'a, 'b>) -> Self {
        BreadCrumbComponent {
            context: context
        }
    }
}

impl<'component, 'a, 'b> RenderOnce for BreadCrumbComponent<'component, Context<'a, 'b>> {

    fn render_once(self, tmpl: &mut TemplateBuffer) {

        let BreadCrumbComponent {context} = self;

        tmpl << html! {
            ul(class="breadcrumb") {
                li(class="breadcrumb-item") {
                    a(href="#") {
                        : "Library"
                    }
                }
                li(class="breadcrumb-item") {
                    a(href="#") {
                        : "Math"
                    }
                }
            }

        };
    }
}

// components/DeckNavComponent
struct DeckNavComponent<'component, C: 'component> {
    context: &'component C
}

impl<'component, 'a, 'b> DeckNavComponent<'component, Context<'a, 'b>> {
    fn new(context: &'component Context<'a, 'b>) -> Self {
        DeckNavComponent {
            context: context
        }
    }
}

impl<'component, 'a, 'b> RenderOnce for DeckNavComponent<'component, Context<'a, 'b>> {

    fn render_once(self, tmpl: &mut TemplateBuffer) {

        let DeckNavComponent {context} = self;

        tmpl << html! {
            ul(class="menu") {
                li(class="menu-header") {
                    span(class="menu-header-text") {
                        : "#123 Mathematics"
                    }

                }
                li(class="menu-item") {
                    a(href = view_route_to_link(AppRoute::Deck(Default::default(), DeckRoute::Description),
                        &context),
                        class="active") {
                        : "Description"
                    }
                }
                li(class="menu-item") {
                    a(href="#") {
                        : "Child Decks"
                    }
                }
                li(class="menu-item") {
                    a(href="#") {
                        : "Cards"
                    }
                }
                li(class="divider") {}
                li(class="menu-item") {
                    : "Brief Deck Statistics (TBA)"
                }
                li(class="divider") {}
                li(class="menu-item") {
                    a(href="#") {
                        : "Meta"
                    }
                }
                li(class="menu-item") {
                    a(href="#") {
                        : "Settings"
                    }
                }
            }

        };
    }
}

/* helpers */

#[inline]
fn decode_percents(string: &OsStr) -> String {

    let string = format!("{}", string.to_string_lossy());

    format!("{}", percent_decode(string.as_bytes()).decode_utf8_lossy())

    // String::from_utf8(.if_any().unwrap()).unwrap()
    // OsStr::new(&token)
}

/* grokdb */

fn main() {

    // TODO: clap-rs

    /* database */

    let db_conn = Connection::open("test");

    let db_conn = match db_conn {
        Err(why) => {
            panic!("{}", why);
        },
        Ok(db_conn) => {
            db_conn
        }
    };

    let global_context = GlobalContext {
        assets_root_path: Path::new("assets/"),
        db_connection: Arc::new(RwLock::new(Mutex::new(db_conn))),
        // db_ops_lock: Arc::new(RwLock::new(true))
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
        AppRoute::Deck(_deck_id, ref _deck_route) => {
            match _deck_route {
                &DeckRoute::New => {},
                &DeckRoute::Description => {},
                &DeckRoute::Decks => {},
                &DeckRoute::Cards => {},
                &DeckRoute::Meta => {},
                &DeckRoute::Settings => {}
            }
        }
    };

    route!(router, Get, AppRoute::Home);
    route!(router, Get, AppRoute::Settings);
    route!(router, Get, AppRoute::Deck(Default::default(), DeckRoute::New));
    route!(router, Get, AppRoute::Deck(Default::default(), DeckRoute::Description));
    route!(router, Get, AppRoute::Deck(Default::default(), DeckRoute::Decks));
    route!(router, Get, AppRoute::Deck(Default::default(), DeckRoute::Cards));
    route!(router, Get, AppRoute::Deck(Default::default(), DeckRoute::Meta));
    route!(router, Get, AppRoute::Deck(Default::default(), DeckRoute::Settings));

    // router.get(r"^/$", route_root);
    // router.get(r"^/settings$", route_settings);

    // TODO: limit path length?
    router.get(r"^/assets/(?P<path>.+)$", route_static_assets);

    // Freeze the router. This will validate the router.
    router.finalize();

    let router = router;

    /* server */

    let server = Server::http("127.0.0.1:3000").unwrap();

    let _guard = server.handle(move |request: Request, response: Response| {

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
            None => route_not_found
        };

        handler(context, request, response);

    });

    println!("Listening on http://127.0.0.1:3000");
}
