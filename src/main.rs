// #![deny(warnings)]
extern crate hyper;
extern crate regex;
extern crate url;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{PathBuf, Path};

use url::percent_encoding::percent_decode;

use regex::{Regex, RegexSet};
use regex::{Captures};

use hyper::method::Method;
use hyper::server::{Server, Handler, Request, Response};
use hyper::uri::RequestUri;
use hyper::status::StatusCode;


static PHRASE: &'static [u8] = b"Hello World!";


fn main() {

    // TODO: clap-rs

    let global_context = GlobalContext {
        assets_root_path: Path::new("assets/")
    };

    /* router setup */

    // let route_info = RouteInfo{route: route.to_owned(), verb: verb};

    let mut router = Router {
        routes: None,
        route_list: Vec::new(),
        compiled_list: Vec::new(),
        route_map: HashMap::new(),
    };

    router.get(r"/assets/(?P<path>.+)", route_static_assets);

    // validate router.
    router.finalize();

    let router = router;

    /* server */

    let server = Server::http("127.0.0.1:3000").unwrap();

    let _guard = server.handle(move |request: Request, response: Response| {

        let uri = format!("{}", request.uri);

        let mut context = Context {
            global_context: &global_context,
            request: request,
            response: response,
            uri: &uri,
            captures: None
        };

        // middleware/router

        let handler = match router.handle(&mut context) {
            Some(handler) => {
                handler
            },
            None => route_not_found
        };

        handler(context);

        // let context = context;

        // response.send(PHRASE).unwrap();
    });

    println!("Listening on http://127.0.0.1:3000");
}

/* contexts */

struct GlobalContext<'a> {
    assets_root_path: &'a Path
}

// struct Context<'a, 'b, 'c, 'd: 'c> {
struct Context<
    'global,

    'regex,

    // hyper lifetimes
    'a, 'k: 'a, 'b
    > {
    global_context: &'global GlobalContext<'global>,

    uri: &'regex str,
    captures: Option<Captures<'regex>>,

    request: Request<'a, 'k>,
    response: Response<'b>


}


/* router */
// Source: https://github.com/gsquire/reroute

type RouterFn = fn(Context);

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
    fn handle<'a, 'b, 'c, 'd, 'e>(&'a self, context: &mut Context<'a, 'b, 'c, 'd, 'e>) -> Option<RouterFn> {

        // TODO: remove
        // let uri = format!("{}", request.uri);

        let matches = self.routes.as_ref().unwrap().matches(&context.uri);
        let index = matches.iter().next();

        match index {
            Some(matched_idx) => {

                // let route = &self.route_list[i];
                let route_info = RouteInfo{
                    route_map_idx: matched_idx,
                    verb: context.request.method.clone()
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

fn route_not_found(mut context: Context) {

    // let mut context = context;

    let message = format!("No route handler found for {}", context.request.uri);
    *context.response.status_mut() = StatusCode::NotFound;

    context.response.send(message.as_bytes()).unwrap();
}

fn route_static_assets(context: Context) {

    let req_path_raw = context.captures.unwrap().name("path").unwrap();

    // let req_path = Path::new(&req_path);
    let decoded_req_path = Path::new(&req_path_raw).iter().map(decode_percents);

    let mut req_path = context.global_context.assets_root_path.to_path_buf();
    req_path.extend(decoded_req_path);
    let req_path = req_path;

    // TODO: fetch file

    // TODO: stream file to output

    // https://github.com/iron/iron/blob/2fd6e079592ec340d8c75200bd54fde91ce34d51/src/modifiers.rs#L116-L154

    let message = format!("Path {:?}", req_path);

    context.response.send(message.as_bytes()).unwrap();

    // response.send(PHRASE).unwrap();
}

/* helpers */

#[inline]
fn decode_percents(string: &OsStr) -> String {

    let string = format!("{}", string.to_string_lossy());

    format!("{}", percent_decode(string.as_bytes()).decode_utf8_lossy())

    // String::from_utf8(.if_any().unwrap()).unwrap()
    // OsStr::new(&token)
}
