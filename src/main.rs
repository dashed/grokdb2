// #![deny(warnings)]
extern crate hyper;
extern crate regex;

use hyper::method::Method;
use hyper::server::{Server, Handler, Request, Response};

use regex::{Regex, RegexSet};


static PHRASE: &'static [u8] = b"Hello World!";


fn main() {

    // TODO: clap-rs

    // TODO: how to manage middleware?

    let route_info = RouteInfo{route: route.to_owned(), verb: verb};


    let server = Server::http("127.0.0.1:3000").unwrap();

    let _guard = server.handle(move |request: Request, response: Response| {

        // routes

        response.send(PHRASE).unwrap();
    });

    println!("Listening on http://127.0.0.1:3000");
}

/* route helpers */
// Source: https://github.com/gsquire/reroute

pub type RouterFn = fn(Request, Response, Captures);

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

#[derive(PartialEq, Eq, Hash)]
pub struct RouteInfo {
    route: String,
    verb: Method
}
