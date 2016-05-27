#![feature(unboxed_closures, fn_traits)]

// caveats:
// https://is.gd/cYclF2

#[macro_use]
extern crate horrorshow;
extern crate hyper;
extern crate regex;
extern crate rusqlite;
// TODO: this is interlized b/c of hyper import
// extern crate reroute;

// use std::boxed::FnBox;

// local modules
mod reroute;
// mod queries;
// mod api;

use std::io::BufWriter;
use std::sync::{Arc, Mutex, LockResult, MutexGuard};


use reroute::{Captures, Router};

use hyper::server::{Server, Request, Response};
use hyper::header::{Headers, ContentType, TransferEncoding};
use hyper::header::Encoding::{Gzip};
use hyper::mime::{Mime, TopLevel, SubLevel};

use rusqlite::{Connection, Error, Result as SqliteResult};

use horrorshow::{RenderOnce, TemplateBuffer, Template};



fn main() {

    // TODO: clap-rs

    // initialize global context

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
        foo: 42,
        db_conn: Arc::new(Mutex::new(db_conn))
    };

    let mut router = Router::new(global_context);

    // router.get(r"/assets/(.+)", static_assets_handler);

    // Use raw strings so you don't need to escape patterns.

    // router.get(r"/(\d+)", {
    //     let global_context = global_context.clone();
    //     |req: Request, mut res: Response, captures: Captures| {
    //             // digit_handler(global_context, req, res, captures);
    //     }
    // });

    router.get(r"/(\d+)", digit_handler);

    router.get(r"/boop/(\d+)", digit_handler);

    // router.get(r"/boop/(\d+)", {
    //     let global_context = global_context.clone();
    //     |req: Request, mut res: Response, captures: Captures| {
    //     // digit_handler2(global_context, req, res, captures);
    //     }
    // });


    // router.get(r"/(\d+)", global_context.wrap_route_handler(digit_handler));

    // router.get(r"/(\d+)", RouteHandler::new(global_context.clone(), digit_handler));

    // There is no 404 handler added, so it will use the default defined in the
    // library.
    router.finalize().unwrap();

    println!("Listening");

    // You can pass the router to hyper's Server's handle function as it
    // implements the Handle trait.
    Server::http("127.0.0.1:3000").unwrap().handle(router).unwrap();

}

/* route handlers */

// struct RouteHandler<Handler> {
//     handler: Handler,
//     global_context: GlobalContext
// }

// impl<'a, 'k, Handler> RouteHandler<Handler>
//     where Handler: Fn(Request<'a, 'k>, Response, Captures), Handler: Sync + Send {

//     fn new(global_context: GlobalContext, handler: Handler) -> Self {
//         RouteHandler {
//             handler: handler,
//             global_context: global_context
//         }
//     }

// }

// impl<'a, 'k, Handler> FnMut<(Request<'a, 'k>, Response<'a>, Captures,)> for RouteHandler<Handler>
//     where Handler: Fn(Request, Response, Captures), Handler: Sync + Send {
//     extern "rust-call" fn call_mut(&mut self, args: (Request, Response, Captures,)) {
//         let (request, response, captures) = args;
//         // let ref handler: Handler = self.handler;
//         // handler(request, response, captures);
//         (self.handler)(request, response, captures);
//     }
// }

// impl<'a, 'k, Handler> Fn<(Request<'a, 'k>, Response<'a>, Captures,)> for RouteHandler<Handler>
//     where Handler: Fn(Request, Response, Captures), Handler: Sync + Send {
//     extern "rust-call" fn call(&self, args: (Request, Response, Captures,)) {
//         let (request, response, captures) = args;
//         // let ref handler: Handler = self.handler;
//         // handler(request, response, captures);
//         self.call_mut(args)
//         // (self.handler)(request, response, captures);
//     }
// }


