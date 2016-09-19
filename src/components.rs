/* rust lib imports */

use std::rc::Rc;
use std::cell::RefCell;

/* 3rd-party imports */

use horrorshow::{TemplateBuffer, Template};

use hyper::method::Method;
use hyper::server::{Server, Handler, Request, Response};
use hyper::header::{Headers, ContentType, TransferEncoding};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::uri::RequestUri;
use hyper::status::StatusCode;
use hyper::header::{Header, HeaderFormat};
use chrono::naive::datetime::NaiveDateTime;
use serde_json;

/* local imports */

use route::{AppRoute, RenderResponse, DeckRoute, CardRoute, DeckSettings, CardSettings};
use context::{self, Context};
use types::{DeckID, DecksPageQuery, CardID, CardsPageQuery, Search, Pagination, SortOrderable, MoveDecksPageQuery};
use api::{decks, cards, user};
use api::review::{self, CachedReviewProcedure};
use timestamp;

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

/* link generators */

#[inline]
fn go_back_up_deck(context: Rc<RefCell<Context>>, deck_id: DeckID) -> MoveDecksPageQuery {

    match user::get_root_deck(context.clone()) {
        Err(_why) => {
            // TODO: internal server error
            panic!();
        },
        Ok(None) => {
            // TODO: internal server error
            panic!();
        },
        Ok(Some(root_deck_id)) => {

            if deck_id == root_deck_id {
                return MoveDecksPageQuery::Root(root_deck_id);
            }

            match decks::get_parent_id_of_deck(context, deck_id) {
                Ok(Some(parent_deck_id)) => {
                    return MoveDecksPageQuery::SourceOfDecks(DecksPageQuery::default_with_deck(parent_deck_id));
                },
                Ok(None) => {

                    // NOTE: this should never occur
                    // TODO: internal server error
                    panic!();
                },
                Err(_why) => {
                    // TODO: internal server error
                    panic!();
                }
            }

        }
    }
}

#[inline]
fn card_route_string(card_route: CardRoute) -> String {
    match card_route {
        CardRoute::Contents => "contents".to_string(),
        CardRoute::Review => "review".to_string(),
        CardRoute::Stats => "stats".to_string(),
        CardRoute::Settings(ref card_settings) => {
            match *card_settings {
                CardSettings::Main => "settings".to_string(),
                CardSettings::Move(ref page_query, ref search) => {

                    let mut query = page_query.generate_query_string();

                    if let Some(search_query) = search.generate_query_string() {
                        query = query + &format!("&{search_query}", search_query = search_query);
                    }

                    format!("settings/move?{query_string}", query_string = query)
                }
            }
        }
    }
}

#[inline]
pub fn view_route_to_link(context: Rc<RefCell<Context>>, app_route: AppRoute) -> String {
    match app_route {
        AppRoute::Preferences => "/preferences".to_string(),
        AppRoute::Stashes => "/stashes".to_string(),
        AppRoute::Deck(deck_id, deck_route) => {
            match deck_route {
                DeckRoute::NewDeck => format!("/deck/{}/new/deck", deck_id),
                DeckRoute::NewCard => format!("/deck/{}/new/card", deck_id),
                DeckRoute::Description => format!("/deck/{}/description", deck_id),
                DeckRoute::Stats => format!("/deck/{}/stats", deck_id),
                DeckRoute::Review(_) => format!("/deck/{}/review", deck_id),
                DeckRoute::Settings(ref setting_mode) => {
                    match *setting_mode {
                        DeckSettings::Main => format!("/deck/{}/settings", deck_id),
                        DeckSettings::Move(ref page_query, ref search) => {

                            let mut query = page_query.generate_query_string();

                            if let Some(search_query) = search.generate_query_string() {
                                query = query + &format!("&{search_query}", search_query = search_query);
                            }

                            format!("/deck/{deck_id}/settings/move?{query_string}",
                                deck_id = deck_id, query_string = query)
                        }
                    }
                },
                DeckRoute::Cards(page_query, search) => {

                    let mut query = page_query.generate_query_string();

                    if let Some(search_query) = search.generate_query_string() {
                        query = query + &format!("&{}", search_query);
                    }

                    format!("/deck/{deck_id}/cards?{query_string}", deck_id = deck_id, query_string = query)
                },
                DeckRoute::Decks(page_query, search) => {

                    let mut query = page_query.generate_query_string();

                    if let Some(search_query) = search.generate_query_string() {
                        query = query + &format!("&{}", search_query);
                    }

                    format!("/deck/{deck_id}/decks?{query_string}", deck_id = deck_id, query_string = query)

                },
                DeckRoute::CardProfile(card_id, card_route) => {
                    format!("/deck/{deck_id}/card/{card_id}/{card_route}",
                        deck_id = deck_id, card_id = card_id, card_route = card_route_string(card_route))
                }
            }
        }
    }
}

#[inline]
pub fn generate_delete_to(app_route: &AppRoute) -> String {
    match *app_route {
        AppRoute::Deck(deck_id, ref deck_route) => {
            match *deck_route {
                DeckRoute::Settings(DeckSettings::Main) => {
                    format!("/api/deck/{deck_id}", deck_id = deck_id)
                },
                DeckRoute::CardProfile(card_id, ref card_route) => {

                    match *card_route {
                        CardRoute::Settings(ref card_settings) => {
                            match *card_settings {
                                CardSettings::Main => {
                                    format!("/api/card/{card_id}", card_id = card_id)
                                },
                                _ => {
                                    panic!("invalid use of generate_delete_to");
                                }
                            }

                        },
                        _ => {
                            panic!("invalid use of generate_delete_to");
                        }
                    }

                },
                _ => {
                    panic!("invalid use of generate_delete_to");
                }
            }
        },
        _ => {
            panic!("invalid use of generate_delete_to");
        }
    }
}

#[inline]
pub fn generate_post_to(app_route: &AppRoute) -> String {
    match *app_route {
        AppRoute::Deck(deck_id, ref deck_route) => {
            match *deck_route {
                DeckRoute::NewDeck => {
                    format!("/api/deck/{deck_id}/new/deck", deck_id = deck_id)
                },
                DeckRoute::NewCard => {
                    format!("/api/deck/{deck_id}/new/card", deck_id = deck_id)
                },
                DeckRoute::Review(_) => {
                    format!("/api/deck/{deck_id}/review", deck_id = deck_id)
                },
                DeckRoute::Description => {
                    format!("/api/deck/{deck_id}/description", deck_id = deck_id)
                },
                DeckRoute::Settings(DeckSettings::Main) => {
                    format!("/api/deck/{deck_id}/settings/name", deck_id = deck_id)
                },
                DeckRoute::CardProfile(card_id, ref card_route) => {
                    match *card_route {
                        CardRoute::Contents => {
                            format!("/api/card/{card_id}/update", card_id = card_id)
                        },
                        CardRoute::Review => {
                            format!("/api/card/{card_id}/review", card_id = card_id)
                        },
                        _ => {
                            panic!("invalid use of generate_post_to");
                        }
                    }
                }
                _ => {
                    panic!("invalid use of generate_post_to");
                }
            }
        },
        _ => {
            panic!("invalid use of generate_post_to");
        }
    }
}

#[inline]
pub fn generate_move_to(app_route: &AppRoute) -> String {
    match *app_route {
        AppRoute::Deck(deck_id, ref deck_route) => {
            match *deck_route {
                DeckRoute::CardProfile(card_id, ref card_route) => {
                    match *card_route {
                        CardRoute::Settings(CardSettings::Move(_, _)) => {
                            format!("/api/card/{card_id}/move", card_id = card_id)
                        },
                        _ => {
                            panic!("invalid use of generate_move_to");
                        }
                    }
                },
                DeckRoute::Settings(DeckSettings::Move(_, _)) => {
                    format!("/api/deck/{deck_id}/move", deck_id = deck_id)
                }
                _ => {
                    panic!("invalid use of generate_move_to");
                }
            }
        },
        _ => {
            panic!("invalid use of generate_move_to");
        }
    }
}

/* javascript generator */

#[derive(Serialize)]
struct MarkdownContents {
    MARKDOWN_CONTENTS: String
}

