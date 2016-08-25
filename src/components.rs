/* rust lib imports */

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

use serde_json;

/* local imports */

use route::{AppRoute, RenderResponse, DeckRoute, DeckSettings};
use context::{self, Context};
use types::{DeckID, DecksPageQuery, Search, Pagination, SortOrderable};
use api::decks;

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
                DeckRoute::Description => format!("/deck/{}/description", deck_id),
                DeckRoute::Settings(ref setting_mode) => {
                    match *setting_mode {
                        DeckSettings::Main => format!("/deck/{}/settings", deck_id),
                        DeckSettings::Move => format!("/deck/{}/settings/move", deck_id),
                    }
                },
                DeckRoute::Decks(page_query, search) => {

                    let mut query = page_query.generate_query_string();


                    if let Some(search_query) = search.generate_query_string() {
                        query = query + &format!("&{}", search_query);
                    }

                    format!("/deck/{deck_id}/decks?{query_string}", deck_id = deck_id, query_string = query)

                },

                // TODO: remove
                _ => panic!("fix")
            }
        },
        _ => {
            // TODO: remove
            panic!("fix this");
        }
    }
}

/* javascript generator */

#[derive(Serialize)]
struct MarkdownContents {
    MARKDOWN_CONTENTS: String
}

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
                                        POST_TO: '/api/deck/{}/new/deck'\
                                    }};\
                                ",
                                deck_id
                            )
                        )
                    }
                },
                DeckRoute::Description => {

                    let markdown_contents: String = match decks::get_deck(context.clone(), deck_id) {
                        Ok(deck) => {
                            let markdown_contents = MarkdownContents {
                                MARKDOWN_CONTENTS: deck.description
                            };
                            serde_json::to_string(&markdown_contents).unwrap()
                        },
                        Err(_) => {
                            // TODO: internal error logging
                            panic!();
                        }
                    };

                    tmpl << html! {
                        : raw!(
                            format!(
                                "window.__PRE_RENDER_STATE__ = \
                                    {{\
                                        POST_TO: '/api/deck/{deck_id}/description',\
                                        DECK_DESCRIPTION: {markdown_contents}\
                                    }};\
                                ",
                                deck_id = deck_id, markdown_contents = markdown_contents
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

                // TODO: necessary?
                // style(type="text/css") {
                //     : raw!(include_str!("../assets/bulma.css"))
                // }

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
                            ");

                            : raw!("\
                                span.mathjax_inline_pre {\
                                    font-weight: normal;\
                                }\
                                span.mathjax_inline {\
                                    font-weight: normal;\
                                }\
                            ")
                        }
                    }
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
                            // TODO: what is this?
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
                                        // TODO: complete this
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

                script(type="text/javascript", src="/assets/mathjax_inline.js") {}

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
                        AppRoute::Deck(_, DeckRoute::Description) =>  {
                            tmpl << html! {
                                script(type="text/javascript") {
                                    |tmpl| {
                                        pre_render_state(tmpl, context.clone(), &app_route);
                                    }
                                }

                                script(type="text/javascript", src="/assets/deck_description.js") {}
                            }
                        },
                        _ => {
                            // NOTE: No script here
                        }
                    };
                }

            }
        }

    };
}