// impl<'a, 'k, Handler> FnOnce<(Request<'a, 'k>, Response<'a>, Captures,)> for RouteHandler<Handler>
//     where Handler: Fn(Request, Response, Captures), Handler: Sync + Send {
//     type Output = ();
//     extern "rust-call" fn call_once<'b, 'c>(mut self, args: (Request<'b, 'c>, Response<'b>, Captures)) {
//     // extern "rust-call" fn call_once(mut self, args: (Request, Response, Captures,)) {
//         self.call_mut(args)
//     }
// }

// #[derive(Clone, Copy)]
pub struct GlobalContext {
    foo: i32,
    db_conn: Arc<Mutex<Connection>>
}

// type RouteHandler = FnBox(Request, Response, Captures);

// impl GlobalContext {

//     fn wrap_route_handler<H: 'static>(&self, handler: H) -> Box<RouteHandler>
//         where H: Fn(GlobalContext, Request, Response, Captures) {

//         let self_clone = self.clone();
//         return Box::new(move |request: Request, mut response: Response, captures: Captures| {
//             handler(self_clone, request, response, captures);
//         });
//     }
// }



// fn digit_handler(_: Request, mut res: Response, c: Captures) {
fn digit_handler(ctx: &GlobalContext, _: Request, mut res: Response, c: Captures) {
    println!("captures: {:?}", c);

      let page = App::new(Context::new(), String::from("My title"));

    // res.headers_mut().set(TransferEncoding(vec![Gzip]));
    res.headers_mut().set((ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![]))));

    let mut stream = BufWriter::new(res.start().unwrap());

    page.write_to_io(&mut stream)
        .unwrap();

    // res
    //     .send(page.into_string().unwrap().as_bytes())
    //     // .header(Connection::close())
    //     .unwrap();
}

fn digit_handler2(ctx: &GlobalContext, _: Request, mut res: Response, c: Captures) {
    println!("captures: {:?}", c);

      let page = App::new(Context::new(), String::from("My title"));

    // res.headers_mut().set(TransferEncoding(vec![Gzip]));
    res.headers_mut().set((ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![]))));

    let mut stream = BufWriter::new(res.start().unwrap());

    page.write_to_io(&mut stream)
        .unwrap();

    // res
    //     .send(page.into_string().unwrap().as_bytes())
    //     // .header(Connection::close())
    //     .unwrap();
}


// fn static_assets_handler(_: Request, mut response: Response, captures: Captures) {

//     println!("captures: {:?}", captures);

//       let page = App::new(String::from("My title"));

//     // res.headers_mut().set(TransferEncoding(vec![Gzip]));
//     response.headers_mut().set((ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![]))));

//     let mut stream = BufWriter::new(response.start().unwrap());

//     page.write_to_io(&mut stream)
//         .unwrap();

// }

/* view route constants */

type DeckID = u64;

enum AppRoute {

    Home,

    Settings,
    Deck(DeckID, DeckRoute),
}

enum DeckRoute {

    New,
    Decks,
    Cards,
    Description,
    Settings

    // Create,
    // Read,
    // Update,
    // http://stackoverflow.com/a/26897298/412627
    // http://programmers.stackexchange.com/questions/114156/why-are-there-are-no-put-and-delete-methods-on-html-forms
    // Delete
}



/* templates */

struct Context {
    route: AppRoute
}

impl Context {
    fn new() -> Self {
        Context {
            route: AppRoute::Home
        }
    }
}

struct App {
    title: String
}

impl App {
    fn new(context: Context, title: String) -> Self {
        App {
            title: title
        }
    }
}

impl RenderOnce for App {

    fn render_once(self, tmpl: &mut TemplateBuffer) {
        // The actual template:
        let App {title} = self;
        tmpl << html! {
            : raw!("<!DOCTYPE html>");
            div {
                header {
                    h1 : title
                }
                p : Page::new(format!("boop"))
            }
        };
    }
}

struct Page {
    content: String
}

impl Page {
    fn new(content: String) -> Self {
        Page { content: content }
    }
}

impl RenderOnce for Page {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let Page {content} = self;
        // The actual template:
        tmpl << html! {
            article {
                section : content
            }
        };
    }
}
