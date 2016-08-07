/* rust lib imports */

use std::panic::{self, AssertUnwindSafe};

/* 3rd-party imports */

use horrorshow::{RenderOnce, TemplateBuffer, Template, FnRenderer};

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

/* local imports */

use route::{AppRoute, RenderResponse};

/// /////////////////////////////////////////////////////////////////////////////

/* rendering */

#[inline]
pub fn render_response(render: RenderResponse, mut response: Response) {

    match render {
        RenderResponse::RenderComponent(app_route) => {
            render_components(app_route, response);
        }
        RenderResponse::RenderNotFound => {

            // let ref url = request.borrow().uri;

            // let message = format!("No route handler found for {}", url);
            let message = format!("Not Found 404");

            // 404 status code
            *response.status_mut() = StatusCode::NotFound;

            response.send(message.as_bytes()).unwrap();

        }
        RenderResponse::RenderAsset(header, content) => {
            response.headers_mut().set((header));
            response.send(&content).unwrap();
        }
        _ => {
            panic!("fix me");
        }
    }


}

#[inline]
fn render_components(app_route: AppRoute, mut response: Response) {

    let app_component = {
        FnRenderer::new(|tmpl| {
            AppComponent(tmpl, app_route);
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

        println!("TEMPLATE RENDERING PANIC: {:?}", result.err().unwrap());

        // TODO: fix... route_internal_server_error
        // super::routes::internal_server_error(request, response);

        return;
    }

    match result.ok().unwrap() {
        Err(why) => {
            println!("ERROR RENDERING: {:?}", why);

            // TODO: fix
            // super::routes::internal_server_error(request, response);

            return;
        }
        Ok(rendered) => {

            response.headers_mut().set((ContentType(mime!(Text/Html))));

            response.send(rendered.as_bytes()).unwrap();

            return;
        }
    };

}


/* components */

#[inline]
pub fn AppComponent(tmpl: &mut TemplateBuffer, app_route: AppRoute) {

    tmpl << html! {
        : raw!("<!DOCTYPE html>");
        html {
            head {
                title {
                    // TODO: fix
                    : "title"
                }
                link (
                    rel="stylesheet",
                    href="/assets/bulma.css"
                );

                |tmpl| {
                    tmpl << html!{
                        style {

                            // sticky footer
                            // source: https://philipwalton.github.io/solved-by-flexbox/demos/sticky-footer/

                            : raw!("\
                                body {\
                                    display: flex;\
                                    min-height: 100vh;\
                                    flex-direction: column;\
                                }\
                            ");
                            : raw!("\
                                #grokdb {\
                                    flex: 1;\
                                }\
                            ");

                            // custom styles
                            // TODO: merge back into bulma css

                            : raw!("\
                                .is-side-paddingless {\
                                    padding-left: 0;\
                                    padding-right: 0;\
                                }\
                            ");

                            : raw!("\
                                .is-bold{\
                                    font-weight: bold;\
                                }\
                            ")
                        }
                    }
                }

                // TODO:  custom stylesheet for specific views
            }
            body {

                div(id="grokdb") {
                    div(class="container", style="max-width:960px;margin-top:10px;margin-bottom:10px;") {
                        nav(class="nav") {
                            div(class="nav-left") {
                                a(class="nav-item", href="#") {
                                    h1(class="title", style="font-weight:bold;") {
                                        : raw!("grokdb")
                                    }

                                }
                            }
                            span(class="nav-toggle") {
                                span {}
                                span {}
                                span {}
                            }
                            div(class="nav-right nav-menu") {
                                a(class="nav-item is-bold", href="#") {
                                    : raw!("Decks")
                                }
                                a(class="nav-item is-bold", href="#") {
                                    : raw!("Stashes")
                                }
                                a(class="nav-item is-bold", href="#") {
                                    : raw!("Preferences")
                                }
                            }
                        }
                    }

                    div(class="container", style="max-width:960px;") {
                        div(class="columns") {

                            div(class="column") {

                                div(class="columns") {
                                    div(class="column") {
                                        span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                            : raw!("/ ");
                                        }
                                        span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                            a(href="#") {
                                                : raw!("Library");
                                            }
                                        }
                                        span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                            : raw!(" / ");
                                        }
                                        span(class="title is-5 is-marginless is-bold") {
                                            a(href="#") {
                                                : raw!("Math");
                                            }
                                        }
                                    }
                                }

                                div(class="columns") {
                                    div(class="column") {
                                        a(class="button is-bold is-success") {
                                            : raw!("New Deck")
                                        }
                                    }
                                }

                                // TODO: hide
                                div(class="columns") {
                                    div(class="column") {
                                        |tmpl| PaginationComponent(tmpl);
                                    }
                                }

                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, false);
                                |tmpl| DeckListItemComponent(tmpl, true);

                                // TODO: hide
                                div(class="columns", style="margin-top:10px;") {
                                    div(class="column") {
                                        |tmpl| PaginationComponent(tmpl);
                                    }
                                }

                            }

                            div(class="column is-one-quarter") {
                                nav(class="panel") {
                                    p(class="panel-heading", style="font-weight: normal;") {
                                        : raw!("Deck #123")
                                    }
                                    div(class="panel-block") {

                                        aside(class="menu") {
                                            // p(class="menu-label is-bold") {
                                            //     : raw!("Deck #123")
                                            // }
                                            ul(class="menu-list") {
                                                li {
                                                    a(href="#", class="is-bold") {
                                                        : "Description"
                                                    }
                                                }
                                                li {
                                                    a(href="#", class="is-active is-bold") {
                                                        : "Decks"
                                                    }
                                                }
                                                li {
                                                    a(href="#", class="is-bold") {
                                                        : "Cards"
                                                    }
                                                }
                                                li {
                                                    a(href="#", class="is-bold") {
                                                        : "Stats"
                                                    }
                                                }
                                                li {
                                                    a(href="#", class="is-bold") {
                                                        : "Settings"
                                                    }
                                                }
                                            }
                                        }

                                    }
                                }


                            }
                        }


                    }
                }

                footer(class="footer container", style="max-width:960px;padding-bottom:20px;") {
                    div(class="content has-text-centered") {
                        p {
                            : raw!("grokdb v");
                            : env!("CARGO_PKG_VERSION");

                            // TODO: use build.rs to generate:
                            // - date/time of compiled binary
                            // - server hostname
                        }
                    }
                }

                // TODO: uncomment
                // |tmpl| {
                //     match context.view_route {
                //         AppRoute::Deck(_, DeckRoute::Review) =>  {
                //             tmpl << html! {
                //                 script(type="text/javascript", src="https://cdnjs.cloudflare.com/ajax/libs/babel-polyfill/6.9.1/polyfill.min.js") {}
                //                 script(type="text/javascript", src="https://cdnjs.cloudflare.com/ajax/libs/react/15.1.0/react.js") {}
                //                 script(type="text/javascript", src="https://cdnjs.cloudflare.com/ajax/libs/react/15.1.0/react-dom.js") {}

                //                 // script(type="text/javascript", src="/assets/vendor.js") {}
                //                 script(type="text/javascript", src="/assets/deck_review.js") {}
                //             };

                //         },
                //         AppRoute::Deck(_, DeckRoute::NewDeck) =>  {
                //             tmpl << html! {
                //                 script(type="text/javascript", src="https://cdnjs.cloudflare.com/ajax/libs/babel-polyfill/6.9.1/polyfill.min.js") {}
                //                 script(type="text/javascript", src="https://cdnjs.cloudflare.com/ajax/libs/react/15.1.0/react.js") {}
                //                 script(type="text/javascript", src="https://cdnjs.cloudflare.com/ajax/libs/react/15.1.0/react-dom.js") {}

                //                 // script(type="text/javascript", src="/assets/vendor.js") {}
                //                 script(type="text/javascript") {
                //                     // needs to be raw b/c of html escaping
                //                     : raw!(format!("window.__PRE_RENDER_STATE__ = {};",
                //                         view_route_to_pre_render_state(context.view_route.clone(), context)))
                //                 }
                //                 script(type="text/javascript", src="/assets/new_deck.js") {}
                //             };

                //         },
                //         _ => {}
                //     };
                // }

            }
        }

    };
}