#[derive(Serialize)]
struct BoolContents {
    VALUE: bool
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
                                        POST_TO: '{post_to}'\
                                    }};\
                                ",
                                post_to = generate_post_to(app_route)
                            )
                        )
                    }
                },
                DeckRoute::NewCard => {
                    tmpl << html! {
                        : raw!(
                            format!(
                                "window.__PRE_RENDER_STATE__ = \
                                    {{\
                                        POST_TO: '{post_to}'\
                                    }};\
                                ",
                                post_to = generate_post_to(app_route)
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
                                        POST_TO: '{post_to}',\
                                        DECK_DESCRIPTION: {markdown_contents}\
                                    }};\
                                ",
                                post_to = generate_post_to(app_route),
                                markdown_contents = markdown_contents
                            )
                        )
                    }
                },
                DeckRoute::Settings(DeckSettings::Main) => {

                    let markdown_contents: String = match decks::get_deck(context.clone(), deck_id) {
                        Ok(deck) => {
                            let markdown_contents = MarkdownContents {
                                MARKDOWN_CONTENTS: deck.name
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
                                        NAME: {{\
                                            POST_TO: '{post_to_rename}',\
                                            DECK_NAME: {markdown_contents}\
                                        }},\
                                        DELETE: {{\
                                            DELETE_TO: '{delete_to}'
                                        }}\
                                    }};\
                                ",
                                post_to_rename = generate_post_to(app_route),
                                markdown_contents = markdown_contents,
                                delete_to = generate_delete_to(app_route)
                            )
                        )
                    }
                },
                DeckRoute::Settings(DeckSettings::Move(_, _)) => {
                    tmpl << html! {
                        : raw!(
                            format!(
                                "window.__PRE_RENDER_STATE__ = \
                                    {{\
                                        MOVE_TO: '{move_to}'\
                                    }};\
                                ",
                                move_to = generate_move_to(app_route)
                            )
                        )
                    }
                },
                DeckRoute::Review(ref card_for_review) => {

                    let (card_id, card_meta) = match *card_for_review {
                        None => {

                            tmpl << html! {
                                : raw!(
                                    format!(
                                        "window.__PRE_RENDER_STATE__ = \
                                            {{\
                                                POST_TO: '{post_to}',\
                                                HAS_CARD_FOR_REVIEW: false
                                            }};\
                                        ",
                                        post_to = generate_post_to(app_route)
                                    )
                                )
                            };

                            return;
                        },
                        Some((card_id, ref cached_review_procedure)) => {

                            let card_meta = match *cached_review_procedure {
                                None => "".to_string(),
                                Some(ref cached_review_procedure) => {
                                    serde_json::to_string(cached_review_procedure).unwrap()
                                }
                            };

                            (card_id, card_meta)
                        }
                    };

                    let (card_title,
                        card_description,
                        card_question,
                        card_answer,
                        card_is_active) = match cards::get_card(context.clone(), card_id) {
                        Ok(card) => {

                            let card_title = MarkdownContents {
                                MARKDOWN_CONTENTS: card.title
                            };
                            let card_title = serde_json::to_string(&card_title).unwrap();

                            let card_description = MarkdownContents {
                                MARKDOWN_CONTENTS: card.description
                            };
                            let card_description = serde_json::to_string(&card_description).unwrap();

                            let card_question = MarkdownContents {
                                MARKDOWN_CONTENTS: card.question
                            };
                            let card_question = serde_json::to_string(&card_question).unwrap();

                            let card_answer = MarkdownContents {
                                MARKDOWN_CONTENTS: card.answer
                            };
                            let card_answer = serde_json::to_string(&card_answer).unwrap();

                            let is_active = BoolContents {
                                VALUE: card.is_active
                            };
                            let is_active = serde_json::to_string(&is_active).unwrap();

                            (card_title, card_description, card_question, card_answer, is_active)
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
                                        POST_TO: '{post_to}',\
                                        PROFILE_URL: '{profile_url}',\
                                        CARD_ID: {card_id},\
                                        CARD_TITLE: {card_title},\
                                        CARD_DESCRIPTION: {card_description},\
                                        CARD_QUESTION: {card_question},\
                                        CARD_ANSWER: {card_answer},\
                                        CARD_IS_ACTIVE: {card_is_active},\
                                        CARD_META: {card_meta}\
                                    }};\
                                ",
                                post_to = generate_post_to(app_route),
                                profile_url = view_route_to_link(context.clone(),
                                    AppRoute::Deck(deck_id, DeckRoute::CardProfile(card_id, Default::default()))),
                                card_id = card_id,
                                card_title = card_title,
                                card_description = card_description,
                                card_question = card_question,
                                card_answer = card_answer,
                                card_is_active = card_is_active,
                                card_meta = card_meta
                            )
                        )
                    };

                },
                DeckRoute::CardProfile(card_id, ref card_route) => {
                    match *card_route {
                        CardRoute::Contents => {

                            let (title, question, answer, description, is_active, created_at, updated_at):
                                (String, String, String, String, String, String, String) =
                                match cards::get_card(context.clone(), card_id) {
                                Ok(card) => {

                                    let title = MarkdownContents {
                                        MARKDOWN_CONTENTS: card.title
                                    };
                                    let title = serde_json::to_string(&title).unwrap();

                                    let question = MarkdownContents {
                                        MARKDOWN_CONTENTS: card.question
                                    };
                                    let question = serde_json::to_string(&question).unwrap();

                                    let answer = MarkdownContents {
                                        MARKDOWN_CONTENTS: card.answer
                                    };
                                    let answer = serde_json::to_string(&answer).unwrap();

                                    let description = MarkdownContents {
                                        MARKDOWN_CONTENTS: card.description
                                    };
                                    let description = serde_json::to_string(&description).unwrap();

                                    let is_active = BoolContents {
                                        VALUE: card.is_active
                                    };
                                    let is_active = serde_json::to_string(&is_active).unwrap();

                                    let created_at = card.created_at;
                                    let updated_at = card.updated_at;

                                    (title, question, answer, description, is_active, created_at, updated_at)
                                },
                                Err(_) => {
                                    // TODO: internal error logging
                                    panic!();
                                }
                            };

                            let (seen_at, reviewed_at) = match review::get_card_score(context.clone(), card_id) {
                                Ok(card_score) => {

                                    let seen_at = card_score.seen_at;

                                    let reviewed_at = card_score.reviewed_at;

                                    (seen_at, reviewed_at)
                                },
                                Err(_) => {
                                    panic!();
                                }
                            };

                            tmpl << html! {
                                : raw!(
                                    format!(
                                        "window.__PRE_RENDER_STATE__ = \
                                            {{\
                                                POST_TO: '{post_to}',\
                                                CARD_TITLE: {title},\
                                                CARD_DESCRIPTION: {description},\
                                                CARD_QUESTION: {question},\
                                                CARD_ANSWER: {answer},\
                                                CARD_IS_ACTIVE: {is_active},\
                                                CREATED_AT: '{created_at}',\
                                                UPDATED_AT: '{updated_at}',\
                                                SEEN_AT: '{seen_at}',\
                                                REVIEWED_AT: '{reviewed_at}'\
                                            }};\
                                        ",
                                        post_to = generate_post_to(app_route),
                                        title = title,
                                        description = description,
                                        question = question,
                                        answer = answer,
                                        is_active = is_active,
                                        created_at = created_at,
                                        updated_at = updated_at,
                                        seen_at = seen_at,
                                        reviewed_at = reviewed_at
                                    )
                                )
                            }

                        },
                        CardRoute::Settings(ref card_settings) => {

                            match *card_settings {
                                CardSettings::Main => {

                                    tmpl << html! {
                                        : raw!(
                                            format!(
                                                "window.__PRE_RENDER_STATE__ = \
                                                    {{\
                                                        DELETE_TO: '{delete_to}'\
                                                    }};\
                                                ",
                                                delete_to = generate_delete_to(app_route)
                                            )
                                        )
                                    }

                                },
                                CardSettings::Move(_, _) => {

                                    tmpl << html! {
                                        : raw!(
                                            format!(
                                                "window.__PRE_RENDER_STATE__ = \
                                                    {{\
                                                        MOVE_TO: '{move_to}'\
                                                    }};\
                                                ",
                                                move_to = generate_move_to(app_route)
                                            )
                                        )
                                    }

                                }
                            }

                        },
                        CardRoute::Review => {

                            let (card_title,
                                card_description,
                                card_question,
                                card_answer) = match cards::get_card(context.clone(), card_id) {
                                Ok(card) => {

                                    let card_title = MarkdownContents {
                                        MARKDOWN_CONTENTS: card.title
                                    };
                                    let card_title = serde_json::to_string(&card_title).unwrap();

                                    let card_description = MarkdownContents {
                                        MARKDOWN_CONTENTS: card.description
                                    };
                                    let card_description = serde_json::to_string(&card_description).unwrap();

                                    let card_question = MarkdownContents {
                                        MARKDOWN_CONTENTS: card.question
                                    };
                                    let card_question = serde_json::to_string(&card_question).unwrap();

                                    let card_answer = MarkdownContents {
                                        MARKDOWN_CONTENTS: card.answer
                                    };
                                    let card_answer = serde_json::to_string(&card_answer).unwrap();

                                    (card_title, card_description, card_question, card_answer)
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
                                                POST_TO: '{post_to}',\
                                                CARD_ID: {card_id},\
                                                CARD_TITLE: {card_title},\
                                                CARD_DESCRIPTION: {card_description},\
                                                CARD_QUESTION: {card_question},\
                                                CARD_ANSWER: {card_answer}\
                                            }};\
                                        ",
                                        post_to = generate_post_to(app_route),
                                        card_id = card_id,
                                        card_title = card_title,
                                        card_description = card_description,
                                        card_question = card_question,
                                        card_answer = card_answer
                                    )
                                )
                            };


                        },
                        _ => {}
                    }
                },
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
                                    background-color: #ffffff;\
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
                                    font-weight: inherit;\
                                }\
                                span.mathjax_inline {\
                                    font-weight: inherit;\
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

                // TODO: moved to the body
                // script(type="text/javascript", async, src="/assets/mathjax/MathJax.js?config=TeX-AMS-MML_HTMLorMML") {}

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
                                                matches!(*app_route, AppRoute::Deck(_, _))
                                            }),
                                        href="/") {
                                        : raw!("Decks")
                                    }
                                }

                                // TODO: add when complete
                                // div(class="nav-item") {
                                //     a(
                                //         class? = classnames!(
                                //             "is-bold",
                                //             "button is-primary" => {
                                //                 // TODO: re-review this
                                //                 matches!(*app_route, AppRoute::Stashes)
                                //             }),
                                //         href="#") {
                                //         : raw!("Stashes")
                                //     }
                                // }

                                div(class="nav-item") {
                                    a(
                                        class? = classnames!(
                                            "is-bold",
                                            "button is-primary" => {
                                                // TODO: re-review this
                                                matches!(*app_route, AppRoute::Preferences)
                                            }),
                                        href = view_route_to_link(context.clone(), AppRoute::Preferences)) {
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
                                    AppRoute::Preferences => {
                                        Preferences(tmpl, context.clone())
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

                // scripts before body

                script(type="text/javascript", src="/assets/mathjax/MathJax.js?config=TeX-AMS-MML_HTMLorMML") {}

                script(type="text/javascript", src="/assets/commons.js") {}

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

                        AppRoute::Preferences =>  {
                            tmpl << html! {

                                script(type="text/javascript", src="/assets/backup.js") {}
                            };

                        },

                        AppRoute::Deck(_, DeckRoute::NewCard) =>  {
                            tmpl << html! {

                                script(type="text/javascript") {
                                    |tmpl| {
                                        pre_render_state(tmpl, context.clone(), &app_route);
                                    }
                                }

                                script(type="text/javascript", src="/assets/new_card.js") {}
                            };

                        },
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
                        AppRoute::Deck(deck_id, DeckRoute::Settings(DeckSettings::Main)) =>  {

                            tmpl << html! {

                                script(type="text/javascript") {
                                    |tmpl| {
                                        pre_render_state(tmpl, context.clone(), &app_route);
                                    }
                                }

                                script(type="text/javascript", src="/assets/deck_settings_main.js") {}
                            }

                        },
                        AppRoute::Deck(deck_id, DeckRoute::Settings(DeckSettings::Move(_, _))) =>  {

                            tmpl << html! {

                                script(type="text/javascript") {
                                    |tmpl| {
                                        pre_render_state(tmpl, context.clone(), &app_route);
                                    }
                                }

                                script(type="text/javascript", src="/assets/deck_move_settings.js") {}
                            }

                        },
                        AppRoute::Deck(_, DeckRoute::CardProfile(_card_id, ref card_route)) =>  {

                            match *card_route {
                                CardRoute::Contents => {

                                    tmpl << html! {

                                        script(type="text/javascript") {
                                            |tmpl| {
                                                pre_render_state(tmpl, context.clone(), &app_route);
                                            }
                                        }

                                        script(type="text/javascript", src="/assets/deck_card_profile.js") {}
                                    }

                                },
                                CardRoute::Review => {

                                    tmpl << html! {

                                        script(type="text/javascript") {
                                            |tmpl| {
                                                pre_render_state(tmpl, context.clone(), &app_route);
                                            }
                                        }

                                        script(type="text/javascript", src="/assets/card_review.js") {}
                                    }

                                },
                                CardRoute::Stats => {

                                    // TODO: complete

                                },
                                CardRoute::Settings(ref card_settings) => {

                                    match *card_settings {
                                        CardSettings::Main => {

                                            tmpl << html! {

                                                script(type="text/javascript") {
                                                    |tmpl| {
                                                        pre_render_state(tmpl, context.clone(), &app_route);
                                                    }
                                                }

                                                script(type="text/javascript", src="/assets/deck_card_settings.js") {}
                                            }

                                        },
                                        CardSettings::Move(_, _) => {

                                            tmpl << html! {

                                                script(type="text/javascript") {
                                                    |tmpl| {
                                                        pre_render_state(tmpl, context.clone(), &app_route);
                                                    }
                                                }

                                                script(type="text/javascript", src="/assets/card_move_settings.js") {}
                                            }

                                        }
                                    }
                                }
                            }

                        },
                        AppRoute::Deck(_, DeckRoute::Review(ref card_for_review)) =>  {

                            tmpl << html! {

                                script(type="text/javascript") {
                                    |tmpl| {
                                        pre_render_state(tmpl, context.clone(), &app_route);
                                    }
                                }

                                script(type="text/javascript", src="/assets/deck_review.js") {}
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
fn Preferences(tmpl: &mut TemplateBuffer, context: Rc<RefCell<Context>>) {

    tmpl << html!{

        div(class="column") {

            div(class="columns") {
                div(class="column") {
                    h2(class="title is-2") {
                        : raw!("Preferences")
                    }
                }
            }

            div(class="columns") {
                div(class="column") {
                    h3(class="title is-3") {
                        : raw!("Database backup")
                    }
                }
            }

            div(class="columns") {
                div(class="column") {
                    div(id="backup_container") {
                        a(class="button is-primary is-bold is-outlined") {
                            : raw!("Backup database")
                        }
                    }
                }
            }

        }
    }
}

#[inline]
fn generateDeckPathLink(context: Rc<RefCell<Context>>, deck_id: DeckID, deck_route: &DeckRoute) -> String {

    let __deck_route = match *deck_route {

        DeckRoute::NewCard |
        DeckRoute::NewDeck |
        DeckRoute::Description |
        DeckRoute::Stats |
        DeckRoute::Review(_) |
        DeckRoute::CardProfile(_, _) |
        DeckRoute::Settings(DeckSettings::Main) => {
            deck_route.clone()
        },
        DeckRoute::Settings(DeckSettings::Move(_, _)) => {

            match user::get_root_deck(context.clone()) {
                Ok(Some(root_deck_id)) => {

                    // NOTE: moving root deck is not possible.

                    if deck_id == root_deck_id {
                        DeckRoute::Settings(DeckSettings::Main)
                    } else {
                        deck_route.clone()
                    }
                },
                Ok(None) => {
                    panic!();
                },
                Err(_why) => {
                    panic!();
                }
            }

        },
        DeckRoute::Decks(ref _page_query, ref _search) => DeckRoute::Decks(
            DecksPageQuery::default_with_deck(deck_id), Default::default()),

        DeckRoute::Cards(ref _page_query, ref _search) => DeckRoute::Cards(
            CardsPageQuery::default_with_deck(deck_id), Default::default())
    };

    view_route_to_link(context, AppRoute::Deck(deck_id, __deck_route))
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

    let num_of_items = deck_path.len();

    tmpl << html!{
        div(style="margin-top:10px;") {
            @ for (index, deck_id) in deck_path.iter().enumerate() {
                span(class="title is-5 is-marginless", style="font-weight:normal;") {
                    |tmpl| {
                        if index == 0 {
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

                    match decks::get_deck(context.clone(), *deck_id) {
                        Ok(deck) => {

                            if (index + 1) >= num_of_items {

                                tmpl << html!{
                                    span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                        a(href = generateDeckPathLink(context.clone(), *deck_id, deck_route)
                                        ) {
                                            // NOTE: we wrap the mathjax-ified name with id of '__deck_name'.
                                            //       when renaming the deck, a react component can re-render this
                                            span(class="__deck_name", style="font-weight:bold;") {
                                                |tmpl| MathJaxInline(tmpl, deck.name.clone(), true);
                                            }
                                        }
                                    }
                                }

                            } else {

                                tmpl << html!{
                                    span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                        a(href = generateDeckPathLink(context.clone(), *deck_id, deck_route)
                                        ) {
                                            |tmpl| MathJaxInline(tmpl, deck.name.clone(), false);
                                        }
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

            |tmpl| {

                match *deck_route {
                    DeckRoute::Review(ref card_for_review) => {
                            tmpl << html!{

                                span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                    : raw!(" / ");
                                }

                                span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                    : raw!("Review")
                                }
                            }
                    },
                    DeckRoute::Description => {
                            tmpl << html!{

                                span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                    : raw!(" / ");
                                }

                                span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                    : raw!("Deck Description")
                                }
                            }
                    },
                    DeckRoute::Decks(_, _) => {
                            tmpl << html!{

                                span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                    : raw!(" / ");
                                }

                                span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                    : raw!("Decks")
                                }
                            }
                    },
                    DeckRoute::Cards(_, _) => {
                            tmpl << html!{

                                span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                    : raw!(" / ");
                                }

                                span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                    : raw!("Cards")
                                }
                            }
                    },
                    DeckRoute::NewCard => {
                            tmpl << html!{

                                span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                    : raw!(" / ");
                                }

                                span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                    : raw!("Add New Card")
                                }
                            }
                    },
                    DeckRoute::NewDeck => {
                            tmpl << html!{

                                span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                    : raw!(" / ");
                                }

                                span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                    : raw!("Add New Deck")
                                }
                            }
                    },
                    DeckRoute::Settings(ref deck_settings) => {
                            tmpl << html!{

                                span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                    : raw!(" / ");
                                }

                                span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                    : raw!("Settings")
                                }

                                |tmpl| {

                                    match *deck_settings {
                                        DeckSettings::Move(_, _) => {

                                            tmpl << html!{
                                                span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                                    : raw!(" / ");
                                                }

                                                span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                                    : raw!("Move Deck")
                                                }
                                            }
                                        },
                                        _ => {
                                            // nothing
                                        }
                                    }
                                }
                            }
                    },
                    DeckRoute::Stats => {
                            tmpl << html!{

                                span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                    : raw!(" / ");
                                }

                                span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                    : raw!("Statistics")
                                }
                            }
                    },
                    DeckRoute::CardProfile(card_id, ref card_route) => {

                        tmpl << html!{

                            span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                : raw!(" / ");
                            }

                            span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                : raw!(format!("Card #{}", card_id))
                            }

                            |tmpl| {

                                match *card_route {
                                    CardRoute::Contents => {
                                        // no-op
                                    },
                                    CardRoute::Review => {

                                        tmpl << html!{

                                            span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                                : raw!(" / ");
                                            }

                                            span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                                : raw!("Review")
                                            }

                                        }
                                    },
                                    CardRoute::Stats => {

                                        tmpl << html!{

                                            span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                                : raw!(" / ");
                                            }

                                            span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                                : raw!("Statistics")
                                            }

                                        }
                                    },
                                    CardRoute::Settings(ref card_settings) => {

                                        tmpl << html!{

                                            span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                                : raw!(" / ");
                                            }

                                            span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                                : raw!("Settings")
                                            }

                                            |tmpl| {

                                                match *card_settings {
                                                    CardSettings::Main => {
                                                        // no-op
                                                    },
                                                    CardSettings::Move(_, _) => {

                                                        tmpl << html!{
                                                            span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                                                : raw!(" / ");
                                                            }

                                                            span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                                                : raw!("Move")
                                                            }
                                                        }


                                                    },
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
        }
    }

}

