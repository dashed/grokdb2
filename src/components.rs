/* rust lib imports */

use std::panic::{self, AssertUnwindSafe};
use std::rc::Rc;
use std::cell::RefCell;

/* 3rd-party imports */

use horrorshow::{RenderOnce, TemplateBuffer, Template, FnRenderer};

use hyper::method::Method;
use hyper::server::{Server, Handler, Request, Response};
use hyper::header::{Headers, ContentType, TransferEncoding};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::uri::RequestUri;
use hyper::status::StatusCode;
use hyper::header::{Header, HeaderFormat};

/* local imports */

use route::{AppRoute, RenderResponse, DeckRoute};
use context::Context;
use types::{DeckID, DecksPageQuery, Search};

/* ////////////////////////////////////////////////////////////////////////// */

/* macro helpers */

// alias wrapper
macro_rules! classnames {
    ($($tail:tt)+) => {{
        let classes = labels!($($tail)*).into_string().unwrap();
        if classes.len() > 0 {
            Some(classes)
        } else {
            None
        }
    }};
}

/* link generator */

pub fn view_route_to_link(context: Rc<RefCell<Context>>, app_route: AppRoute) -> String {
    match app_route {
        AppRoute::Deck(deck_id, deck_route) => {
            match deck_route {
                DeckRoute::NewDeck => format!("/deck/{}/new/deck", deck_id),
                DeckRoute::Decks(page_query, search) => format!("/deck/{}/decks", deck_id),
                _ => panic!("fix")
            }
        },
        _ => {
            panic!("fix this");
        }
    }
}

/* javascript generator */

fn pre_render_state(tmpl: &mut TemplateBuffer, context: Rc<RefCell<Context>>, app_route: &AppRoute) {

    // invariant: this function is excuted inside a script tag

    // NOTES:
    // - use raw! macro
    // - if possible, write JSON manually

    // window.__PRE_RENDER_STATE__
    match *app_route {
        AppRoute::Deck(deck_id, ref deck_route) => {
            match *deck_route {
                DeckRoute::NewDeck => {
                    tmpl << html! {
                        : raw!(
                            format!(
                                "window.__PRE_RENDER_STATE__ = \
                                    {{\
                                        POST_TO: '/api/deck/{}'\
                                    }};\
                                ",
                                deck_id
                            )
                        )
                    }
                }
                _ => {
                    // nothing
                }
            }
        },
        _ => {
            // nothing
        }
    }
}

/* components */