#[inline]
fn DeckListItemComponent(tmpl: &mut TemplateBuffer, is_bottom: bool) {
    tmpl <<
    html!{
        div(class="columns is-marginless",
            style=labels!(
                "border-bottom:1px dotted #d3d6db;" => !is_bottom)) {
            div(class="column is-side-paddingless") {
                h5(class="title is-5 is-marginless is-bold") {
                    a(href="#") {
                        : "What does the fox say?"
                    }
                }
                span(style="font-size:12px;") {
                    : "Deck #123";
                    : raw!(" ");
                    a(href="#") {
                        : raw!("View Cards")
                    }
                }
            }
        }
    }
}

#[inline]
fn PaginationComponent(tmpl: &mut TemplateBuffer) {
    tmpl <<
    html!{
        nav(class="pagination") {
            a(class="button is-bold") {
                : raw!("Previous")
            }
            a(class="button is-bold") {
                : raw!("Next")
            }
            ul {
                li {
                    a(class="button is-bold") {
                        : raw!("1")
                    }
                }
                li {
                    a(class="button is-bold") {
                        : raw!("2")
                    }
                }
                li {
                    a(class="button is-bold") {
                        : raw!("3")
                    }
                }
                li {
                    span(class="is-bold") {
                        : "..."
                    }
                }
                li {
                    a(class="button is-bold") {
                        : raw!("44")
                    }
                }
                li {
                    a(class="button is-bold") {
                        : raw!("45")
                    }
                }
                li {
                    a(class="button is-primary is-bold") {
                        : raw!("46")
                    }
                }
                li {
                    a(class="button is-bold") {
                        : raw!("47")
                    }
                }
                li {
                    a(class="button is-bold") {
                        : raw!("48")
                    }
                }
                li {
                    span(class="is-bold") {
                        : "..."
                    }
                }
                li {
                    a(class="button is-bold") {
                        : raw!("100")
                    }
                }
                li {
                    a(class="button is-bold") {
                        : raw!("101")
                    }
                }
                li {
                    a(class="button is-bold") {
                        : raw!("102")
                    }
                }
            }
        }
    }
}