#[inline]
fn DeckDetail(tmpl: &mut TemplateBuffer, context: Rc<RefCell<Context>>, deck_id: DeckID, deck_route: &DeckRoute) {
    tmpl << html!{


        div(class="column is-one-quarter") {

            |tmpl| CardDetailNav(tmpl, context.clone(), deck_id, deck_route);

            nav(class="panel") {
                p(class="panel-heading is-bold") {
                    : raw!("Deck #");
                    : deck_id
                }
                div(class="panel-block") {

                    aside(class="menu") {
                        // p(class="menu-label is-bold") {
                        //     : raw!("Deck #123")
                        // }
                        ul(class="menu-list") {
                            li(style = raw!("padding-bottom:2px;")) {
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
                            li(style = raw!("padding-top:2px;padding-bottom:2px;")) {
                                a(href = view_route_to_link(context.clone(),
                                    AppRoute::Deck(deck_id,
                                        DeckRoute::Decks(DecksPageQuery::default_with_deck(deck_id),
                                            Default::default()))),
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
                            li(style = raw!("padding-top:2px;padding-bottom:2px;")) {
                                a(href = view_route_to_link(context.clone(),
                                    AppRoute::Deck(deck_id,
                                        DeckRoute::Cards(CardsPageQuery::default_with_deck(deck_id),
                                            Default::default()))),
                                    class? = classnames!(
                                        "is-bold",
                                        "is-active" => {
                                            matches!(*deck_route, DeckRoute::NewCard) ||
                                            matches!(*deck_route, DeckRoute::Cards(_, _))
                                        })
                                ) {
                                    : "Cards"
                                }
                            }
                            li(style = raw!("padding-top:2px;padding-bottom:2px;")) {
                                a(href = view_route_to_link(context.clone(),
                                    AppRoute::Deck(deck_id,
                                        DeckRoute::Stats)),
                                    class? = classnames!(
                                        "is-bold",
                                        "is-active" => {
                                            matches!(*deck_route, DeckRoute::Stats)
                                        })
                                ) {
                                    : "Stats"
                                }
                            }
                            li(style = raw!("padding-top:2px;")) {
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
                div(class="panel-block") {

                    a(href = view_route_to_link(context.clone(),
                        AppRoute::Deck(deck_id,
                            DeckRoute::Review(None))),
                        class? = classnames!(
                            "is-bold button is-primary is-fullwidth is-medium",
                            "is-outlined" => {
                                !matches!(*deck_route, DeckRoute::Review(_))
                            })
                    ) {
                        : "Review Deck"
                    }

                }
            }
        }

        div(class="column") {

            div(class="columns") {
                div(class="column") {
                    |tmpl| DeckPath(tmpl, context.clone(), deck_id, deck_route);
                }
            }

            |tmpl| {
                match *deck_route {
                    DeckRoute::Decks(ref decks_page_query, ref search) => {
                        DecksChildren(
                            tmpl,
                            context.clone(),
                            deck_id,
                            decks_page_query,
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
                    DeckRoute::NewCard => {
                        NewCard(
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
                    DeckRoute::Cards(ref cards_page_query, ref search) => {
                        DeckCards(
                            tmpl,
                            context.clone(),
                            deck_id,
                            cards_page_query,
                            search
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
                    DeckRoute::CardProfile(card_id, ref card_route) => {
                        CardDetail(
                            tmpl,
                            context.clone(),
                            deck_id,
                            card_id,
                            card_route
                        )
                    },
                    DeckRoute::Review(ref card_for_review) => {
                        DeckReview(
                            tmpl,
                            context.clone(),
                            deck_id,
                            card_for_review
                        )
                    },
                    DeckRoute::Stats => {
                        DeckStats(
                            tmpl,
                            context.clone(),
                            deck_id
                        )
                    }
                }
            }

        }

    }
}

#[inline]
fn CardDetailNav(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    deck_route: &DeckRoute) {

    match *deck_route {
        DeckRoute::CardProfile(card_id, ref card_route) => {
            tmpl << html!{

                nav(class="panel") {
                    p(class="panel-heading is-bold") {
                        : raw!("Card #");
                        : card_id
                    }
                    div(class="panel-block") {

                        aside(class="menu") {
                            ul(class="menu-list") {
                                li(style = raw!("padding-bottom:2px;")) {
                                    a(href = view_route_to_link(context.clone(),
                                        AppRoute::Deck(deck_id,
                                            DeckRoute::CardProfile(card_id, CardRoute::Contents))),
                                        class? = classnames!(
                                            "is-bold",
                                            "is-active" => {
                                                matches!(*deck_route,
                                                    DeckRoute::CardProfile(_, CardRoute::Contents))
                                            })
                                    ) {
                                        : "Contents"
                                    }
                                }
                                li(style = raw!("padding-top:2px;padding-bottom:2px;")) {
                                    a(href = view_route_to_link(context.clone(),
                                        AppRoute::Deck(deck_id,
                                            DeckRoute::CardProfile(card_id, CardRoute::Stats))),
                                        class? = classnames!(
                                            "is-bold",
                                            "is-active" => {
                                                matches!(*deck_route,
                                                    DeckRoute::CardProfile(_, CardRoute::Stats))
                                            })
                                    ) {
                                        : "Stats"
                                    }
                                }
                                li(style = raw!("padding-top:2px;")) {
                                    a(href = view_route_to_link(context.clone(),
                                        AppRoute::Deck(deck_id,
                                            DeckRoute::CardProfile(card_id, CardRoute::Settings(CardSettings::Main)))),
                                        class? = classnames!(
                                            "is-bold",
                                            "is-active" => {
                                                matches!(*deck_route,
                                                    DeckRoute::CardProfile(_,
                                                        CardRoute::Settings(_)))
                                            })
                                    ) {
                                        : "Settings"
                                    }
                                }
                            }
                        }

                    }

                    div(class="panel-block") {

                        a(href = view_route_to_link(context.clone(),
                            AppRoute::Deck(deck_id,
                                DeckRoute::CardProfile(card_id, CardRoute::Review))),

                            class? = classnames!(
                                "is-bold button is-primary is-fullwidth is-medium",
                                "is-outlined" => {
                                    !matches!(*deck_route, DeckRoute::CardProfile(_, CardRoute::Review))
                                })

                        ) {
                            : "Review Card"
                        }

                    }
                }

            }
        },
        _ => {
            // Do not render card nav detail
        }
    }

}

#[inline]
fn NewDeck(tmpl: &mut TemplateBuffer, context: Rc<RefCell<Context>>, deck_id: DeckID) {
    tmpl << html!{

        // TODO: remove
        // div(class="columns") {
        //     div(class="column") {
        //         h1(class="title") {
        //             : raw!("Add New Deck")
        //         }
        //     }
        // }

        div(id="new_deck_container") {
            : raw!(include_str!("react_components/new_deck"))
        }
    }
}

#[inline]
fn NewCard(tmpl: &mut TemplateBuffer, context: Rc<RefCell<Context>>, deck_id: DeckID) {
    tmpl << html!{

        // TODO: remove
        // div(class="columns") {
        //     div(class="column") {
        //         h1(class="title") {
        //             : raw!("Add New Card")
        //         }
        //     }
        // }

        div(id="new_card_container") {
            // TODO: fix
            // : raw!(include_str!("react_components/new_card"))
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

        // TODO: keep?
        // div(class="columns") {
        //     div(class="column") {
        //         h1(class="title") {
        //             : raw!("Deck Description")
        //         }
        //     }
        // }

        div(id="deck_description_container") {
            : raw!(include_str!("react_components/deck_description"))
        }

        div(id="deck_description_container_stub", style="margin-top: 10px;") {

            // NOTE: this mocks react component

            div(class="columns") {
                div(class="column") {
                    a(class="button is-primary is-bold") {
                        : raw!("Preview")
                    }
                    : raw!(" ");
                    a(class="button is-bold") {
                        : raw!("Source")
                    }
                }
            }

            div(class="columns") {
                div(class="column", style="margin-top:20px;margin-bottom:20px;") {
                    |tmpl| {
                        if description.trim().len() <= 0 {

                            tmpl << html!{
                                div(class="message is-info") {
                                    div(class="message-body") {
                                        : raw!("No description set for this deck. \
                                            Click \"Edit\" button to add a description.");
                                    }
                                }
                            };

                        } else {

                            tmpl << html! {
                                : description
                            };
                        }
                    }
                }
            }
        }

    }
}

#[inline]
fn DeckCards(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    cards_page_query: &CardsPageQuery,
    search: &Search) {

    tmpl << html!{

        // TODO: remove
        // div(class="columns") {
        //     div(class="column") {
        //         h1(class="title") {
        //             : raw!("Cards")
        //         }
        //     }
        // }

        div(class="columns") {
            div(class="column") {
                div(class="level") {
                    div(class="level-left") {
                        div(class="level-item") {
                            a(class="button is-bold is-success",
                                href = view_route_to_link(context.clone(),
                                    AppRoute::Deck(deck_id, DeckRoute::NewCard))) {
                                : raw!("New Card")
                            }
                        }
                    }

                    div(class="level-right") {
                        div(class="level-item") {
                            span(class="select") {
                                select(onchange="if(this.value && this.value.length > 0) window.location.href = this.value;") {
                                    option(value = "") {
                                        : format!("Order By: {}", cards_page_query.sort_order_string())
                                    }
                                    option(
                                        value = view_route_to_link(context.clone(),
                                            AppRoute::Deck(deck_id,
                                                DeckRoute::Cards(cards_page_query.descending(), search.clone())))
                                    ) {
                                        : cards_page_query.descending().sort_order_string()
                                    }
                                    option(
                                        value = view_route_to_link(context.clone(),
                                            AppRoute::Deck(deck_id,
                                                DeckRoute::Cards(cards_page_query.ascending(), search.clone())))
                                    ) {
                                        : cards_page_query.ascending().sort_order_string()
                                    }
                                }
                            }
                        }
                        div(class="level-item") {
                            span(class="select") {
                                select(onchange="if(this.value && this.value.length > 0) window.location.href = this.value;") {
                                    option(value="") {
                                        : format!("Sort By: {}", cards_page_query.sort_by_string())
                                    }

                                    option(
                                        value = view_route_to_link(context.clone(),
                                            AppRoute::Deck(deck_id,
                                                DeckRoute::Cards(cards_page_query.updated_at(),
                                                    search.clone())))
                                    ) {
                                        : cards_page_query.updated_at().sort_by_string()
                                    }

                                    option(
                                        value = view_route_to_link(context.clone(),
                                            AppRoute::Deck(deck_id,
                                                DeckRoute::Cards(cards_page_query.card_title(),
                                                    search.clone())))
                                    ) {
                                        : cards_page_query.card_title().sort_by_string()
                                    }

                                    option(
                                        value = view_route_to_link(context.clone(),
                                            AppRoute::Deck(deck_id,
                                                DeckRoute::Cards(cards_page_query.created_at(),
                                                    search.clone())))
                                    ) {
                                        : cards_page_query.created_at().sort_by_string()
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        |tmpl| CardsPaginationComponent(tmpl, context.clone(), deck_id, &cards_page_query, &search);
        |tmpl| CardsList(tmpl, context.clone(), deck_id, &cards_page_query, &search);
        |tmpl| CardsPaginationComponent(tmpl, context.clone(), deck_id, &cards_page_query, &search);
    }
}

#[inline]
fn CardsList(tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>, deck_id: DeckID, cards_page_query: &CardsPageQuery, search: &Search) {

    let children = match cards::cards_in_deck(context.clone(), deck_id, cards_page_query, search) {
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
                            : raw!("There are no cards to display. You may create one within this deck.")
                        }
                    }
                }
            }
        };

        return;
    }

    tmpl << html!{
        @ for (index, card) in children.iter().enumerate() {
            |tmpl| CardListItemComponent(tmpl, context.clone(), deck_id, card.id, (index + 1) >= number_of_items);
        }
    };
}

#[inline]
fn CardListItemComponent(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    card_id: CardID,
    is_bottom: bool) {

    let card = match cards::get_card(context.clone(), card_id) {
        Ok(card) => card,
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
                    a(href = view_route_to_link(context.clone(),
                                    AppRoute::Deck(deck_id, DeckRoute::CardProfile(card_id, CardRoute::Contents)))
                    ) {
                        |tmpl| MathJaxInline(tmpl, card.title.clone(), false);
                    }
                }
                span(style="font-size:12px;") {
                    : format!("Card #{}", card.id);
                }
            }
        }
    }
}

#[inline]
fn CardsPaginationComponent(tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    cards_page_query: &CardsPageQuery,
    search: &Search) {

    if !cards_page_query.should_show_pagination(context.clone()) {
        return;
    }

    let current_app_route = AppRoute::Deck(deck_id, DeckRoute::Cards(cards_page_query.clone(), search.clone()));
    let current_href = view_route_to_link(context.clone(), current_app_route);

    tmpl << html!{
        div(class="columns") {
            div(class="column") {
                nav(class="pagination") {

                    |tmpl| {

                        match cards_page_query.previous() {
                            None => {},
                            Some(page_query) => {

                                let app_route = AppRoute::Deck(deck_id,
                                    DeckRoute::Cards(page_query, search.clone()));
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

                            match cards_page_query.get_trailing_left_side() {
                                None => {},
                                Some(list) => {
                                    tmpl << html!{
                                        @ for page_query in list {
                                            |tmpl| {

                                                let current_page = page_query.current_page();

                                                let app_route = AppRoute::Deck(deck_id,
                                                    DeckRoute::Cards(page_query, search.clone()));
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

                            if cards_page_query.has_trailing_left_side_delimeter() {
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
                                @ for page_query in cards_page_query.get_left_side() {
                                    |tmpl| {

                                        let current_page = page_query.current_page();

                                        let app_route = AppRoute::Deck(deck_id,
                                            DeckRoute::Cards(page_query, search.clone()));
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
                                : cards_page_query.current_page()
                            }
                        }

                        // right side
                        |tmpl| {

                            tmpl << html!{
                                @ for page_query in cards_page_query.get_right_side(context.clone()) {
                                    |tmpl| {

                                        let current_page = page_query.current_page();

                                        let app_route = AppRoute::Deck(deck_id,
                                            DeckRoute::Cards(page_query, search.clone()));
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

                            if cards_page_query.has_trailing_right_side_delimeter(context.clone()) {
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

                            match cards_page_query.get_trailing_right_side(context.clone()) {
                                None => {},
                                Some(list) => {
                                    tmpl << html!{
                                        @ for page_query in list {
                                            |tmpl| {

                                                let current_page = page_query.current_page();

                                                let app_route = AppRoute::Deck(deck_id,
                                                    DeckRoute::Cards(page_query, search.clone()));
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

                        match cards_page_query.next(context.clone()) {
                            None => {},
                            Some(page_query) => {

                                let app_route = AppRoute::Deck(deck_id,
                                    DeckRoute::Cards(page_query, search.clone()));
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
fn DeckReview(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    card_for_review: &Option<(CardID, Option<CachedReviewProcedure>)>) {


    // TODO: remove

    // if card_for_review.is_none() {

    //     tmpl << html!{

    //         div(class="columns") {
    //             div(class="column") {
    //                 h1(class="title") {
    //                     : raw!("No cards in this deck to review")
    //                 }
    //             }
    //         }

    //         // TODO: remove
    //         // div(class="columns") {
    //         //     div(class="column") {
    //         //         article(class="message") {
    //         //             div(class="message-body") {
    //         //                 : raw!("There are no cards to review.")
    //         //             }
    //         //         }
    //         //     }
    //         // }

    //     };

    //     return;
    // }

    tmpl << html!{

        // TODO: remove
        // div(class="columns") {
        //     div(class="column") {
        //         h1(class="title") {
        //             : raw!("Reviewing Cards in this Deck")
        //         }
        //     }
        // }

        div(id="deck_review_container") {
            // TODO: fix
            // : raw!(include_str!("react_components/deck_review"))
        }

    };
}

#[inline]
fn DeckStats(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    deck_id: DeckID) {

    tmpl << html!{

        // TODO: remove
        // div(class="columns") {
        //     div(class="column") {
        //         h1(class="title") {
        //             : raw!("Deck Statistics")
        //         }
        //     }
        // }

    }
}

#[inline]
fn DeckSettings(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    setting_mode: &DeckSettings,
    deck_id: DeckID) {

    tmpl << html!{

        // TODO: remove
        // div(class="columns") {
        //     div(class="column") {
        //         h1(class="title") {
        //             : raw!("Deck Settings")
        //         }
        //     }
        // }

        |tmpl| DeckSettingsNav(tmpl, context.clone(), &setting_mode, deck_id);

        |tmpl| {
            match *setting_mode {
                DeckSettings::Main => DeckSettingsMain(tmpl, context.clone(), deck_id),
                DeckSettings::Move(ref page_query, ref search) => {
                    DeckSettingsMove(tmpl, context.clone(), deck_id, page_query, search)
                }
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
                                "is-active is-bold" => {
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

                        |tmpl| {

                            let should_show: bool = match user::get_root_deck(context.clone()) {
                                Ok(Some(root_deck_id)) => {
                                    root_deck_id != deck_id
                                },
                                Ok(None) => {
                                    panic!();
                                },
                                Err(why) => {
                                    // TODO: logging
                                    panic!()
                                }
                            };

                            if should_show {
                                tmpl << html! {
                                    li(
                                        class? = classnames!(
                                            "is-active is-bold" => {
                                                matches!(*setting_mode, DeckSettings::Move(_, _))
                                            })
                                        ) {
                                        a(href = view_route_to_link(
                                            context.clone(),
                                            AppRoute::Deck(deck_id, DeckRoute::Settings(DeckSettings::Move(
                                                MoveDecksPageQuery::default_with_parent_of_deck(context.clone(),
                                                    deck_id),
                                                Default::default())))
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
                h4(class="title is-4") {
                    : raw!("Deck Name")
                }
            }
        }

        div(id="deck_settings_main_name_container", style="margin-bottom:10px;") {
            // : raw!(include_str!("react_components/deck_description"))
        }

        |tmpl| {

            // get number of decks
            let (count_of_descendents, deck_noun) = match decks::get_num_descendents(context.clone(), deck_id) {
                Ok(count_of_descendents) => {

                    let deck_noun = if count_of_descendents == 1 {
                        "deck"
                    } else {
                        "decks"
                    };

                    (count_of_descendents, deck_noun)
                },
                Err(_) => {
                    // TODO: internal error logging
                    panic!();
                }
            };

            // get number of cards
            let (count_of_cards, card_noun) = match cards::total_num_of_cards_in_deck(context.clone(), deck_id) {
                Ok(count_of_cards) => {

                    let card_noun = if count_of_cards == 1 {
                        "card"
                    } else {
                        "cards"
                    };

                    (count_of_cards, card_noun)
                },
                Err(_) => {
                    // TODO: internal error logging
                    panic!();
                }
            };

            let is_root_deck = match user::is_root_deck(context.clone(), deck_id) {
                Ok(is_root_deck) => is_root_deck,
                Err(_) => {
                    panic!();
                }
            };

            if !is_root_deck {
                tmpl << html!{

                    div(class="columns") {
                        div(class="column") {
                            hr(class="is-marginless");
                        }
                    }

                    div(class="columns") {
                        div(class="column") {
                            h4(class="title is-4") {
                                : raw!("Delete Deck")
                            }

                            h2(class="subtitle") {
                                : raw!("This action is irreversible!")
                            }

                            p {
                                : raw!(format!(
                                    "By deleting this deck, {count_of_descendents} {deck_noun} \
                                    and {count_of_cards} {card_noun} will also be deleted.",
                                    count_of_descendents = count_of_descendents,
                                    deck_noun = deck_noun,
                                    count_of_cards = count_of_cards,
                                    card_noun = card_noun))
                            }
                        }
                    }

                    div(id="deck_settings_main_delete_container") {
                        // : raw!(include_str!("react_components/deck_description"))
                    }
                };
            }
        }

    }
}

#[inline]
fn DeckSettingsMove(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    deck_page_query: &MoveDecksPageQuery,
    search: &Search) {

    tmpl << html!{

        div(class="columns") {
            div(class="column") {

                h4(class="title is-4") {
                    : raw!("Move deck to a new deck")
                }

            }
        }

        div(class="columns is-marginless") {
            div(class="column is-side-paddingless") {

                div(class="level") {
                    div(class="level-left") {
                        div(class="level-item") {
                            strong(class="is-bold") {
                                : raw!("This deck's actual deck parent: ")
                            }
                        }

                        div(class="level-item") {
                            |tmpl| {

                                match decks::get_parent_id_of_deck(context.clone(), deck_id) {
                                    Ok(Some(deck_id)) => {
                                        VanillaRealDeckPath(tmpl, context.clone(), deck_id);
                                    },
                                    Ok(None) => {
                                        // TODO: logging
                                        panic!();
                                    },
                                    Err(_why) => {
                                        // TODO: logging
                                        panic!();
                                    }
                                }

                            }
                        }
                    }

                }

            }
        }

        div(class="columns is-marginless") {
            div(class="column is-side-paddingless") {
                |tmpl| DeckMovePathToDeck(tmpl, context.clone(), deck_id, &deck_page_query);
            }
        }

        |tmpl| DeckMovePaginationComponent(tmpl, context.clone(), deck_id, &deck_page_query, &search);
        |tmpl| DeckMoveDecksList(tmpl, context.clone(), deck_id, &deck_page_query, &search);
        |tmpl| DeckMovePaginationComponent(tmpl, context.clone(), deck_id, &deck_page_query, &search);

    }
}

#[inline]
fn DeckMovePaginationComponent(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    parent_deck: DeckID,
    deck_page_query: &MoveDecksPageQuery,
    search: &Search) {

    if !deck_page_query.should_show_pagination(context.clone()) {
        return;
    }

    let make_link = |deck_page_query: MoveDecksPageQuery| -> AppRoute {
        AppRoute::Deck(parent_deck, DeckRoute::Settings(
            DeckSettings::Move(deck_page_query, search.clone())))
    };

    let current_app_route = make_link(deck_page_query.clone());

    let current_href = view_route_to_link(context.clone(), current_app_route);

    tmpl << html!{
        div(class="columns") {
            div(class="column") {
                nav(class="pagination") {

                    |tmpl| {

                        match deck_page_query.previous() {
                            None => {},
                            Some(page_query) => {

                                let app_route = make_link(page_query);

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

                                                let app_route = make_link(page_query);

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

                                        let app_route = make_link(page_query);

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
                                @ for page_query in deck_page_query.get_right_side(context.clone()) {
                                    |tmpl| {

                                        let current_page = page_query.current_page();

                                        let app_route = make_link(page_query);

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

                            if deck_page_query.has_trailing_right_side_delimeter(context.clone()) {
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

                            match deck_page_query.get_trailing_right_side(context.clone()) {
                                None => {},
                                Some(list) => {
                                    tmpl << html!{
                                        @ for page_query in list {
                                            |tmpl| {

                                                let current_page = page_query.current_page();

                                                let app_route = make_link(page_query);

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

                        match deck_page_query.next(context.clone()) {
                            None => {},
                            Some(page_query) => {

                                let app_route = make_link(page_query);

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
fn DeckMoveDecksList(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    child_deck: DeckID,
    deck_page_query: &MoveDecksPageQuery,
    search: &Search) {

    // get root deck
    let root_deck_id = match user::get_root_deck(context.clone()) {
        Ok(Some(root_deck_id)) => root_deck_id,
        Ok(None) => {
            // TODO: internal error logging
            panic!();
        },
        Err(_) => {
            // TODO: internal error logging
            panic!();
        }
    };

    match *deck_page_query {
        MoveDecksPageQuery::Root(root_deck_id) => {
            // case: card is inside root deck

            // TODO: complete
            tmpl << html!{
                |tmpl| MoveDeckToDeckListItemComponent(tmpl,
                    context.clone(), child_deck, root_deck_id, true);
            };

            return;

        },
        MoveDecksPageQuery::SourceOfDecks(ref page_query) => {

            let &DecksPageQuery(deck_id, _, _) = page_query;

            // case: card is not inside root deck

            let children = match decks::get_deck_children(context.clone(), deck_id, page_query, search) {
                Ok(children) => children,
                Err(_) => {
                    // TODO: internal error logging
                    panic!();
                }
            };

            let number_of_items = children.len();

            if number_of_items <= 0 {
                tmpl << html!{

                    div(class="columns is-marginless") {
                        div(class="column is-side-paddingless") {
                            a(href = view_route_to_link(context.clone(),
                                        AppRoute::Deck(child_deck,
                                            DeckRoute::Settings(
                                                DeckSettings::Move(go_back_up_deck(context.clone(), deck_id),
                                                    Default::default())))),
                                class=raw!("is-bold button is-primary is-fullwidth is-outlined")
                            ) {
                                // TODO: phrasing?
                                : raw!("Go up one deck")
                            }
                        }
                    }

                    div(class="columns") {
                        div(class="column") {
                            article(class="message") {
                                div(class="message-body") {
                                    : raw!("There are no decks to display.")
                                }
                            }
                        }
                    }
                };

                return;
            }

            tmpl << html!{

                div(class="columns is-marginless") {
                    div(class="column is-side-paddingless") {
                        a(href = view_route_to_link(context.clone(),
                                    AppRoute::Deck(child_deck,
                                        DeckRoute::Settings(
                                            DeckSettings::Move(go_back_up_deck(context.clone(), deck_id),
                                                Default::default())))),
                            class=raw!("is-bold button is-primary is-fullwidth is-outlined")
                        ) {
                            // TODO: phrasing?
                            : raw!("Go up one deck")
                        }
                    }
                }

                @ for (index, deck_id) in children.iter().enumerate() {
                    |tmpl| MoveDeckToDeckListItemComponent(tmpl,
                        context.clone(), child_deck, *deck_id, (index + 1) >= number_of_items);
                }
            };


        }
    }

}

#[inline]
fn MoveDeckToDeckListItemComponent(tmpl: &mut TemplateBuffer, context: Rc<RefCell<Context>>,
    child_deck: DeckID, deck_id: DeckID, is_bottom: bool) {

    let deck = match decks::get_deck(context.clone(), deck_id) {
        Ok(deck) => deck,
        Err(_) => {
            // TODO: internal error logging
            panic!();
        }
    };

    let is_decks_parent = match decks::get_parent_id_of_deck(context.clone(), child_deck) {
        Err(_) => {
            // TODO: internal error logging
            panic!();
        },
        Ok(None) => {
            // TODO: internal error logging
            panic!();
        },
        Ok(Some(parent_deck_id)) => {
            parent_deck_id == deck_id
        }
    };

    let is_deck_descendent = match decks::is_descendent_of_deck(context.clone(), deck_id, child_deck) {
        Ok(is_deck_descendent) => is_deck_descendent,
        Err(_why) => {
            // TODO: logging
            panic!();
        }
    };

    tmpl << html!{
        div(class="columns is-marginless",
            style=labels!(
                "border-bottom:1px dotted #d3d6db;" => !is_bottom)) {
            div(class="column is-side-paddingless") {

                div(class="level is-marginless",
                    id = raw!(format!("{}-level", deck_id))) {

                    div(class="level-left") {

                        |tmpl| {
                            if is_decks_parent {

                                tmpl << html!{
                                    div(class="level-item") {
                                        span(class="tag is-dark is-bold") {
                                            : raw!("Current Deck")
                                        }
                                    }
                                }
                            }
                        }

                        |tmpl| {

                            if !is_decks_parent && !is_deck_descendent {
                                tmpl << html!{
                                    div(class="level-item", id = raw!(format!("{}-confirm", deck_id))) {
                                        // NOTE: confirm move button here; will be populated by react component
                                    }
                                }
                            } else {
                                tmpl << html!{
                                    div(class="level-item") {
                                        // NOTE: spacing fix
                                    }
                                }
                            }

                        }

                        div(class="level-item") {

                            h5(class="title is-5 is-marginless is-bold") {
                                a(href = view_route_to_link(context.clone(),
                                            AppRoute::Deck(child_deck,
                                                DeckRoute::Settings(
                                                    DeckSettings::Move(
                                                        MoveDecksPageQuery::SourceOfDecks(
                                                            DecksPageQuery::default_with_deck(deck_id)),
                                                        Default::default()))))

                                ) {
                                    |tmpl| MathJaxInline(tmpl, deck.name.clone(), is_decks_parent);
                                }
                            }

                            span(style="font-size:12px;") {
                                : format!("Deck #{}", deck.id);
                                : raw!(" ");
                                a(href = view_route_to_link(context.clone(),
                                                AppRoute::Deck(deck_id,
                                                    DeckRoute::Cards(CardsPageQuery::default_with_deck(deck_id),
                                                        Default::default())))) {
                                    : raw!("View Cards")
                                }
                            }

                        }

                    }

                    |tmpl| {
                        if !is_decks_parent && !is_deck_descendent {

                            tmpl << html!{
                                div(class="level-right") {
                                    div(class="level-item") {
                                        div(class="move_to_deck",
                                            data-deck-id = raw!(format!("{}", deck_id))) {

                                            // TODO: server rendered component
                                            a(class="button is-primary is-bold is-outlined") {
                                                : raw!("Move to this deck")
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }


                }

                |tmpl| {
                    if !is_decks_parent && !is_deck_descendent {

                        tmpl << html!{
                            div(id = raw!(format!("{}-error", deck_id))) {
                                // NOTE: rendered by react component
                            }
                        }
                    }
                }

            }
        }
    }

}

#[inline]
fn DeckMovePathToDeck(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    this_deck: DeckID,
    deck_page_query: &MoveDecksPageQuery) {

    match *deck_page_query {
        MoveDecksPageQuery::Root(_) => {
            tmpl << html!{
                span(class="title is-5 is-marginless", style="font-weight:normal;") {
                    : raw!("/");
                }
            };
        },
        MoveDecksPageQuery::SourceOfDecks(DecksPageQuery(deck_id, _, _)) => {

            let deck_path = match decks::get_path_of_deck(context.clone(), deck_id) {
                Ok(path) => path,
                Err(_) => {
                    // TODO: internal error logging
                    panic!();
                }
            };

            let num_of_items = deck_path.len();

            tmpl << html!{

                @ for (index, deck_id) in deck_path.iter().enumerate() {

                    span(class="title is-5 is-marginless", style="font-weight:normal;") {
                        |tmpl| {
                            if index == 0 {
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

                        match decks::get_deck(context.clone(), *deck_id) {
                            Err(_) => {
                                // TODO: internal error logging
                                panic!();
                            },
                            Ok(deck) => {

                                tmpl << html!{
                                    span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                        a(href = view_route_to_link(context.clone(),
                                            AppRoute::Deck(this_deck,
                                                DeckRoute::Settings(
                                                    DeckSettings::Move(
                                                        MoveDecksPageQuery::default_with_deck(context.clone(), deck.id),
                                                        Default::default()))))
                                        ) {
                                            |tmpl| MathJaxInline(tmpl, deck.name.clone(), false);
                                        }
                                    }
                                }

                            }
                        }

                    }

                }

            };

        }
    }
}

#[inline]
fn VanillaRealDeckPath(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    deck_id: DeckID) {

    let deck_path = match decks::get_path_of_deck(context.clone(), deck_id) {
        Ok(path) => path,
        Err(_) => {
            // TODO: internal error logging
            panic!();
        }
    };

    let num_of_items = deck_path.len();

    tmpl << html!{

        @ for (index, deck_id) in deck_path.iter().enumerate() {

            span(class="title is-5 is-marginless", style="font-weight:normal;") {
                |tmpl| {
                    if index == 0 {
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

                match decks::get_deck(context.clone(), *deck_id) {
                    Err(_) => {
                        // TODO: internal error logging
                        panic!();
                    },
                    Ok(deck) => {

                        tmpl << html!{
                            span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                |tmpl| MathJaxInline(tmpl, deck.name.clone(), false);
                            }
                        }

                    }
                }

            }

        }
    }
}

#[inline]
fn DecksChildren(tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    deck_page_query: &DecksPageQuery,
    search: &Search) {

    tmpl << html!{

        // TODO: keep?
        // div(class="columns") {
        //     div(class="column") {
        //         h1(class="title") {
        //             : raw!("Decks")
        //         }
        //     }
        // }

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
                                                DeckRoute::Decks(deck_page_query.deck_name(),
                                                    search.clone())))
                                    ) {
                                        : deck_page_query.deck_name().sort_by_string()
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
                    a(href = view_route_to_link(context.clone(),
                                    AppRoute::Deck(deck_id,
                                        DeckRoute::Decks(DecksPageQuery::default_with_deck(deck_id),
                                            Default::default())))
                    ) {
                        |tmpl| MathJaxInline(tmpl, deck.name.clone(), false);
                    }
                }
                span(style="font-size:12px;") {
                    : format!("Deck #{}", deck.id);
                    : raw!(" ");
                    a(href = view_route_to_link(context.clone(),
                                    AppRoute::Deck(deck_id,
                                        DeckRoute::Cards(CardsPageQuery::default_with_deck(deck.id),
                                            Default::default())))) {
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

    if !deck_page_query.should_show_pagination(context.clone()) {
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
                                @ for page_query in deck_page_query.get_right_side(context.clone()) {
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

                            if deck_page_query.has_trailing_right_side_delimeter(context.clone()) {
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

                            match deck_page_query.get_trailing_right_side(context.clone()) {
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

                        match deck_page_query.next(context.clone()) {
                            None => {},
                            Some(page_query) => {

                                let app_route = AppRoute::Deck(deck_id,
                                    DeckRoute::Decks(page_query, search.clone()));
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
fn CardDetail(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    card_id: CardID,
    card_route: &CardRoute) {

    match *card_route {
        CardRoute::Contents => CardDetailContents(tmpl, context, deck_id, card_id),
        CardRoute::Review => CardDetailReview(tmpl, context, deck_id, card_id),
        CardRoute::Stats => CardDetailStats(tmpl, context, deck_id, card_id),
        CardRoute::Settings(ref card_settings) => CardDetailSettings(tmpl, context, deck_id, card_id, card_settings),
    }


}

#[inline]
fn CardDetailContents(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    card_id: CardID) {

    tmpl << html!{
        div(id="card_profile_container") {
        }
    }
}

#[inline]
fn CardDetailReview(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    card_id: CardID) {

    tmpl << html!{

        // TODO: remove
        // div(class="columns") {
        //     div(class="column") {
        //         h1(class="title") {
        //             : raw!("Reviewing Card")
        //         }
        //     }
        // }

        div(id="card_review_container") {
            // TODO: static render
        }
    }
}

#[inline]
fn CardDetailStats(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    card_id: CardID) {

    tmpl << html!{

        // TODO: remove
        // div(class="columns") {
        //     div(class="column") {
        //         h1(class="title") {
        //             : raw!("Card Stats")
        //         }
        //     }
        // }

        div(id="card_stats_container") {
        }
    }
}

#[inline]
fn CardDetailSettings(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    card_id: CardID,
    card_settings: &CardSettings) {

    tmpl << html!{

        // TODO: remove
        // div(class="columns") {
        //     div(class="column") {
        //         h1(class="title") {
        //             : raw!("Card Settings")
        //         }
        //     }
        // }

        // div(class="columns") {
        //     div(class="column") {
        //         hr(class="is-marginless");
        //     }
        // }

        div(class="columns") {
            div(class="column") {
                div(class="tabs is-boxed") {
                    ul {
                        li(
                            class? = classnames!(
                                "is-active is-bold" => {
                                    matches!(*card_settings, CardSettings::Main)
                                })
                            ) {
                            a(href = view_route_to_link(
                                context.clone(),
                                AppRoute::Deck(deck_id, DeckRoute::CardProfile(card_id,
                                    CardRoute::Settings(CardSettings::Main)))
                                )) {
                                span {
                                    : "General"
                                }
                            }
                        }
                        li(
                            class? = classnames!(
                                "is-active is-bold" => {
                                    matches!(*card_settings, CardSettings::Move(_, _))
                                })
                            ) {
                            a(href = view_route_to_link(
                                context.clone(),
                                AppRoute::Deck(deck_id, DeckRoute::CardProfile(card_id,
                                    CardRoute::Settings(CardSettings::Move(
                                        MoveDecksPageQuery::default_with_card(context.clone(), card_id),
                                        Default::default()))))
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

        |tmpl| {
            match *card_settings {
                CardSettings::Main => CardSettingsMain(tmpl, context.clone(), deck_id, card_id),
                CardSettings::Move(ref page_query, ref search) => {
                    CardSettingsMove(tmpl, context.clone(), deck_id, card_id, page_query, search)
                }
            }
        }


    }
}

#[inline]
fn CardSettingsMain(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    card_id: CardID) {

    tmpl << html!{

        div(class="columns") {
            div(class="column") {

                h4(class="title is-4") {
                    : raw!("Delete card")
                }

                h2(class="subtitle") {
                    : raw!("This action is irreversible!")
                }
            }
        }

        div(id="card_settings_main_delete_container") {
            // TODO: stubs....
        }

    }
}

#[inline]
fn CardSettingsMove(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    card_id: CardID,
    deck_page_query: &MoveDecksPageQuery,
    search: &Search) {

    tmpl << html!{

        div(class="columns") {
            div(class="column") {

                h4(class="title is-4") {
                    : raw!("Move card to a new deck")
                }

            }
        }

        div(class="columns is-marginless") {
            div(class="column is-side-paddingless") {

                div(class="level") {
                    div(class="level-left") {
                        div(class="level-item") {
                            strong(class="is-bold") {
                                : raw!("This card's actual deck parent: ")
                            }
                        }

                        div(class="level-item") {
                            |tmpl| CardRealDeckPath(tmpl, context.clone(), card_id);
                        }
                    }

                }

            }
        }

        div(class="columns is-marginless") {
            div(class="column is-side-paddingless") {
                |tmpl| CardMovePathToDeck(tmpl, context.clone(), deck_id, card_id, &deck_page_query);
            }
        }

        |tmpl| CardMovePaginationComponent(tmpl, context.clone(), deck_id, card_id, &deck_page_query, &search);
        |tmpl| CardMoveDecksList(tmpl, context.clone(), deck_id, card_id, &deck_page_query, &search);
        |tmpl| CardMovePaginationComponent(tmpl, context.clone(), deck_id, card_id, &deck_page_query, &search);


    }
}

#[inline]
fn CardRealDeckPath(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    card_id: CardID) {

    let deck_id = match cards::get_card(context.clone(), card_id) {
        Err(_) => {
            // TODO: internal error logging
            panic!();
        },
        Ok(card) => {
            card.deck_id
        }
    };

    VanillaRealDeckPath(tmpl, context, deck_id);

}

#[inline]
fn CardMovePathToDeck(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    this_deck: DeckID,
    this_card: CardID,
    deck_page_query: &MoveDecksPageQuery) {

    match *deck_page_query {
        MoveDecksPageQuery::Root(_) => {
            tmpl << html!{
                span(class="title is-5 is-marginless", style="font-weight:normal;") {
                    : raw!("/");
                }
            };
        },
        MoveDecksPageQuery::SourceOfDecks(DecksPageQuery(deck_id, _, _)) => {

            let deck_path = match decks::get_path_of_deck(context.clone(), deck_id) {
                Ok(path) => path,
                Err(_) => {
                    // TODO: internal error logging
                    panic!();
                }
            };

            let num_of_items = deck_path.len();

            tmpl << html!{

                @ for (index, deck_id) in deck_path.iter().enumerate() {

                    span(class="title is-5 is-marginless", style="font-weight:normal;") {
                        |tmpl| {
                            if index == 0 {
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

                        match decks::get_deck(context.clone(), *deck_id) {
                            Err(_) => {
                                // TODO: internal error logging
                                panic!();
                            },
                            Ok(deck) => {

                                tmpl << html!{
                                    span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                        a(href = view_route_to_link(context.clone(),
                                            AppRoute::Deck(this_deck,
                                                DeckRoute::CardProfile(this_card,
                                                    CardRoute::Settings(CardSettings::Move(
                                                        MoveDecksPageQuery::SourceOfDecks(
                                                            DecksPageQuery::default_with_deck(*deck_id)),
                                                        Default::default())))))
                                        ) {
                                            |tmpl| MathJaxInline(tmpl, deck.name.clone(), false);
                                        }
                                    }
                                }

                            }
                        }

                    }

                }

            };

        }
    }
}

#[inline]
fn CardMovePaginationComponent(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    parent_deck: DeckID,
    card_id: CardID,
    deck_page_query: &MoveDecksPageQuery,
    search: &Search) {

    if !deck_page_query.should_show_pagination(context.clone()) {
        return;
    }

    let make_link = |deck_page_query: MoveDecksPageQuery| -> AppRoute {
        AppRoute::Deck(parent_deck, DeckRoute::CardProfile(card_id,
            CardRoute::Settings(CardSettings::Move(deck_page_query, search.clone()))))
    };

    let current_app_route = make_link(deck_page_query.clone());

    // TODO: remove
    // let current_app_route = AppRoute::Deck(parent_deck, DeckRoute::CardProfile(card_id,
    //     CardRoute::Settings(CardSettings::Move(deck_page_query.clone(), search.clone()))));

    let current_href = view_route_to_link(context.clone(), current_app_route);

    tmpl << html!{
        div(class="columns") {
            div(class="column") {
                nav(class="pagination") {

                    |tmpl| {

                        match deck_page_query.previous() {
                            None => {},
                            Some(page_query) => {

                                let app_route = make_link(page_query);

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

                                                let app_route = make_link(page_query);

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

                                        let app_route = make_link(page_query);

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
                                @ for page_query in deck_page_query.get_right_side(context.clone()) {
                                    |tmpl| {

                                        let current_page = page_query.current_page();

                                        let app_route = make_link(page_query);

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

                            if deck_page_query.has_trailing_right_side_delimeter(context.clone()) {
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

                            match deck_page_query.get_trailing_right_side(context.clone()) {
                                None => {},
                                Some(list) => {
                                    tmpl << html!{
                                        @ for page_query in list {
                                            |tmpl| {

                                                let current_page = page_query.current_page();

                                                let app_route = make_link(page_query);

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

                        match deck_page_query.next(context.clone()) {
                            None => {},
                            Some(page_query) => {

                                let app_route = make_link(page_query);

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
fn CardMoveDecksList(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    parent_deck: DeckID,
    card_id: CardID,
    deck_page_query: &MoveDecksPageQuery,
    search: &Search) {

    // get root deck
    let root_deck_id = match user::get_root_deck(context.clone()) {
        Ok(Some(root_deck_id)) => root_deck_id,
        Ok(None) => {
            // TODO: internal error logging
            panic!();
        },
        Err(_) => {
            // TODO: internal error logging
            panic!();
        }
    };

    match *deck_page_query {
        MoveDecksPageQuery::Root(root_deck_id) => {
            // case: card is inside root deck

            // TODO: complete
            tmpl << html!{
                |tmpl| MoveCardToDeckListItemComponent(tmpl,
                    context.clone(), parent_deck, card_id, root_deck_id, true);
            };

            return;

        },
        MoveDecksPageQuery::SourceOfDecks(ref page_query) => {

            let &DecksPageQuery(deck_id, _, _) = page_query;

            // case: card is not inside root deck

            let children = match decks::get_deck_children(context.clone(), deck_id, page_query, search) {
                Ok(children) => children,
                Err(_) => {
                    // TODO: internal error logging
                    panic!();
                }
            };

            let number_of_items = children.len();

            if number_of_items <= 0 {
                tmpl << html!{

                    div(class="columns is-marginless") {
                        div(class="column is-side-paddingless") {
                            a(href = view_route_to_link(context.clone(),
                                        AppRoute::Deck(parent_deck,
                                            DeckRoute::CardProfile(card_id,
                                                CardRoute::Settings(
                                                    CardSettings::Move(go_back_up_deck(context.clone(), deck_id),
                                                        Default::default()))))),
                                class=raw!("is-bold button is-primary is-fullwidth is-outlined")
                            ) {
                                // TODO: phrasing?
                                : raw!("Go up one deck")
                            }
                        }
                    }

                    div(class="columns") {
                        div(class="column") {
                            article(class="message") {
                                div(class="message-body") {
                                    : raw!("There are no decks to display.")
                                }
                            }
                        }
                    }
                };

                return;
            }

            tmpl << html!{

                div(class="columns is-marginless") {
                    div(class="column is-side-paddingless") {
                        a(href = view_route_to_link(context.clone(),
                                    AppRoute::Deck(parent_deck,
                                        DeckRoute::CardProfile(card_id,
                                            CardRoute::Settings(
                                                CardSettings::Move(go_back_up_deck(context.clone(), deck_id),
                                                    Default::default()))))),
                            class=raw!("is-bold button is-primary is-fullwidth is-outlined")
                        ) {
                            // TODO: phrasing?
                            : raw!("Go up one deck")
                        }
                    }
                }

                @ for (index, deck_id) in children.iter().enumerate() {
                    |tmpl| MoveCardToDeckListItemComponent(tmpl,
                        context.clone(), parent_deck, card_id, *deck_id, (index + 1) >= number_of_items);
                }
            };


        }
    }

}

#[inline]
fn MoveCardToDeckListItemComponent(tmpl: &mut TemplateBuffer, context: Rc<RefCell<Context>>,
    parent_deck: DeckID, card_id: CardID, deck_id: DeckID, is_bottom: bool) {

    let deck = match decks::get_deck(context.clone(), deck_id) {
        Ok(deck) => deck,
        Err(_) => {
            // TODO: internal error logging
            panic!();
        }
    };

    let is_cards_parent = match cards::get_card(context.clone(), card_id) {
        Err(_) => {
            // TODO: internal error logging
            panic!();
        },
        Ok(card) => {
            card.deck_id == deck_id
        }
    };

    tmpl << html!{
        div(class="columns is-marginless",
            style=labels!(
                "border-bottom:1px dotted #d3d6db;" => !is_bottom)) {
            div(class="column is-side-paddingless") {

                div(class="level is-marginless",
                    id = raw!(format!("{}-level", deck_id))) {

                    div(class="level-left") {

                        |tmpl| {
                            if is_cards_parent {

                                tmpl << html!{
                                    div(class="level-item") {
                                        span(class="tag is-dark is-bold") {
                                            : raw!("Current Deck")
                                        }
                                    }
                                }
                            }
                        }

                        |tmpl| {

                            if !is_cards_parent {
                                tmpl << html!{
                                    div(class="level-item", id = raw!(format!("{}-confirm", deck_id))) {
                                        // NOTE: confirm move button here; will be populated by react component
                                    }
                                }
                            }

                        }

                        div(class="level-item") {

                            h5(class="title is-5 is-marginless is-bold") {
                                a(href = view_route_to_link(context.clone(),
                                            AppRoute::Deck(parent_deck,
                                                DeckRoute::CardProfile(card_id,
                                                    CardRoute::Settings(CardSettings::Move(
                                                        MoveDecksPageQuery::SourceOfDecks(
                                                            DecksPageQuery::default_with_deck(deck_id)),
                                                        Default::default())))))

                                ) {
                                    |tmpl| MathJaxInline(tmpl, deck.name.clone(), is_cards_parent);
                                }
                            }

                            span(style="font-size:12px;") {
                                : format!("Deck #{}", deck.id);
                                : raw!(" ");
                                a(href = view_route_to_link(context.clone(),
                                                AppRoute::Deck(deck_id,
                                                    DeckRoute::Cards(CardsPageQuery::default_with_deck(deck_id),
                                                        Default::default())))) {
                                    : raw!("View Cards")
                                }
                            }

                        }

                    }

                    |tmpl| {
                        if !is_cards_parent {

                            tmpl << html!{
                                div(class="level-right") {
                                    div(class="level-item") {
                                        div(class="move_to_deck",
                                            data-deck-id = raw!(format!("{}", deck_id))) {

                                            // TODO: server rendered component
                                            a(class="button is-primary is-bold is-outlined") {
                                                : raw!("Move to this deck")
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }


                }

                |tmpl| {
                    if !is_cards_parent {

                        tmpl << html!{
                            div(id = raw!(format!("{}-error", deck_id))) {
                                // NOTE: rendered by react component
                            }
                        }
                    }
                }

            }
        }
    }

}

#[inline]
fn MathJaxInline(tmpl: &mut TemplateBuffer, content: String, should_bold: bool) {

    if should_bold {

        tmpl << html!{
            span(style="font-weight:bold;") {
                span(class="mathjax_inline_pre") {
                    : content
                }
            }
        };

        return;
    }

    tmpl << html!{
        span(style="font-weight:normal;") {
            span(class="mathjax_inline_pre") {
                : content
            }
        }
    };

}
