// #![deny(warnings)]
extern crate hyper;
extern crate regex;

use std::collections::HashMap;

use regex::{Regex, RegexSet};
use regex::{Captures};

use hyper::method::Method;
use hyper::server::{Server, Handler, Request, Response};
use hyper::uri::RequestUri;
use hyper::status::StatusCode;


static PHRASE: &'static [u8] = b"Hello World!";


fn main() {

    // TODO: clap-rs

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


        let mut context = Context {
            uri: &format!("{}", request.uri),
            captures: None
        };

        // middleware/router

        let handler = match router.handle(&mut context, &request) {
            Some(handler) => {
                handler
            },
            None => route_not_found
        };

        handler(context, request, response);

        // response.send(PHRASE).unwrap();
    });

    println!("Listening on http://127.0.0.1:3000");
}

/* router */
// Source: https://github.com/gsquire/reroute

struct Context<'c> {

    uri: &'c str,
    captures: Option<Captures<'c>>
}

type RouterFn = fn(Context, Request, Response);

#[derive(PartialEq, Eq, Hash)]
struct RouteInfo {
    // route: String,
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
    fn handle<'c>(&'c self, context: &mut Context<'c>, request: &Request) -> Option<RouterFn> {

        // TODO: remove
        // let uri = format!("{}", request.uri);

        let matches = self.routes.as_ref().unwrap().matches(&context.uri);
        let index = matches.iter().next();

        match index {
            Some(matched_idx) => {

                // let route = &self.route_list[i];
                let route_info = RouteInfo{
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

fn route_not_found(ctx: Context, request: Request, mut response: Response) {
    let message = format!("No route handler found for {}", request.uri);
    *response.status_mut() = StatusCode::NotFound;
    response.send(message.as_bytes()).unwrap();
}

fn route_static_assets(ctx: Context, _: Request, mut response: Response) {

    let path = ctx.captures.unwrap().name("path").unwrap();

    let message = format!("Path {}", path);

    response.send(message.as_bytes()).unwrap();

    // response.send(PHRASE).unwrap();
}