#[inline]
fn DeckPath(tmpl: &mut TemplateBuffer, context: Rc<RefCell<Context>>, deck_id: DeckID, deck_route: &DeckRoute) {

    let deck_path = match decks::get_path_of_deck(context.clone(), deck_id) {
        Ok(path) => path,
        Err(_) => {
            // TODO: internal error logging
            panic!();
        }
    };

    let mut is_first = true;

    tmpl << html!{
        @ for deck_id in deck_path {
            span(class="title is-5 is-marginless", style="font-weight:normal;") {
                |tmpl| {
                    if is_first {
                        is_first = false;
                        tmpl << html!{
                            : raw!("/ ");
                        }
                    } else {
                        tmpl << html!{
                            : raw!(" / ");
                        }
                    }
                }
            }
            |tmpl| {

                match decks::get_deck(context.clone(), deck_id) {
                    Ok(deck) => {
                        tmpl << html!{
                            span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                a(href = view_route_to_link(context.clone(),
                                    AppRoute::Deck(deck_id, DeckRoute::Decks(Default::default(), Default::default())))
                                ) {
                                    |tmpl| MathJaxInline(tmpl, deck.name.clone());
                                }
                            }
                        }
                    },
                    Err(_) => {
                        // TODO: internal error logging
                        panic!();
                    }
                }
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
                    DeckRoute::Description => {
                        DeckDescription(
                            tmpl,
                            context.clone(),
                            deck_id
                        )
                    },
                    DeckRoute::Settings(ref setting_mode) => {
                        DeckSettings(
                            tmpl,
                            context.clone(),
                            setting_mode,
                            deck_id
                        )
                    },
                    _ => {
                        // TODO: remove eventually
                        panic!();
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
                                a(href = view_route_to_link(context.clone(),
                                    AppRoute::Deck(deck_id,
                                        DeckRoute::Description)),
                                    class? = classnames!(
                                        "is-bold",
                                        "is-active" => {
                                            matches!(*deck_route, DeckRoute::Description)
                                        })
                                ) {
                                    : "Description"
                                }
                            }
                            li {
                                a(href = view_route_to_link(context.clone(),
                                    AppRoute::Deck(deck_id,
                                        DeckRoute::Decks(Default::default(), Default::default()))),
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
                                a(href = view_route_to_link(context.clone(),
                                    AppRoute::Deck(deck_id,
                                        DeckRoute::Settings(DeckSettings::Main))),
                                    class? = classnames!(
                                        "is-bold",
                                        "is-active" => {
                                            matches!(*deck_route, DeckRoute::Settings(_))
                                        })
                                ) {
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

        div(id="new_deck_container") {
            : raw!(include_str!("react_components/new_deck"))
        }
    }
}

#[inline]
fn DeckDescription(tmpl: &mut TemplateBuffer, context: Rc<RefCell<Context>>, deck_id: DeckID) {

    let description = match decks::get_deck(context.clone(), deck_id) {
        Ok(deck) => {
            deck.description
        },
        Err(_) => {
            // TODO: internal error logging
            panic!();
        }
    };

    tmpl << html!{
        div(class="columns") {
            div(class="column") {
                h1(class="title") {
                    : raw!("Deck Description")
                }
            }
        }

        div(id="deck_description_container") {
            : raw!(include_str!("react_components/deck_description"))
        }

        div(class="columns", id="deck_description_container_stub", style="margin-top: 10px;") {
            div(class="column") {
                : description
            }
        }
    }
}

#[inline]
fn DeckSettings(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    setting_mode: &DeckSettings,
    deck_id: DeckID) {

    let deck = match decks::get_deck(context.clone(), deck_id) {
        Ok(deck) => deck,
        Err(_) => {
            // TODO: internal error logging
            panic!();
        }
    };

    tmpl << html!{
        div(class="columns") {
            div(class="column") {
                h1(class="title") {
                    : raw!("Deck Settings")
                }
            }
        }

        |tmpl| DeckSettingsNav(tmpl, context.clone(), &setting_mode, deck_id);

        |tmpl| {
            match *setting_mode {
                DeckSettings::Main => DeckSettingsMain(tmpl, context.clone(), deck_id),
                DeckSettings::Move => DeckSettingsMove(tmpl, context.clone(), deck_id),
            }
        }

        div(id="settings_deck_name_container") {
            // : raw!(include_str!("react_components/deck_description"))
        }

        div(class="columns", id="settings_deck_name_container_stub", style="margin-top: 10px;") {
            div(class="column") {
                : &deck.name
            }
        }
    }
}

#[inline]
fn DeckSettingsNav(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    setting_mode: &DeckSettings,
    deck_id: DeckID) {

    tmpl << html!{

        div(class="columns") {
            div(class="column") {
                div(class="tabs is-boxed") {
                    ul {
                        li(
                            class? = classnames!(
                                "is-active" => {
                                    matches!(*setting_mode, DeckSettings::Main)
                                })
                            ) {
                            a(href = view_route_to_link(
                                context.clone(),
                                AppRoute::Deck(deck_id, DeckRoute::Settings(DeckSettings::Main))
                                )) {
                                span {
                                    : "General"
                                }
                            }
                        }
                        li(
                            class? = classnames!(
                                "is-active" => {
                                    matches!(*setting_mode, DeckSettings::Move)
                                })
                            ) {
                            a(href = view_route_to_link(
                                context.clone(),
                                AppRoute::Deck(deck_id, DeckRoute::Settings(DeckSettings::Move))
                                )) {
                                span {
                                    : "Move"
                                }
                            }
                        }
                    }
                }
            }
        }

    };

}

#[inline]
fn DeckSettingsMain(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    deck_id: DeckID
    ) {
    tmpl << html!{
        : "main"
    }
}

#[inline]
fn DeckSettingsMove(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    deck_id: DeckID
    ) {
    tmpl << html!{
        : "move"
    }
}


#[inline]
fn DecksChildren(tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    deck_page_query: &DecksPageQuery,
    search: &Search) {

    tmpl << html!{

        div(class="columns") {
            div(class="column") {
                div(class="level") {
                    div(class="level-left") {
                        div(class="level-item") {
                            a(class="button is-bold is-success",
                                href = view_route_to_link(context.clone(),
                                    AppRoute::Deck(deck_id, DeckRoute::NewDeck))) {
                                : raw!("New Deck")
                            }
                        }
                    }

                    div(class="level-right") {
                        div(class="level-item") {
                            span(class="select") {
                                select(onchange="if(this.value && this.value.length > 0) window.location.href = this.value;") {
                                    option(value = "") {
                                        : format!("Order By: {}", deck_page_query.sort_order_string())
                                    }
                                    option(
                                        value = view_route_to_link(context.clone(),
                                            AppRoute::Deck(deck_id,
                                                DeckRoute::Decks(deck_page_query.descending(), search.clone())))
                                    ) {
                                        : deck_page_query.descending().sort_order_string()
                                    }
                                    option(
                                        value = view_route_to_link(context.clone(),
                                            AppRoute::Deck(deck_id,
                                                DeckRoute::Decks(deck_page_query.ascending(), search.clone())))
                                    ) {
                                        : deck_page_query.ascending().sort_order_string()
                                    }
                                }
                            }
                        }
                        div(class="level-item") {
                            span(class="select") {
                                select(onchange="if(this.value && this.value.length > 0) window.location.href = this.value;") {
                                    option(value="") {
                                        : format!("Sort By: {}", deck_page_query.sort_by_string())
                                    }

                                    option(
                                        value = view_route_to_link(context.clone(),
                                            AppRoute::Deck(deck_id,
                                                DeckRoute::Decks(deck_page_query.updated_at(),
                                                    search.clone())))
                                    ) {
                                        : deck_page_query.updated_at().sort_by_string()
                                    }

                                    option(
                                        value = view_route_to_link(context.clone(),
                                            AppRoute::Deck(deck_id,
                                                DeckRoute::Decks(deck_page_query.deck_title(),
                                                    search.clone())))
                                    ) {
                                        : deck_page_query.deck_title().sort_by_string()
                                    }

                                    option(
                                        value = view_route_to_link(context.clone(),
                                            AppRoute::Deck(deck_id,
                                                DeckRoute::Decks(deck_page_query.created_at(),
                                                    search.clone())))
                                    ) {
                                        : deck_page_query.created_at().sort_by_string()
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        |tmpl| DeckChildrenPaginationComponent(tmpl, context.clone(), deck_id, &deck_page_query, &search);
        |tmpl| DecksChildrenList(tmpl, context.clone(), deck_id, &deck_page_query, &search);
        |tmpl| DeckChildrenPaginationComponent(tmpl, context.clone(), deck_id, &deck_page_query, &search);
    }
}

#[inline]
#[inline]
fn DecksChildrenList(tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>, deck_id: DeckID, deck_page_query: &DecksPageQuery, search: &Search) {

    let children = match decks::get_deck_children(context.clone(), deck_id, deck_page_query, search) {
        Ok(children) => children,
        Err(_) => {
            // TODO: internal error logging
            panic!();
        }
    };

    let number_of_items = children.len();

    if number_of_items <= 0 {
        tmpl << html!{
            div(class="columns") {
                div(class="column") {
                    article(class="message") {
                        div(class="message-body") {
                            : raw!("There are no decks to display. You may create one within this deck.")
                        }
                    }
                }
            }
        };

        return;
    }

    tmpl << html!{
        @ for (index, deck_id) in children.iter().enumerate() {
            |tmpl| DeckListItemComponent(tmpl, context.clone(), *deck_id, (index + 1) >= number_of_items);
        }
    };
}

#[inline]
fn DeckListItemComponent(tmpl: &mut TemplateBuffer, context: Rc<RefCell<Context>>,
    deck_id: DeckID, is_bottom: bool) {

    let deck = match decks::get_deck(context.clone(), deck_id) {
        Ok(deck) => deck,
        Err(_) => {
            // TODO: internal error logging
            panic!();
        }
    };

    tmpl << html!{
        div(class="columns is-marginless",
            style=labels!(
                "border-bottom:1px dotted #d3d6db;" => !is_bottom)) {
            div(class="column is-side-paddingless") {
                h5(class="title is-5 is-marginless is-bold") {
                    a(href = view_route_to_link(context,
                                    AppRoute::Deck(deck_id,
                                        DeckRoute::Decks(Default::default(), Default::default())))
                    ) {
                        |tmpl| MathJaxInline(tmpl, deck.name.clone());
                    }
                }
                span(style="font-size:12px;") {
                    : format!("Deck #{}", deck.id);
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
fn DeckChildrenPaginationComponent(tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    deck_page_query: &DecksPageQuery,
    search: &Search) {

    if !deck_page_query.should_show_pagination(context.clone(), deck_id) {
        return;
    }

    let current_app_route = AppRoute::Deck(deck_id, DeckRoute::Decks(deck_page_query.clone(), search.clone()));
    let current_href = view_route_to_link(context.clone(), current_app_route);

    tmpl << html!{
        div(class="columns") {
            div(class="column") {
                nav(class="pagination") {

                    |tmpl| {

                        match deck_page_query.previous() {
                            None => {},
                            Some(page_query) => {

                                let app_route = AppRoute::Deck(deck_id, DeckRoute::Decks(page_query, search.clone()));
                                let href = view_route_to_link(context.clone(), app_route);

                                tmpl << html!(
                                    a(class="button is-bold", href = href) {
                                        : raw!("Previous")
                                    }
                                );
                            }
                        }

                    }

                    ul {

                        // trailing left side
                        |tmpl| {

                            match deck_page_query.get_trailing_left_side() {
                                None => {},
                                Some(list) => {
                                    tmpl << html!{
                                        @ for page_query in list {
                                            |tmpl| {

                                                let current_page = page_query.current_page();

                                                let app_route = AppRoute::Deck(deck_id,
                                                    DeckRoute::Decks(page_query, search.clone()));
                                                let href = view_route_to_link(context.clone(), app_route);

                                                tmpl << html!(
                                                    li {
                                                        a(class="button is-bold", href = href) {
                                                            : current_page
                                                        }
                                                    }
                                                );
                                            }
                                        }
                                    }

                                }
                            }

                        }

                        // trailing left side delimeter
                        |tmpl| {

                            if deck_page_query.has_trailing_left_side_delimeter() {
                                tmpl << html!{
                                    li {
                                        span(class="is-bold") {
                                            : "..."
                                        }
                                    }
                                }
                            }
                        }

                        // left side
                        |tmpl| {

                            tmpl << html!{
                                @ for page_query in deck_page_query.get_left_side() {
                                    |tmpl| {

                                        let current_page = page_query.current_page();

                                        let app_route = AppRoute::Deck(deck_id,
                                            DeckRoute::Decks(page_query, search.clone()));
                                        let href = view_route_to_link(context.clone(), app_route);

                                        tmpl << html!(
                                            li {
                                                a(class="button is-bold", href = href) {
                                                    : current_page
                                                }
                                            }
                                        );
                                    }
                                }
                            }

                        }

                        // current page
                        li {
                            a(class="button is-primary is-bold", href = current_href) {
                                : deck_page_query.current_page()
                            }
                        }

                        // right side
                        |tmpl| {

                            tmpl << html!{
                                @ for page_query in deck_page_query.get_right_side(context.clone(), deck_id) {
                                    |tmpl| {

                                        let current_page = page_query.current_page();

                                        let app_route = AppRoute::Deck(deck_id,
                                            DeckRoute::Decks(page_query, search.clone()));
                                        let href = view_route_to_link(context.clone(), app_route);

                                        tmpl << html!(
                                            li {
                                                a(class="button is-bold", href = href) {
                                                    : current_page
                                                }
                                            }
                                        );
                                    }
                                }
                            }

                        }

                        // trailing right side delimeter
                        |tmpl| {

                            if deck_page_query.has_trailing_right_side_delimeter(context.clone(), deck_id) {
                                tmpl << html!{
                                    li {
                                        span(class="is-bold") {
                                            : "..."
                                        }
                                    }
                                }
                            }
                        }

                        // trailing right side
                        |tmpl| {

                            match deck_page_query.get_trailing_right_side(context.clone(), deck_id) {
                                None => {},
                                Some(list) => {
                                    tmpl << html!{
                                        @ for page_query in list {
                                            |tmpl| {

                                                let current_page = page_query.current_page();

                                                let app_route = AppRoute::Deck(deck_id,
                                                    DeckRoute::Decks(page_query, search.clone()));
                                                let href = view_route_to_link(context.clone(), app_route);

                                                tmpl << html!(
                                                    li {
                                                        a(class="button is-bold", href = href) {
                                                            : current_page
                                                        }
                                                    }
                                                );
                                            }
                                        }
                                    }

                                }
                            }

                        }

                    }

                    |tmpl| {

                        match deck_page_query.next(context.clone(), deck_id) {
                            None => {},
                            Some(page_query) => {

                                let app_route = AppRoute::Deck(deck_id, DeckRoute::Decks(page_query, search.clone()));
                                let href = view_route_to_link(context.clone(), app_route);

                                tmpl << html!(
                                    a(class="button is-bold", href = href) {
                                        : raw!("Next")
                                    }
                                );
                            }
                        }

                    }

                }
            }
        }
    }
}

#[inline]
fn MathJaxInline(tmpl: &mut TemplateBuffer, content: String) {
    tmpl << html!{
        span(class="mathjax_inline_pre") {
            : content
        }
    }
}
