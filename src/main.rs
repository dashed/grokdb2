#[macro_use]
extern crate horrorshow;
extern crate hyper;
extern crate regex;
// TODO: this is interlized b/c of hyper import
// extern crate reroute;

// local modules
mod reroute;
// mod queries;
// mod api;

use std::io::BufWriter;

use reroute::{Captures, Router};

use hyper::server::{Server, Request, Response};
use hyper::header::{Headers, ContentType, TransferEncoding};
use hyper::header::Encoding::{Gzip};
use hyper::mime::{Mime, TopLevel, SubLevel};


use horrorshow::{RenderOnce, TemplateBuffer, Template};

struct Page<C> {
    title: String,
    content: C,
}

impl Page<String> {
    fn from_string_content(title: String, content: String) -> Self {
        Page { title: title, content: content }
    }
}

impl<C> RenderOnce for Page<C> where C: RenderOnce {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let Page {title, content} = self;
        // The actual template:
        tmpl << html! {
            article {
                header {
                    h1 : title
                }
                section : content
            }
        };
    }
}

fn digit_handler(_: Request, mut res: Response, c: Captures) {
    println!("captures: {:?}", c);

      let page = Page::from_string_content(String::from("My title"),
                                       String::from("Some content."));

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

fn main() {
    let mut router = Router::new();

    // Use raw strings so you don't need to escape patterns.
    router.get(r"/(\d+)", digit_handler);

    // There is no 404 handler added, so it will use the default defined in the
    // library.
    router.finalize().unwrap();

    println!("Listening");

    // You can pass the router to hyper's Server's handle function as it
    // implements the Handle trait.
    Server::http("127.0.0.1:3000").unwrap().handle(router).unwrap();

}