#[inline]
pub fn AppComponent(tmpl: &mut TemplateBuffer, context: Rc<RefCell<Context>>, app_route: &AppRoute) {

    tmpl << html! {
        : raw!("<!DOCTYPE html>");
        html {
            head {
                title {
                    // TODO: fix
                    : "title"
                }

                // http://docs.mathjax.org/en/latest/configuration.html
                script(type="text/x-mathjax-config") {
                    : raw!(r"
                        MathJax.Hub.Config({
                            skipStartupTypeset: true,
                            tex2jax: {
                                inlineMath: [ ['$', '$'], ['\\\\(','\\\\)'] ],
                                displayMath: [ ['$$','$$'], ['\\[', '\\]'] ],
                                processEscapes: true
                            }
                        });
                    ");
                }

                script(type="text/javascript", async, src="/assets/mathjax/MathJax.js?config=TeX-AMS-MML_HTMLorMML") {}

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
                                a(class="nav-item", href="/") {
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

                                div(class="nav-item") {
                                    a(
                                        class? = classnames!(
                                            "is-bold",
                                            "button is-primary" => {
                                                // TODO: re-review this
                                                matches!(*app_route, AppRoute::Deck(_, _)) ||
                                                matches!(*app_route, AppRoute::Card(_, _)) ||
                                                matches!(*app_route, AppRoute::CardInDeck(_, _, _))
                                            }),
                                        href="/") {
                                        : raw!("Decks")
                                    }
                                }

                                div(class="nav-item") {
                                    a(
                                        class? = classnames!(
                                            "is-bold",
                                            "button is-primary" => {
                                                // TODO: re-review this
                                                matches!(*app_route, AppRoute::Stashes)
                                            }),
                                        href="#") {
                                        : raw!("Stashes")
                                    }
                                }

                                div(class="nav-item") {
                                    a(
                                        class? = classnames!(
                                            "is-bold",
                                            "button is-primary" => {
                                                // TODO: re-review this
                                                matches!(*app_route, AppRoute::Preferences)
                                            }),
                                        href="#") {
                                        : raw!("Preferences")
                                    }
                                }

                            }
                        }
                    }

                    div(class="container", style="max-width:960px;") {
                        div(class="columns") {

                            |tmpl| {
                                match *app_route {
                                    AppRoute::Deck(deck_id, ref deck_route) => {
                                        DeckDetail(tmpl, context.clone(), deck_id, deck_route)
                                    },
                                    _ => {
                                        // TODO: complete this shit
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

                |tmpl| {
                    match *app_route {
                        // TODO: fix
                        // AppRoute::Deck(_, DeckRoute::Review) =>  {
                        //     tmpl << html! {
                        //         script(type="text/javascript", src="https://cdnjs.cloudflare.com/ajax/libs/babel-polyfill/6.9.1/polyfill.min.js") {}
                        //         script(type="text/javascript", src="https://cdnjs.cloudflare.com/ajax/libs/react/15.1.0/react.js") {}
                        //         script(type="text/javascript", src="https://cdnjs.cloudflare.com/ajax/libs/react/15.1.0/react-dom.js") {}

                        //         // script(type="text/javascript", src="/assets/vendor.js") {}
                        //         script(type="text/javascript", src="/assets/deck_review.js") {}
                        //     };

                        // },
                        AppRoute::Deck(_, DeckRoute::NewDeck) =>  {
                            tmpl << html! {
                                // script(type="text/javascript", src="https://cdnjs.cloudflare.com/ajax/libs/babel-polyfill/6.9.1/polyfill.min.js") {}
                                // script(type="text/javascript", src="https://cdnjs.cloudflare.com/ajax/libs/react/15.3.0/react.js") {}
                                // script(type="text/javascript", src="https://cdnjs.cloudflare.com/ajax/libs/react/15.3.0/react-dom.js") {}

                                // script(type="text/javascript") {
                                //     // needs to be raw b/c of html escaping
                                //     : raw!(format!("window.__PRE_RENDER_STATE__ = {};",
                                //         view_route_to_pre_render_state(context.view_route.clone(), context)))
                                // }

                                script(type="text/javascript") {
                                    |tmpl| {
                                        pre_render_state(tmpl, context.clone(), &app_route);
                                    }
                                }

                                script(type="text/javascript", src="/assets/new_deck.js") {}
                            };

                        },
                        _ => {}
                    };
                }

            }
        }

    };
}

#[inline]
fn DeckPath(tmpl: &mut TemplateBuffer, context: Rc<RefCell<Context>>, deck_id: DeckID, deck_route: &DeckRoute) {
    tmpl << html!{
        // TODO: path generator

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

#[inline]
fn DeckDetail(tmpl: &mut TemplateBuffer, context: Rc<RefCell<Context>>, deck_id: DeckID, deck_route: &DeckRoute) {
    tmpl << html!{
        div(class="column") {

            div(class="columns") {
                div(class="column") {
                    |tmpl| DeckPath(tmpl, context.clone(), deck_id, deck_route);
                }
            }

            |tmpl| {
                match *deck_route {
                    DeckRoute::Decks(ref deck_page_query, ref search) => {
                        DecksChildren(
                            tmpl,
                            context.clone(),
                            deck_id,
                            deck_page_query,
                            search
                        )
                    },
                    DeckRoute::NewDeck => {
                        NewDeck(
                            tmpl,
                            context.clone(),
                            deck_id
                        )
                    },
                    _ => {
                        // TODO: complete this shit
                    }
                }
            }

        }

        div(class="column is-one-quarter") {
            nav(class="panel") {
                p(class="panel-heading", style="font-weight: normal;") {
                    : raw!("Deck #");
                    : deck_id
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
                                a(href = view_route_to_link(context.clone(),
                                    AppRoute::Deck(deck_id, DeckRoute::Decks(Default::default(), Default::default()))),
                                    class? = classnames!(
                                        "is-bold",
                                        "is-active" => {
                                            // TODO: re-review this
                                            matches!(*deck_route, DeckRoute::NewDeck) ||
                                            matches!(*deck_route, DeckRoute::Decks(_, _))
                                        })
                                ) {
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

#[inline]
fn NewDeck(tmpl: &mut TemplateBuffer, context: Rc<RefCell<Context>>, deck_id: DeckID) {
    tmpl << html!{
        div(class="columns") {
            div(class="column") {
                h1(class="title") {
                    : raw!("Add New Deck")
                }
            }
        }

        div(id="new_deck_container") {}
    }
}

#[inline]
fn DecksChildren(tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>, deck_id: DeckID, deck_page_query: &DecksPageQuery, search: &Search) {
    tmpl << html!{

        div(class="columns") {
            div(class="column") {
                a(class="button is-bold is-success",
                    href = view_route_to_link(context.clone(), AppRoute::Deck(deck_id, DeckRoute::NewDeck))) {
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
}

#[inline]
fn DeckListItemComponent(tmpl: &mut TemplateBuffer, is_bottom: bool) {
    tmpl << html!{
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
    tmpl << html!{
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
