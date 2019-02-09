/* rust lib imports */

use std::rc::Rc;
use std::cell::RefCell;

/* 3rd-party imports */

use horrorshow::{TemplateBuffer, Template,Raw};

use serde_json;

/* local imports */

use route::{AppRoute, DeckRoute, CardRoute, DeckSettings, CardSettings};
use context::{Context};
use types::{DeckID, DecksPageQuery, CardID, CardsPageQuery, Search, Pagination, SortOrderable, MoveDecksPageQuery};
use api::{decks, cards, user};
use api::review::{self, CachedReviewProcedure, Reviewable, ActiveSelection};
use timestamp;
use separator::Separatable;

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

macro_rules! number_format_sep {
    ($num:expr) => {{
        ($num).separated_string()
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
pub fn view_route_to_link(
    _context: Rc<RefCell<Context>>,
    app_route: AppRoute) -> String {

    match app_route {
        AppRoute::Preferences => "/preferences".to_string(),
        AppRoute::Stashes => "/stashes".to_string(),
        AppRoute::Card(card_id, card_route) => {
            format!("/card/{card_id}/{card_route}",
                card_id = card_id,
                card_route = card_route_string(card_route))
        },
        AppRoute::Deck(deck_id, deck_route) => {
            match deck_route {
                DeckRoute::NewDeck => format!("/deck/{}/new/deck", deck_id),
                DeckRoute::NewCard(None) => format!("/deck/{}/new/card", deck_id),
                DeckRoute::NewCard(Some(card_id)) => {
                    format!("/deck/{deck_id}/new/card?clone_from={card_id}",
                        deck_id = deck_id,
                        card_id = card_id)
                },
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
                DeckRoute::Cards(page_query) => {

                    let query = page_query.generate_query_string();

                    format!("/deck/{deck_id}/cards?{query_string}", deck_id = deck_id, query_string = query)
                },
                DeckRoute::Decks(page_query, search) => {

                    let mut query = page_query.generate_query_string();

                    if let Some(search_query) = search.generate_query_string() {
                        query = query + &format!("&{}", search_query);
                    }

                    format!("/deck/{deck_id}/decks?{query_string}", deck_id = deck_id, query_string = query)

                }
            }
        }
    }
}

#[inline]
pub fn generate_delete_to(app_route: &AppRoute) -> String {
    match *app_route {
        AppRoute::Card(card_id, ref card_route) => {
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
        AppRoute::Deck(deck_id, ref deck_route) => {
            match *deck_route {
                DeckRoute::Settings(DeckSettings::Main) => {
                    format!("/api/deck/{deck_id}", deck_id = deck_id)
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
        AppRoute::Card(card_id, ref card_route) => {
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
        },
        AppRoute::Deck(deck_id, ref deck_route) => {
            match *deck_route {
                DeckRoute::NewDeck => {
                    format!("/api/deck/{deck_id}/new/deck", deck_id = deck_id)
                },
                DeckRoute::NewCard(_) => {
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
        AppRoute::Card(card_id, ref card_route) => {
            match *card_route {
                CardRoute::Settings(CardSettings::Move(_, _)) => {
                    format!("/api/card/{card_id}/move", card_id = card_id)
                },
                _ => {
                    panic!("invalid use of generate_move_to");
                }
            }
        },
        AppRoute::Deck(deck_id, ref deck_route) => {
            match *deck_route {
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
    // - use Raw macro
    // - if possible, write JSON manually

    // window.__PRE_RENDER_STATE__
    match *app_route {
        AppRoute::Card(card_id, ref card_route) => {
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
                        : Raw(
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
                                : Raw(
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
                                : Raw(
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
                        : Raw(
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
        AppRoute::Deck(deck_id, ref deck_route) => {
            match *deck_route {
                DeckRoute::NewDeck => {
                    tmpl << html! {
                        : Raw(
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
                DeckRoute::NewCard(Some(card_id)) => {

                    let (title, question, answer, description):
                        (String, String, String, String) =
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

                            (title, question, answer, description)
                        },
                        Err(_) => {
                            // TODO: internal error logging
                            panic!();
                        }
                    };

                    tmpl << html! {
                        : Raw(
                            format!(
                                "window.__PRE_RENDER_STATE__ = \
                                    {{\
                                        POST_TO: '{post_to}',\
                                        CARD_TITLE: {title},\
                                        CARD_DESCRIPTION: {description},\
                                        CARD_QUESTION: {question},\
                                        CARD_ANSWER: {answer}\
                                    }};\
                                ",
                                post_to = generate_post_to(app_route),
                                title = title,
                                question = question,
                                answer = answer,
                                description = description
                            )
                        )
                    }
                },
                DeckRoute::NewCard(None) => {
                    tmpl << html! {
                        : Raw(
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
                        : Raw(
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
                        : Raw(
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
                        : Raw(
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
                                : Raw(
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

                    let card_score = match review::get_card_score(
                        context.clone(),
                        card_id) {
                        Ok(card_score) => card_score,
                        Err(_) => {
                            // TODO: internal error logging
                            panic!();
                        }
                    };

                    tmpl << html! {
                        : Raw(
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
                                        CARD_META: {card_meta},\
                                        TIME_TILL_AVAILABLE_FOR_REVIEW: {time_till_available_for_review},\
                                        CARDS_TILL_AVAILABLE_FOR_REVIEW: {cards_till_available_for_review},\
                                        TIME_CONTROL: '{time_control}'\
                                    }};\
                                ",
                                post_to = generate_post_to(app_route),
                                profile_url = view_route_to_link(context.clone(),
                                    AppRoute::Card(card_id, Default::default())),
                                card_id = card_id,
                                card_title = card_title,
                                card_description = card_description,
                                card_question = card_question,
                                card_answer = card_answer,
                                card_is_active = card_is_active,
                                card_meta = card_meta,
                                time_till_available_for_review = card_score.review_after_normalized,
                                cards_till_available_for_review = card_score.cards_till_ready_for_review,
                                time_control = card_score.review_after_time_control
                            )
                        )
                    };

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

/* title generator */

fn generate_title(
    _context: Rc<RefCell<Context>>,
    app_route: &AppRoute) -> String {

    // NOTE: do not prefix with grokdb

    match *app_route {
        AppRoute::Card(_card_id, ref card_route) => {
            // TODO: put card # in title???
            match *card_route {
                CardRoute::Contents => format!("Card Contents"),
                CardRoute::Review => format!("Card Review"),
                CardRoute::Stats => format!("Card Statistics"),
                CardRoute::Settings(ref card_settings) => {
                    match *card_settings {
                        CardSettings::Main => format!("Card Settings"),
                        CardSettings::Move(ref _page_query, ref _search) => format!("Move Card"),
                    }
                }
            }
        },
        AppRoute::Preferences => format!("Preferences"),
        AppRoute::Stashes => format!("Stashes"),
        AppRoute::Deck(_deck_id, ref deck_route) => {
            match *deck_route {
                DeckRoute::NewCard(_) => format!("New Card"),
                DeckRoute::NewDeck => format!("New Deck"),
                DeckRoute::Description => format!("Deck Description"),
                DeckRoute::Decks(ref _decks_page_query, ref _search) => format!("Decks"),
                DeckRoute::Cards(ref _cards_page_query) => format!("Cards"),
                DeckRoute::Settings(ref deck_settings) => {
                    match *deck_settings {
                        DeckSettings::Main => format!("Deck Settings"),
                        DeckSettings::Move(ref _page_query, ref _search) => format!("Move Deck"),
                    }
                },
                DeckRoute::Stats => format!("Deck Statistics"),
                DeckRoute::Review(_) => format!("Deck Review")
            }
        }
    }
}

/* components */

#[inline]
pub fn AppComponent(tmpl: &mut TemplateBuffer, context: Rc<RefCell<Context>>, app_route: &AppRoute) {

    tmpl << html! {
        : Raw("<!DOCTYPE html>");
        html {
            head {
                title {
                    // TODO: fix
                    : generate_title(context.clone(), app_route)
                }

                // NOTE: reference: https://github.com/joshbuchea/HEAD#favicons
                link(rel="icon", href="/assets/favicon-16.png", sizes="16x16", type="image/png");

                // TODO: necessary?
                // style(type="text/css") {
                //     : Raw(include_str!("../assets/bulma.css"))
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

                            : Raw("\
                                body {\
                                    display: flex;\
                                    min-height: 100vh;\
                                    flex-direction: column;\
                                    background-color: #ffffff;\
                                }\
                            ");
                            : Raw("\
                                #grokdb {\
                                    flex: 1;\
                                }\
                            ");

                            // custom styles
                            // TODO: merge back into bulma css

                            : Raw("\
                                .is-side-paddingless {\
                                    padding-left: 0;\
                                    padding-right: 0;\
                                }\
                            ");

                            : Raw("\
                                .is-bold{\
                                    font-weight: bold;\
                                }\
                            ");

                            : Raw("\
                                span.mathjax_inline_pre {\
                                    font-weight: inherit;\
                                }\
                                span.mathjax_inline {\
                                    font-weight: inherit;\
                                }\
                            ");

                            : Raw("
                                .button.is-success,
                                .button.is-success:focus {
                                  background-color: #5c940d;
                                  color: #f4fce3;
                                  border-color: #5c940d;
                                }

                                .button.is-success:hover {
                                  background-color: #f4fce3;
                                  color: #5c940d;
                                  border-color: #5c940d;
                                }

                                .button.is-success.is-outlined,
                                .button.is-success.is-outlined:focus {
                                  background-color: #f4fce3;
                                  color: #5c940d;
                                  border-color: #5c940d;
                                }

                                .button.is-success.is-outlined:hover {
                                  background-color: #5c940d;
                                  color: #f4fce3;
                                  border-color: #5c940d;
                                }

                                .button.is-danger,
                                .button.is-danger:focus {
                                  background-color: #c92a2a;
                                  color: #fff5f5;
                                  border-color: #c92a2a;
                                }

                                .button.is-danger:hover {
                                  background-color: #fff5f5;
                                  color: #c92a2a;
                                  border-color: #c92a2a;
                                }

                                .button.is-danger.is-outlined,
                                .button.is-danger.is-outlined:focus {
                                  background-color: #fff5f5;
                                  color: #c92a2a;
                                  border-color: #c92a2a;
                                }

                                .button.is-danger.is-outlined:hover {
                                  background-color: #c92a2a;
                                  color: #fff5f5;
                                  border-color: #c92a2a;
                                }

                                .button.is-warning,
                                .button.is-warning:focus {
                                  background-color: #d9480f;
                                  color: #fff9db;
                                  border-color: #d9480f;
                                }

                                .button.is-warning:hover {
                                  background-color: #fff9db;
                                  color: #d9480f;
                                  border-color: #d9480f;
                                }

                                .button.is-warning.is-outlined,
                                .button.is-warning.is-outlined:focus {
                                  background-color: #fff9db;
                                  color: #d9480f;
                                  border-color: #d9480f;
                                }

                                .button.is-warning.is-outlined:hover {
                                  background-color: #d9480f;
                                  color: #fff9db;
                                  border-color: #d9480f;
                                }

                                .button.is-purple,
                                .button.is-purple:focus {
                                  background-color: #5f3dc4;
                                  color: #f3f0ff;
                                  border-color: #5f3dc4;
                                }

                                .button.is-purple:hover {
                                  background-color: #f3f0ff;
                                  color: #5f3dc4;
                                  border-color: #5f3dc4;
                                }

                                .button.is-purple.is-outlined,
                                .button.is-purple.is-outlined:focus {
                                  background-color: #f3f0ff;
                                  color: #5f3dc4;
                                  border-color: #5f3dc4;
                                }

                                .button.is-purple.is-outlined:hover {
                                  background-color: #5f3dc4;
                                  color: #f3f0ff;
                                  border-color: #5f3dc4;
                                }
                            ");

                            // : Raw("
                            //     .button.is-success {
                            //       background-color: #5c940d;
                            //       color: #f4fce3;
                            //       border-color: #5c940d;
                            //     }

                            //     .button.is-success:hover,
                            //     .button.is-success:focus {
                            //       background-color: #f4fce3;
                            //       color: #5c940d;
                            //       border-color: #5c940d;
                            //     }

                            //     .button.is-success.is-outlined {
                            //       background-color: #f4fce3;
                            //       color: #5c940d;
                            //       border-color: #5c940d;
                            //     }

                            //     .button.is-success.is-outlined:hover,
                            //     .button.is-success.is-outlined:focus {
                            //       background-color: #5c940d;
                            //       color: #f4fce3;
                            //       border-color: #5c940d;
                            //     }

                            //     .button.is-danger {
                            //       background-color: #c92a2a;
                            //       color: #fff5f5;
                            //       border-color: #c92a2a;
                            //     }

                            //     .button.is-danger:hover,
                            //     .button.is-danger:focus {
                            //       background-color: #fff5f5;
                            //       color: #c92a2a;
                            //       border-color: #c92a2a;
                            //     }

                            //     .button.is-danger.is-outlined {
                            //       background-color: #fff5f5;
                            //       color: #c92a2a;
                            //       border-color: #c92a2a;
                            //     }

                            //     .button.is-danger.is-outlined:hover,
                            //     .button.is-danger.is-outlined:focus {
                            //       background-color: #c92a2a;
                            //       color: #fff5f5;
                            //       border-color: #c92a2a;
                            //     }

                            //     .button.is-warning {
                            //       background-color: #d9480f;
                            //       color: #fff9db;
                            //       border-color: #d9480f;
                            //     }

                            //     .button.is-warning:hover,
                            //     .button.is-warning:focus {
                            //       background-color: #fff9db;
                            //       color: #d9480f;
                            //       border-color: #d9480f;
                            //     }

                            //     .button.is-warning.is-outlined {
                            //       background-color: #fff9db;
                            //       color: #d9480f;
                            //       border-color: #d9480f;
                            //     }

                            //     .button.is-warning.is-outlined:hover,
                            //     .button.is-warning.is-outlined:focus {
                            //       background-color: #d9480f;
                            //       color: #fff9db;
                            //       border-color: #d9480f;
                            //     }

                            //     .button.is-purple {
                            //       background-color: #5f3dc4;
                            //       color: #f3f0ff;
                            //       border-color: #5f3dc4;
                            //     }

                            //     .button.is-purple:hover,
                            //     .button.is-purple:focus {
                            //       background-color: #f3f0ff;
                            //       color: #5f3dc4;
                            //       border-color: #5f3dc4;
                            //     }

                            //     .button.is-purple.is-outlined {
                            //       background-color: #f3f0ff;
                            //       color: #5f3dc4;
                            //       border-color: #5f3dc4;
                            //     }

                            //     .button.is-purple.is-outlined:hover,
                            //     .button.is-purple.is-outlined:focus {
                            //       background-color: #5f3dc4;
                            //       color: #f3f0ff;
                            //       border-color: #5f3dc4;
                            //     }
                            // ");
                        }
                    }
                }

                // http://docs.mathjax.org/en/latest/configuration.html
                script(type="text/x-mathjax-config") {
                    : Raw(r"
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
                                        : Raw("grokdb")
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
                                        : Raw("Decks")
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
                                //         : Raw("Stashes")
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
                                        : Raw("Preferences")
                                    }
                                }

                            }
                        }
                    }

                    div(class="container", style="max-width:960px;") {
                        div(class="columns") {

                            |tmpl| {
                                match *app_route {
                                    AppRoute::Card(_card_id, ref _card_route) => {
                                        DeckDetail(tmpl, context.clone(), app_route)
                                    },
                                    AppRoute::Deck(_deck_id, ref _deck_route) => {
                                        DeckDetail(tmpl, context.clone(), app_route)
                                    },
                                    AppRoute::Preferences => {
                                        Preferences(tmpl, context.clone())
                                    },
                                    _ => {
                                        panic!("invalid route");
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

                            : Raw("grokdb v");
                            : env!("CARGO_PKG_VERSION");

                            : format!(" - Total Reviews: {}", {

                                let result = match user::get_review_count(context.clone()) {
                                    Err(_why) => {
                                        panic!();
                                    },
                                    Ok(result) => result
                                };

                                match result {
                                    None => 0,
                                    Some(review_count) => review_count
                                }
                            });

                            : format!(" - Total Cards: {}", {

                                let result = match user::get_root_deck(context.clone()) {
                                    Err(_why) => {
                                        panic!();
                                    },
                                    Ok(result) => result
                                };

                                let root_deck_id = match result {
                                    None => panic!(),
                                    Some(root_deck_id) => root_deck_id
                                };

                                match cards::total_num_of_cards_in_deck(context.clone(),
                                    root_deck_id,
                                    &Default::default()) {
                                    Err(_why) => {
                                        panic!();
                                    },
                                    Ok(result) => result
                                }
                            });

                            : format!(" - Total Decks: {}", {

                                let result = match user::get_root_deck(context.clone()) {
                                    Err(_why) => {
                                        panic!();
                                    },
                                    Ok(result) => result
                                };

                                let root_deck_id = match result {
                                    None => panic!(),
                                    Some(root_deck_id) => root_deck_id
                                };

                                match decks::get_num_descendents(context.clone(), root_deck_id) {
                                    Err(_why) => {
                                        panic!();
                                    },
                                    Ok(result) => result + 1
                                }
                            });

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

                        AppRoute::Deck(_, DeckRoute::NewCard(_)) =>  {
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
                                //     : Raw(format!("window.__PRE_RENDER_STATE__ = {};",
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
                        AppRoute::Deck(_deck_id, DeckRoute::Settings(DeckSettings::Main)) =>  {

                            tmpl << html! {

                                script(type="text/javascript") {
                                    |tmpl| {
                                        pre_render_state(tmpl, context.clone(), &app_route);
                                    }
                                }

                                script(type="text/javascript", src="/assets/deck_settings_main.js") {}
                            }

                        },
                        AppRoute::Deck(_deck_id, DeckRoute::Settings(DeckSettings::Move(_, _))) =>  {

                            tmpl << html! {

                                script(type="text/javascript") {
                                    |tmpl| {
                                        pre_render_state(tmpl, context.clone(), &app_route);
                                    }
                                }

                                script(type="text/javascript", src="/assets/deck_move_settings.js") {}
                            }

                        },
                        AppRoute::Deck(_, DeckRoute::Review(ref _card_for_review)) =>  {

                            tmpl << html! {

                                script(type="text/javascript") {
                                    |tmpl| {
                                        pre_render_state(tmpl, context.clone(), &app_route);
                                    }
                                }

                                script(type="text/javascript", src="/assets/deck_review.js") {}
                            }
                        },
                        AppRoute::Card(_card_id, ref card_route) => {
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
fn Preferences(
    tmpl: &mut TemplateBuffer,
    _context: Rc<RefCell<Context>>) {

    tmpl << html!{

        div(class="column") {

            div(class="columns") {
                div(class="column") {
                    h2(class="title is-2") {
                        : Raw("Preferences")
                    }
                }
            }

            div(class="columns") {
                div(class="column") {
                    h3(class="title is-3") {
                        : Raw("Database backup")
                    }
                }
            }

            div(class="columns") {
                div(class="column") {
                    div(id="backup_container") {
                        a(class="button is-primary is-bold is-outlined") {
                            : Raw("Backup database")
                        }
                    }
                }
            }

        }
    }
}

#[inline]
fn generateDeckPathLink(context: Rc<RefCell<Context>>, app_route: &AppRoute, deck_id: DeckID) -> String {

    let next_app_route = match *app_route {
        AppRoute::Card(card_id, ref card_route) => {

            let deck_route = DeckRoute::Cards(CardsPageQuery::default_with_deck(deck_id));

            AppRoute::Deck(deck_id, deck_route)
        },
        AppRoute::Deck(__route_deck_id, ref deck_route) => {

            let __deck_route = match *deck_route {

                DeckRoute::NewCard(_) |
                DeckRoute::NewDeck |
                DeckRoute::Description |
                DeckRoute::Stats |
                DeckRoute::Review(_) |
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

                DeckRoute::Cards(ref _page_query) => DeckRoute::Cards(CardsPageQuery::default_with_deck(deck_id))
            };

            AppRoute::Deck(deck_id, __deck_route)

        },
        _ => {
            panic!("invalid use of generateDeckPathLink");
        }
    };

    view_route_to_link(context, next_app_route)
}

#[inline]
fn DeckPath(tmpl: &mut TemplateBuffer, context: Rc<RefCell<Context>>, deck_id: DeckID, app_route: &AppRoute) {

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
                                : Raw("/ ");
                            }
                        } else {
                            tmpl << html!{
                                : Raw(" / ");
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
                                        a(href = generateDeckPathLink(context.clone(), app_route, *deck_id)
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
                                        a(href = generateDeckPathLink(context.clone(), app_route, *deck_id)
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

                match *app_route {
                    AppRoute::Deck(_route_deck_id, DeckRoute::Review(ref _card_for_review)) => {
                            tmpl << html!{

                                span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                    : Raw(" / ");
                                }

                                span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                    : Raw("Review")
                                }
                            }
                    },
                    AppRoute::Deck(_route_deck_id, DeckRoute::Description) => {
                            tmpl << html!{

                                span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                    : Raw(" / ");
                                }

                                span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                    : Raw("Deck Description")
                                }
                            }
                    },
                    AppRoute::Deck(_route_deck_id, DeckRoute::Decks(_, _)) => {
                            tmpl << html!{

                                span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                    : Raw(" / ");
                                }

                                span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                    : Raw("Decks")
                                }
                            }
                    },
                    AppRoute::Deck(_route_deck_id, DeckRoute::Cards(_)) => {
                            tmpl << html!{

                                span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                    : Raw(" / ");
                                }

                                span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                    : Raw("Cards")
                                }
                            }
                    },
                    AppRoute::Deck(_route_deck_id, DeckRoute::NewCard(_)) => {
                            tmpl << html!{

                                span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                    : Raw(" / ");
                                }

                                span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                    : Raw("Add New Card")
                                }
                            }
                    },
                    AppRoute::Deck(_route_deck_id, DeckRoute::NewDeck) => {
                            tmpl << html!{

                                span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                    : Raw(" / ");
                                }

                                span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                    : Raw("Add New Deck")
                                }
                            }
                    },
                    AppRoute::Deck(_route_deck_id, DeckRoute::Settings(ref deck_settings)) => {
                            tmpl << html!{

                                span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                    : Raw(" / ");
                                }

                                span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                    : Raw("Settings")
                                }

                                |tmpl| {

                                    match *deck_settings {
                                        DeckSettings::Move(_, _) => {

                                            tmpl << html!{
                                                span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                                    : Raw(" / ");
                                                }

                                                span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                                    : Raw("Move Deck")
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
                    AppRoute::Deck(_route_deck_id, DeckRoute::Stats) => {
                            tmpl << html!{

                                span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                    : Raw(" / ");
                                }

                                span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                    : Raw("Statistics")
                                }
                            }
                    },
                    AppRoute::Card(card_id, ref card_route) => {

                        tmpl << html!{

                            span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                : Raw(" / ");
                            }

                            span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                : Raw(format!("Card #{}", card_id))
                            }

                            |tmpl| {

                                match *card_route {
                                    CardRoute::Contents => {
                                        // no-op
                                    },
                                    CardRoute::Review => {

                                        tmpl << html!{

                                            span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                                : Raw(" / ");
                                            }

                                            span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                                : Raw("Review")
                                            }

                                        }
                                    },
                                    CardRoute::Stats => {

                                        tmpl << html!{

                                            span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                                : Raw(" / ");
                                            }

                                            span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                                : Raw("Statistics")
                                            }

                                        }
                                    },
                                    CardRoute::Settings(ref card_settings) => {

                                        tmpl << html!{

                                            span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                                : Raw(" / ");
                                            }

                                            span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                                : Raw("Settings")
                                            }

                                            |tmpl| {

                                                match *card_settings {
                                                    CardSettings::Main => {
                                                        // no-op
                                                    },
                                                    CardSettings::Move(_, _) => {

                                                        tmpl << html!{
                                                            span(class="title is-5 is-marginless", style="font-weight:normal;") {
                                                                : Raw(" / ");
                                                            }

                                                            span(class="title is-5 is-marginless", style="font-weight:bold;") {
                                                                : Raw("Move")
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

                    },
                    _ => {
                        panic!("invalid use of component");
                    }
                }
            }
        }
    }

}

#[inline]
fn DeckDetail(tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    app_route: &AppRoute) {

    let deck_id = match *app_route {
        AppRoute::Card(card_id, _) => {
            let card = match cards::get_card(context.clone(), card_id) {
                Ok(card) => card,
                Err(_) => {
                    // TODO: internal error logging
                    panic!();
                }
            };

            card.deck_id
        },
        AppRoute::Deck(deck_id, _) => deck_id,
        _ => {
            // TODO: refactor
            panic!("invalid");
        }
    };

    let number_of_decks = match decks::get_num_descendents(
        context.clone(),
        deck_id) {
        Ok(number_of_decks) => number_of_decks,
        Err(_) => {
            panic!();
        }
    };

    let number_of_cards = match cards::total_num_of_cards_in_deck(
        context.clone(),
        deck_id,
        &Default::default()) {
        Ok(number_of_cards) => number_of_cards,
        Err(_) => {
            panic!();
        }
    };

    tmpl << html!{

        div(class="column is-one-quarter") {

            |tmpl| CardDetailNav(tmpl, context.clone(), deck_id, app_route);

            nav(class="panel") {
                p(class="panel-heading is-bold") {
                    : Raw("Deck #");
                    : deck_id
                }
                div(class="panel-block") {

                    aside(class="menu has-text-right") {
                        ul(class="menu-list") {
                            li(style = Raw("padding-bottom:2px;")) {
                                a(href = view_route_to_link(context.clone(),
                                    AppRoute::Deck(deck_id,
                                        DeckRoute::Description)),
                                    class? = classnames!(
                                        "is-bold",
                                        "is-active" => {
                                            matches!(*app_route,
                                                AppRoute::Deck(_, DeckRoute::Description))
                                        })
                                ) {
                                    : "Description"
                                }
                            }
                            li(style = Raw("padding-top:2px;padding-bottom:2px;")) {
                                a(href = view_route_to_link(context.clone(),
                                    AppRoute::Deck(deck_id,
                                        DeckRoute::Decks(DecksPageQuery::default_with_deck(deck_id),
                                            Default::default()))),
                                    class? = classnames!(
                                        "is-bold",
                                        "is-active" => {
                                            matches!(*app_route, AppRoute::Deck(_, DeckRoute::NewDeck)) ||
                                            matches!(*app_route, AppRoute::Deck(_, DeckRoute::Decks(_, _)))
                                        })
                                ) {

                                    span(class="level") {
                                        span(class="level-left") {
                                            span(class="level-item") {
                                                span(class ?= classnames!("tag is-small",
                                                    "is-primary" => {
                                                        !(matches!(*app_route, AppRoute::Deck(_, DeckRoute::NewDeck)) ||
                                                        matches!(*app_route, AppRoute::Deck(_, DeckRoute::Decks(_, _))))
                                                    })
                                                ) {
                                                    : number_format_sep!(number_of_decks)
                                                }
                                            }
                                        }

                                        span(class="level-right") {
                                            span(class="level-item") {
                                                : Raw("Decks")
                                            }
                                        }
                                    }

                                }
                            }
                            li(style = Raw("padding-top:2px;padding-bottom:2px;")) {
                                a(href = view_route_to_link(context.clone(),
                                    AppRoute::Deck(deck_id,
                                        DeckRoute::Cards(CardsPageQuery::default_with_deck(deck_id)))),
                                    class? = classnames!(
                                        "is-bold",
                                        "is-active" => {
                                            matches!(*app_route, AppRoute::Deck(_, DeckRoute::NewCard(_))) ||
                                            matches!(*app_route, AppRoute::Deck(_, DeckRoute::Cards(_)))
                                        })
                                ) {

                                    span(class="level") {
                                        span(class="level-left") {
                                            span(class="level-item") {
                                                span(class ?= classnames!("tag is-small",
                                                    "is-primary" => {
                                                        !(matches!(*app_route, AppRoute::Deck(_, DeckRoute::NewCard(_))) ||
                                                        matches!(*app_route, AppRoute::Deck(_, DeckRoute::Cards(_))))
                                                    })
                                                ) {
                                                    : number_format_sep!(number_of_cards)
                                                }
                                            }
                                        }

                                        span(class="level-right") {
                                            span(class="level-item") {
                                                : Raw("Cards")
                                            }
                                        }
                                    }

                                }
                            }
                            li(style = Raw("padding-top:2px;padding-bottom:2px;")) {
                                a(href = view_route_to_link(context.clone(),
                                    AppRoute::Deck(deck_id,
                                        DeckRoute::Stats)),
                                    class? = classnames!(
                                        "is-bold",
                                        "is-active" => {
                                            matches!(*app_route, AppRoute::Deck(_, DeckRoute::Stats))
                                        })
                                ) {
                                    : "Stats"
                                }
                            }
                            li(style = Raw("padding-top:2px;")) {
                                a(href = view_route_to_link(context.clone(),
                                    AppRoute::Deck(deck_id,
                                        DeckRoute::Settings(DeckSettings::Main))),
                                    class? = classnames!(
                                        "is-bold",
                                        "is-active" => {
                                            matches!(*app_route, AppRoute::Deck(_, DeckRoute::Settings(_)))
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
                            DeckRoute::NewCard(None))),
                        class? = classnames!(
                            "is-bold button is-primary is-fullwidth is-medium",
                            "is-outlined" => {
                                !matches!(*app_route, AppRoute::Deck(_, DeckRoute::NewCard(_)))
                            })
                    ) {
                        : "New Card"
                    }

                    br;

                    a(href = view_route_to_link(context.clone(),
                        AppRoute::Deck(deck_id,
                            DeckRoute::NewDeck)),
                        class? = classnames!(
                            "is-bold button is-primary is-fullwidth is-medium",
                            "is-outlined" => {
                                !matches!(*app_route, AppRoute::Deck(_, DeckRoute::NewDeck))
                            })
                    ) {
                        : "New Deck"
                    }

                    br;

                    a(href = view_route_to_link(context.clone(),
                        AppRoute::Deck(deck_id,
                            DeckRoute::Review(None))),
                        class? = classnames!(
                            "is-bold button is-primary is-fullwidth is-medium",
                            "is-outlined" => {
                                !matches!(*app_route, AppRoute::Deck(_, DeckRoute::Review(_)))
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
                    |tmpl| DeckPath(tmpl, context.clone(), deck_id, app_route);
                }
            }

            |tmpl| {
                match *app_route {
                    AppRoute::Deck(deck_id, DeckRoute::Decks(ref decks_page_query, ref search)) => {
                        DecksChildren(
                            tmpl,
                            context.clone(),
                            deck_id,
                            decks_page_query,
                            search
                        )
                    },
                    AppRoute::Deck(deck_id, DeckRoute::NewDeck) => {
                        NewDeck(
                            tmpl,
                            context.clone(),
                            deck_id
                        )
                    },
                    AppRoute::Deck(deck_id, DeckRoute::NewCard(_)) => {
                        NewCard(
                            tmpl,
                            context.clone(),
                            deck_id
                        )
                    },
                    AppRoute::Deck(deck_id, DeckRoute::Description) => {
                        DeckDescription(
                            tmpl,
                            context.clone(),
                            deck_id
                        )
                    },
                    AppRoute::Deck(deck_id, DeckRoute::Cards(ref cards_page_query)) => {
                        DeckCards(
                            tmpl,
                            context.clone(),
                            deck_id,
                            cards_page_query
                        )
                    },
                    AppRoute::Deck(deck_id, DeckRoute::Settings(ref setting_mode)) => {
                        DeckSettings(
                            tmpl,
                            context.clone(),
                            setting_mode,
                            deck_id
                        )
                    },
                    AppRoute::Card(card_id, ref card_route) => {
                        CardDetail(
                            tmpl,
                            context.clone(),
                            card_id,
                            card_route
                        )
                    },
                    AppRoute::Deck(deck_id, DeckRoute::Review(ref card_for_review)) => {
                        DeckReview(
                            tmpl,
                            context.clone(),
                            deck_id,
                            card_for_review
                        )
                    },
                    AppRoute::Deck(deck_id, DeckRoute::Stats) => {
                        DeckStats(
                            tmpl,
                            context.clone(),
                            deck_id
                        )
                    },
                    _ => {
                        panic!("invalid");
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
    app_route: &AppRoute) {

    match *app_route {
        AppRoute::Card(card_id, ref card_route) => {
            tmpl << html!{

                nav(class="panel") {
                    p(class="panel-heading is-bold") {
                        : Raw("Card #");
                        : card_id
                    }
                    div(class="panel-block") {

                        aside(class="menu has-text-right") {
                            ul(class="menu-list") {
                                li(style = Raw("padding-bottom:2px;")) {
                                    a(href = view_route_to_link(context.clone(),
                                                AppRoute::Card(card_id, CardRoute::Contents)),
                                        class? = classnames!(
                                            "is-bold",
                                            "is-active" => {
                                                matches!(*app_route,
                                                    AppRoute::Card(_, CardRoute::Contents))
                                            })
                                    ) {
                                        : "Contents"
                                    }
                                }
                                li(style = Raw("padding-top:2px;padding-bottom:2px;")) {
                                    a(href = view_route_to_link(context.clone(),
                                        AppRoute::Card(card_id, CardRoute::Stats)),
                                        class? = classnames!(
                                            "is-bold",
                                            "is-active" => {
                                                matches!(*app_route,
                                                    AppRoute::Card(_, CardRoute::Stats))
                                            })
                                    ) {
                                        : "Stats"
                                    }
                                }
                                li(style = Raw("padding-top:2px;")) {
                                    a(href = view_route_to_link(context.clone(),
                                        AppRoute::Card(card_id, CardRoute::Settings(CardSettings::Main))),
                                        class? = classnames!(
                                            "is-bold",
                                            "is-active" => {
                                                matches!(*app_route,
                                                    AppRoute::Card(_, CardRoute::Settings(_)))
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
                                DeckRoute::NewCard(Some(card_id)))),
                            class = "is-bold button is-primary is-fullwidth is-medium is-outlined"
                        ) {
                            : "Clone Card"
                        }

                        br;

                        a(href = view_route_to_link(context.clone(),
                                    AppRoute::Card(card_id, CardRoute::Review)),
                            class? = classnames!(
                                "is-bold button is-primary is-fullwidth is-medium",
                                "is-outlined" => {
                                    !matches!(*app_route,
                                        AppRoute::Card(_, CardRoute::Review))
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
fn NewDeck(
    tmpl: &mut TemplateBuffer,
    _context: Rc<RefCell<Context>>,
    _deck_id: DeckID) {

    tmpl << html!{

        div(id="new_deck_container") {
            : Raw(include_str!("react_components/new_deck"))
        }
    }
}

#[inline]
fn NewCard(
    tmpl: &mut TemplateBuffer,
    _context: Rc<RefCell<Context>>,
    _deck_id: DeckID) {

    tmpl << html!{
        div(id="new_card_container") {
            : Raw(include_str!("react_components/new_card"))
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
        //             : Raw("Deck Description")
        //         }
        //     }
        // }

        div(id="deck_description_container") {
            : Raw(include_str!("react_components/deck_description"))
        }

        div(id="deck_description_container_stub", style="margin-top: 10px;") {

            // NOTE: this mocks react component

            div(class="columns") {
                div(class="column") {
                    a(class="button is-primary is-bold") {
                        : Raw("Preview")
                    }
                    : Raw(" ");
                    a(class="button is-bold") {
                        : Raw("Source")
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
                                        : Raw("No description set for this deck. \
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
    cards_page_query: &CardsPageQuery) {

    let make_link = |cards_page_query: CardsPageQuery| -> AppRoute {
        AppRoute::Deck(deck_id, DeckRoute::Cards(cards_page_query))
    };

    let search_query = match cards_page_query.search_query() {
        Search::NoQuery => None,
        Search::Query(query) => Some(query)
    };

    tmpl << html!{

        div(class="columns") {
            div(class="column") {
                div(class="level") {
                    div(class="level-left") {
                        div(class="level-item") {
                            a(class="button is-bold is-success",
                                href = view_route_to_link(context.clone(),
                                    AppRoute::Deck(deck_id, DeckRoute::NewCard(None)))) {
                                : Raw("New Card")
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
                                                DeckRoute::Cards(cards_page_query.descending())))
                                    ) {
                                        : cards_page_query.descending().sort_order_string()
                                    }
                                    option(
                                        value = view_route_to_link(context.clone(),
                                            AppRoute::Deck(deck_id,
                                                DeckRoute::Cards(cards_page_query.ascending())))
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
                                                DeckRoute::Cards(cards_page_query.updated_at())))
                                    ) {
                                        : cards_page_query.updated_at().sort_by_string()
                                    }

                                    option(
                                        value = view_route_to_link(context.clone(),
                                            AppRoute::Deck(deck_id,
                                                DeckRoute::Cards(cards_page_query.card_title())))
                                    ) {
                                        : cards_page_query.card_title().sort_by_string()
                                    }

                                    option(
                                        value = view_route_to_link(context.clone(),
                                            AppRoute::Deck(deck_id,
                                                DeckRoute::Cards(cards_page_query.created_at())))
                                    ) {
                                        : cards_page_query.created_at().sort_by_string()
                                    }

                                    option(
                                        value = view_route_to_link(context.clone(),
                                            AppRoute::Deck(deck_id,
                                                DeckRoute::Cards(cards_page_query.card_score())))
                                    ) {
                                        : cards_page_query.card_score().sort_by_string()
                                    }

                                    option(
                                        value = view_route_to_link(context.clone(),
                                            AppRoute::Deck(deck_id,
                                                DeckRoute::Cards(cards_page_query.last_reviewed_at())))
                                    ) {
                                        : cards_page_query.last_reviewed_at().sort_by_string()
                                    }

                                    option(
                                        value = view_route_to_link(context.clone(),
                                            AppRoute::Deck(deck_id,
                                                DeckRoute::Cards(cards_page_query.last_picked_for_review())))
                                    ) {
                                        : cards_page_query.last_picked_for_review().sort_by_string()
                                    }

                                    option(
                                        value = view_route_to_link(context.clone(),
                                            AppRoute::Deck(deck_id,
                                                DeckRoute::Cards(cards_page_query.times_reviewed())))
                                    ) {
                                        : cards_page_query.times_reviewed().sort_by_string()
                                    }

                                    option(
                                        value = view_route_to_link(context.clone(),
                                            AppRoute::Deck(deck_id,
                                                DeckRoute::Cards(cards_page_query.times_picked_for_review())))
                                    ) {
                                        : cards_page_query.times_picked_for_review().sort_by_string()
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        div(class="columns") {
            div(class="column") {
                div(class="level") {
                    div(class="level-item") {

                        form(
                            onsubmit = format!("\
                                window.location.href = \
                                encodeURIComponent(String({get_value}).trim()).length > 0 ?\
                                '{go_to}&search=' + encodeURIComponent(String({get_value}).trim()) :\
                                '{go_to}'\
                                ;\
                                return false;\
                            ",
                                go_to = view_route_to_link(context.clone(),
                                    AppRoute::Deck(deck_id,
                                        DeckRoute::Cards(CardsPageQuery::default_with_deck(deck_id)))),
                                get_value = "document.getElementById('search_card').value"
                            )
                        ) {

                            p(class="control has-addons") {

                                |tmpl| {

                                    if search_query.is_some() {

                                        tmpl << html!{
                                            a(class="button", href=view_route_to_link(context.clone(),
                                                AppRoute::Deck(deck_id,
                                                    DeckRoute::Cards(CardsPageQuery::default_with_deck(deck_id))))) {
                                                : Raw("Clear")
                                            }
                                        }
                                    }
                                }

                                input(
                                    id="search_card",
                                    class="input is-expanded",
                                    type="text",
                                    placeholder="Search",
                                    value ?= search_query
                                );

                                button(class="button is-primary is-outlined", type="submit") {
                                    : Raw("Search Cards")
                                }

                            }

                        }

                    }
                }
            }
        }

        |tmpl| GenericPaginationComponent(tmpl, context.clone(), cards_page_query, &make_link);
        |tmpl| CardsList(tmpl, context.clone(), deck_id, &cards_page_query);
        |tmpl| GenericPaginationComponent(tmpl, context.clone(), cards_page_query, &make_link);

    }
}

#[inline]
fn CardsList(tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>, deck_id: DeckID, cards_page_query: &CardsPageQuery) {

    let children = match cards::cards_in_deck(context.clone(), deck_id, cards_page_query) {
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
                            : Raw("There are no cards to display. You may create one within this deck.")
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

    let should_bold = true;

    tmpl << html!{
        div(class="columns is-marginless",
            style=labels!(
                "border-bottom:1px dotted #d3d6db;" => !is_bottom)) {
            div(class="column is-side-paddingless") {
                h5(class="title is-5 is-bold", style="margin-bottom: 10px;") {
                    a(href = view_route_to_link(context.clone(),
                                AppRoute::Card(card_id, CardRoute::Contents))
                    ) {
                        |tmpl| MathJaxInline(tmpl, card.title.clone(), should_bold);
                    }
                }

                |tmpl| CardListItemDetailComponent(tmpl, context.clone(), deck_id, card_id);

            }
        }
    }
}

#[inline]
fn CardListItemDetailComponent(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    card_id: CardID) {

    let card = match cards::get_card(context.clone(), card_id) {
        Err(_why) => {
            // TODO: logging
            panic!();
        },
        Ok(card) => card
    };

    let card_score = match review::get_card_score(context.clone(), card_id) {
        Ok(card_score) => card_score,
        Err(_) => {
            // TODO: logging
            panic!();
        }
    };

    tmpl << html!{

        div(class="level") {

            div(class="level-left") {
                div(class="level-item") {

                    div(class="level") {

                        div(class="level-left") {
                            div(class="level-item", style="font-size:12px; margin-right: 10px;") {
                                : Raw(format!("Card #{}", card.id));
                            }

                            div(class="level-item", style="font-size:12px; margin-right: 10px;") {
                                strong {
                                    : Raw("Score: ")
                                }
                                : Raw(format!("{} / 100", card_score.get_perf_score_percent_string()))
                            }

                            div(class="level-item", style="font-size:12px; margin-right: 10px;") {
                                strong {
                                    : Raw("Times reviewed: ");
                                }
                                : card_score.times_reviewed
                            }

                            div(class="level-item", style="font-size:12px; margin-right: 10px;") {

                                |tmpl| {

                                    if card.is_active {

                                        tmpl << html!{

                                            span(class="tag is-bold is-success is-small") {
                                                : Raw("Active")
                                            }

                                        };

                                    } else {

                                        tmpl << html!{

                                            span(class="tag is-bold is-danger is-small") {
                                                : Raw("Not Active")
                                            }

                                        };

                                    }
                                }

                            }
                        }

                    }

                }
            }

            div(class="level-right") {
                div(class="level-item") {

                    div(class="control is-grouped") {

                        p(class="control", style="font-size:12px;") {
                            a(href = view_route_to_link(context.clone(),
                                        AppRoute::Card(card_id, CardRoute::Contents)),
                                class = Raw("button is-small")
                            ) {
                                : Raw("Contents")
                            }
                        }

                        p(class="control", style="font-size:12px;") {
                            a(href = view_route_to_link(context.clone(),
                                            AppRoute::Card(card_id, CardRoute::Stats)),
                                class = Raw("button is-small")
                            ) {
                                : Raw("Stats")
                            }
                        }

                        p(class="control", style="font-size:12px;") {
                            a(href = view_route_to_link(context.clone(),
                                        AppRoute::Card(card_id, CardRoute::Settings(Default::default()))),
                                class = Raw("button is-small")
                            ) {
                                : Raw("Settings")
                            }
                        }

                        p(class="control", style="font-size:12px;") {
                            a(href = view_route_to_link(context.clone(),
                                        AppRoute::Card(card_id, CardRoute::Review)),
                                class = Raw("button is-small is-primary is-outlined")
                            ) {
                                : Raw("Review")
                            }
                        }

                    }

                }
            }
        }
    };
}

#[inline]
fn DeckReview(
    tmpl: &mut TemplateBuffer,
    _context: Rc<RefCell<Context>>,
    _deck_id: DeckID,
    _card_for_review: &Option<(CardID, Option<CachedReviewProcedure>)>) {


    // TODO: remove

    // if card_for_review.is_none() {

    //     tmpl << html!{

    //         div(class="columns") {
    //             div(class="column") {
    //                 h1(class="title") {
    //                     : Raw("No cards in this deck to review")
    //                 }
    //             }
    //         }

    //         // TODO: remove
    //         // div(class="columns") {
    //         //     div(class="column") {
    //         //         article(class="message") {
    //         //             div(class="message-body") {
    //         //                 : Raw("There are no cards to review.")
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
        //             : Raw("Reviewing Cards in this Deck")
        //         }
        //     }
        // }

        div(id="deck_review_container") {
            // TODO: fix
            // : Raw(include_str!("react_components/deck_review"))
        }

    };
}

#[inline]
fn DeckStats(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    deck_id: DeckID) {

    let deck = match decks::get_deck(context.clone(), deck_id) {
        Ok(deck) => deck,
        Err(_) => {
            panic!();
        }
    };

    let number_of_cards = match cards::total_num_of_cards_in_deck(
        context.clone(),
        deck_id,
        &Default::default()) {
        Ok(number_of_cards) => number_of_cards,
        Err(_) => {
            panic!();
        }
    };

    let number_of_decks = match decks::get_num_descendents(
        context.clone(),
        deck_id) {
        Ok(number_of_decks) => number_of_decks,
        Err(_) => {
            panic!();
        }
    };

    let number_of_child_decks = match decks::get_deck_children_total_count(
        context.clone(),
        deck_id) {
        Ok(number_of_child_decks) => number_of_child_decks,
        Err(_) => {
            panic!();
        }
    };

    let cards_active_for_review = match deck.deck_num_of_cards_for_review(
        context.clone(),
        &ActiveSelection::Active) {
        Ok(count) => count,
        Err(_) => {
            panic!();
        }
    };

    let cards_inactive_for_review = match deck.deck_num_of_cards_for_review(
        context.clone(),
        &ActiveSelection::Inactive) {
        Ok(count) => count,
        Err(_) => {
            panic!();
        }
    };

    let new_cards_for_review = match deck.num_of_new_cards_for_review(
        context.clone(),
        &ActiveSelection::Active) {
        Ok(count) => count,
        Err(_) => {
            panic!();
        }
    };

    let cards_ready_for_review = match deck.num_of_cards_ready_for_review(
        context.clone(),
        &ActiveSelection::Active) {
        Ok(count) => count,
        Err(_) => {
            panic!();
        }
    };

    tmpl << html!{

        div(class="columns") {
            div(class="column") {

                div(class="level") {
                    div(class="level-item has-text-centered") {
                        p(class="heading") {
                            : Raw("Number of cards")
                        }

                        p(class="title") {
                            : number_format_sep!(number_of_cards)
                        }
                    }

                    div(class="level-item has-text-centered") {
                        p(class="heading") {
                            : Raw("Number of decks")
                        }

                        p(class="title") {
                            : number_format_sep!(number_of_decks)
                        }
                    }

                    div(class="level-item has-text-centered") {
                        p(class="heading") {
                            : Raw("Direct deck children")
                        }

                        p(class="title") {
                            : number_format_sep!(number_of_child_decks)
                        }
                    }
                }

            }
        }

        div(class="columns") {
            div(class="column") {
                hr(class="is-marginless");
            }
        }

        div(class="columns") {
            div(class="column") {

                h1(class="title") {
                    : Raw("Deck created at")
                }

                h2(class="subtitle") {
                    : &deck.created_at
                }

            }
        }

        div(class="columns") {
            div(class="column") {

                h1(class="title") {
                    : Raw("Deck updated at")
                }

                h2(class="subtitle") {
                    : &deck.updated_at
                }

            }
        }

        |tmpl| {
            if deck.has_reviewed {

                tmpl << html!{

                    div(class="columns") {
                        div(class="column") {

                            h1(class="title") {
                                : Raw("Deck reviewed at")
                            }

                            h2(class="subtitle") {
                                : &deck.reviewed_at
                            }

                        }
                    }

                };

            }
        }

        div(class="columns") {
            div(class="column") {
                hr(class="is-marginless");
            }
        }

        div(class="columns") {
            div(class="column") {

                div(class="level") {
                    div(class="level-item has-text-centered") {
                        p(class="heading") {
                            : Raw("Cards active for review")
                        }

                        p(class="title") {
                            : number_format_sep!(cards_active_for_review)
                        }
                    }

                    div(class="level-item has-text-centered") {
                        p(class="heading") {
                            : Raw("Cards not active for review")
                        }

                        p(class="title") {
                            : number_format_sep!(cards_inactive_for_review)
                        }
                    }

                }

            }
        }

        div(class="columns") {
            div(class="column") {

                div(class="level") {
                    div(class="level-item has-text-centered") {
                        p(class="heading") {
                            : Raw("New cards for review")
                        }

                        p(class="title") {
                            : number_format_sep!(new_cards_for_review)
                        }
                    }

                    div(class="level-item has-text-centered") {
                        p(class="heading") {
                            : Raw("Cards ready for review")
                        }

                        p(class="title") {
                            : number_format_sep!(cards_ready_for_review)
                        }
                    }

                }

            }
        }

    };
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
        //             : Raw("Deck Settings")
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
                                Err(_why) => {
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

    tmpl << html!{

        div(class="columns") {
            div(class="column") {
                h4(class="title is-4") {
                    : Raw("Deck Name")
                }
            }
        }

        div(id="deck_settings_main_name_container", style="margin-bottom:10px;") {
            // : Raw(include_str!("react_components/deck_description"))
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
            let (count_of_cards, card_noun) = match cards::total_num_of_cards_in_deck(
                context.clone(),
                deck_id,
                &Search::NoQuery) {

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
                                : Raw("Delete Deck")
                            }

                            h2(class="subtitle") {
                                : Raw("This action is irreversible!")
                            }

                            p {
                                : Raw(format!(
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
                        // : Raw(include_str!("react_components/deck_description"))
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

    let make_link = |deck_page_query: MoveDecksPageQuery| -> AppRoute {
        AppRoute::Deck(deck_id, DeckRoute::Settings(
            DeckSettings::Move(deck_page_query, search.clone())))
    };

    tmpl << html!{

        div(class="columns") {
            div(class="column") {

                h4(class="title is-4") {
                    : Raw("Move deck to a new deck")
                }

            }
        }

        div(class="columns is-marginless") {
            div(class="column is-side-paddingless") {

                div(class="level") {
                    div(class="level-left") {
                        div(class="level-item") {
                            strong(class="is-bold") {
                                : Raw("This deck's actual deck parent: ")
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

        |tmpl| GenericPaginationComponent(tmpl, context.clone(), deck_page_query, &make_link);
        |tmpl| DeckMoveDecksList(tmpl, context.clone(), deck_id, &deck_page_query, &search);
        |tmpl| GenericPaginationComponent(tmpl, context.clone(), deck_page_query, &make_link);

    }
}

#[inline]
fn DeckMoveDecksList(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    child_deck: DeckID,
    deck_page_query: &MoveDecksPageQuery,
    search: &Search) {

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
                                class=Raw("is-bold button is-primary is-fullwidth is-outlined")
                            ) {
                                // TODO: phrasing?
                                : Raw("Go up one deck")
                            }
                        }
                    }

                    div(class="columns") {
                        div(class="column") {
                            article(class="message") {
                                div(class="message-body") {
                                    : Raw("There are no decks to display.")
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
                            class=Raw("is-bold button is-primary is-fullwidth is-outlined")
                        ) {
                            // TODO: phrasing?
                            : Raw("Go up one deck")
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
                    id = Raw(format!("{}-level", deck_id))) {

                    div(class="level-left") {

                        |tmpl| {
                            if is_decks_parent {

                                tmpl << html!{
                                    div(class="level-item") {
                                        span(class="tag is-dark is-bold") {
                                            : Raw("Current Deck")
                                        }
                                    }
                                }
                            }
                        }

                        |tmpl| {

                            if !is_decks_parent && !is_deck_descendent {
                                tmpl << html!{
                                    div(class="level-item", id = Raw(format!("{}-confirm", deck_id))) {
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

                            h5(class="title is-5 is-bold", style="margin-bottom: 10px;") {
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

                            |tmpl| DeckListItemDetailComponent(tmpl, context.clone(), deck.id);

                        }

                    }

                    |tmpl| {
                        if !is_decks_parent && !is_deck_descendent {

                            tmpl << html!{
                                div(class="level-right") {
                                    div(class="level-item") {
                                        div(class="move_to_deck",
                                            data-deck-id = Raw(format!("{}", deck_id))) {

                                            // TODO: server rendered component
                                            a(class="button is-primary is-bold is-outlined") {
                                                : Raw("Move to this deck")
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
                            div(id = Raw(format!("{}-error", deck_id))) {
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
                    : Raw("/");
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

            tmpl << html!{

                @ for (index, deck_id) in deck_path.iter().enumerate() {

                    span(class="title is-5 is-marginless", style="font-weight:normal;") {
                        |tmpl| {
                            if index == 0 {
                                tmpl << html!{
                                    : Raw("/ ");
                                }
                            } else {
                                tmpl << html!{
                                    : Raw(" / ");
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

    tmpl << html!{

        @ for (index, deck_id) in deck_path.iter().enumerate() {

            span(class="title is-5 is-marginless", style="font-weight:normal;") {
                |tmpl| {
                    if index == 0 {
                        tmpl << html!{
                            : Raw("/ ");
                        }
                    } else {
                        tmpl << html!{
                            : Raw(" / ");
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

    let make_link = |deck_page_query: DecksPageQuery| -> AppRoute {
        AppRoute::Deck(deck_id, DeckRoute::Decks(deck_page_query.clone(), search.clone()))
    };

    tmpl << html!{

        div(class="columns") {
            div(class="column") {
                div(class="level") {
                    div(class="level-left") {
                        div(class="level-item") {
                            a(class="button is-bold is-success",
                                href = view_route_to_link(context.clone(),
                                    AppRoute::Deck(deck_id, DeckRoute::NewDeck))) {
                                : Raw("New Deck")
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

        |tmpl| GenericPaginationComponent(tmpl, context.clone(), deck_page_query, &make_link);
        |tmpl| DecksChildrenList(tmpl, context.clone(), deck_id, &deck_page_query, &search);
        |tmpl| GenericPaginationComponent(tmpl, context.clone(), deck_page_query, &make_link);
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
                            : Raw("There are no decks to display. You may create one within this deck.")
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

    let should_bold = true;

    tmpl << html!{
        div(class="columns is-marginless",
            style=labels!(
                "border-bottom:1px dotted #d3d6db;" => !is_bottom)) {
            div(class="column is-side-paddingless") {
                h5(class="title is-5 is-bold", style="margin-bottom: 10px;") {
                    a(href = view_route_to_link(context.clone(),
                                    AppRoute::Deck(deck_id,
                                        DeckRoute::Decks(DecksPageQuery::default_with_deck(deck_id),
                                            Default::default())))
                    ) {
                        |tmpl| MathJaxInline(tmpl, deck.name.clone(), should_bold);
                    }
                }

                |tmpl| DeckListItemDetailComponent(tmpl, context.clone(), deck.id);
            }
        }
    }
}

#[inline]
fn CardDetail(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    card_id: CardID,
    card_route: &CardRoute) {

    match *card_route {
        CardRoute::Contents => CardDetailContents(tmpl, context, card_id),
        CardRoute::Review => CardDetailReview(tmpl, context, card_id),
        CardRoute::Stats => CardDetailStats(tmpl, context, card_id),
        CardRoute::Settings(ref card_settings) => CardDetailSettings(tmpl, context, card_id, card_settings),
    }

}

#[inline]
fn CardDetailContents(
    tmpl: &mut TemplateBuffer,
    _context: Rc<RefCell<Context>>,
    _card_id: CardID) {

    tmpl << html!{
        div(id="card_profile_container") {
        }
    }
}

#[inline]
fn CardDetailReview(
    tmpl: &mut TemplateBuffer,
    _context: Rc<RefCell<Context>>,
    _card_id: CardID) {

    tmpl << html!{

        // TODO: remove
        // div(class="columns") {
        //     div(class="column") {
        //         h1(class="title") {
        //             : Raw("Reviewing Card")
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
    card_id: CardID) {

    let card_score = match review::get_card_score(context.clone(), card_id) {
        Ok(card_score) => card_score,
        Err(_) => {
            // TODO: logging
            panic!();
        }
    };

    tmpl << html!{

        div(class="columns") {
            div(class="column") {

                div(class="level") {
                    div(class="level-left") {
                        div(class="level-item has-text-centered",
                            style="padding-right: 10px;") {

                            p(class="heading") {
                                : Raw("Performance score")
                            }

                            p(class="title") {
                                : Raw(format!("{} / 100", card_score.get_perf_score_percent_string()))
                            }
                        }
                    }

                    div(class="level-item") {

                        progress(
                            class ?= classnames!("progress",
                                "is-danger" => card_score.get_perf_score() < 0.5,
                                "is-warning" =>  0.5 <= card_score.get_perf_score() && card_score.get_perf_score() < 0.75,
                                "is-success" => 0.75 <= card_score.get_perf_score()
                            ),
                            value = card_score.get_perf_score_string(),
                            max = card_score.get_max_perf_score_string()
                        ) {
                            : Raw(card_score.get_perf_score_percent_string())
                        }

                    }
                }

            }
        }

        div(class="columns") {

            div(class="column") {

                div(class="level") {

                    div(class="level-item has-text-centered") {

                        p(class="heading") {
                            : Raw("Times reviewed")
                        }

                        p(class="title") {
                            : number_format_sep!(card_score.times_reviewed)
                        }
                    }

                    div(class="level-item has-text-centered") {

                        p(class="heading") {
                            : Raw("Times picked for review")
                        }

                        p(class="title") {
                            : number_format_sep!(card_score.times_seen)
                        }
                    }

                }
            }
        }

        div(class="columns") {

            div(class="column") {

                div(class="level") {

                    |tmpl| {

                        if card_score.was_reviewed(context.clone()) {
                            tmpl << html!{

                                div(class="level-item has-text-centered") {

                                    p(class="heading") {
                                        : Raw("Last reviewed")
                                    }

                                    p(class="title") {
                                        : timestamp::relative_time_from_timestamp(card_score.reviewed_at_actual)
                                    }
                                }

                            }
                        }
                    }

                    |tmpl| {

                        if card_score.was_picked_for_review(context.clone()) {

                            tmpl << html!{

                                div(class="level-item has-text-centered") {

                                    p(class="heading") {
                                        : Raw("Last picked for review")
                                    }

                                    p(class="title") {
                                        : timestamp::relative_time_from_timestamp(card_score.seen_at_actual)
                                    }
                                }

                            }
                        }
                    }

                }
            }
        }

        div(class="columns") {
            div(class="column") {
                hr(class="is-marginless");
            }
        }

        div(class="columns") {
            div(class="column") {

                h1(class="title") {
                    : Raw("Ready for review at")
                }

                h2(class="subtitle") {
                    : &card_score.ready_for_review_at
                }

            }
        }

    }
}

#[inline]
fn CardDetailSettings(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    card_id: CardID,
    card_settings: &CardSettings) {

    tmpl << html!{

        // TODO: remove
        // div(class="columns") {
        //     div(class="column") {
        //         h1(class="title") {
        //             : Raw("Card Settings")
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
                                AppRoute::Card(card_id,
                                    CardRoute::Settings(CardSettings::Main))
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
                                AppRoute::Card(card_id,
                                    CardRoute::Settings(CardSettings::Move(
                                        MoveDecksPageQuery::default_with_card(context.clone(), card_id),
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

        |tmpl| {
            match *card_settings {
                CardSettings::Main => CardSettingsMain(tmpl, context.clone(), card_id),
                CardSettings::Move(ref page_query, ref search) => {
                    CardSettingsMove(tmpl, context.clone(), card_id, page_query, search)
                }
            }
        }


    }
}

#[inline]
fn CardSettingsMain(
    tmpl: &mut TemplateBuffer,
    _context: Rc<RefCell<Context>>,
    _card_id: CardID) {

    tmpl << html!{

        div(class="columns") {
            div(class="column") {

                h4(class="title is-4") {
                    : Raw("Delete card")
                }

                h2(class="subtitle") {
                    : Raw("This action is irreversible!")
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
    card_id: CardID,
    deck_page_query: &MoveDecksPageQuery,
    search: &Search) {

    let make_link = |deck_page_query: MoveDecksPageQuery| -> AppRoute {
        AppRoute::Card(card_id,
            CardRoute::Settings(CardSettings::Move(deck_page_query, search.clone())))
    };

    tmpl << html!{

        div(class="columns") {
            div(class="column") {

                h4(class="title is-4") {
                    : Raw("Move card to a new deck")
                }

            }
        }

        div(class="columns is-marginless") {
            div(class="column is-side-paddingless") {

                div(class="level") {
                    div(class="level-left") {
                        div(class="level-item") {
                            strong(class="is-bold") {
                                : Raw("This card's actual deck parent: ")
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
                |tmpl| CardMovePathToDeck(tmpl, context.clone(), card_id, &deck_page_query);
            }
        }

        |tmpl| GenericPaginationComponent(tmpl, context.clone(), deck_page_query, &make_link);
        |tmpl| CardMoveDecksList(tmpl, context.clone(), card_id, &deck_page_query, &search);
        |tmpl| GenericPaginationComponent(tmpl, context.clone(), deck_page_query, &make_link);

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
    this_card: CardID,
    deck_page_query: &MoveDecksPageQuery) {

    match *deck_page_query {
        MoveDecksPageQuery::Root(_) => {
            tmpl << html!{
                span(class="title is-5 is-marginless", style="font-weight:normal;") {
                    : Raw("/");
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

            tmpl << html!{

                @ for (index, deck_id) in deck_path.iter().enumerate() {

                    span(class="title is-5 is-marginless", style="font-weight:normal;") {
                        |tmpl| {
                            if index == 0 {
                                tmpl << html!{
                                    : Raw("/ ");
                                }
                            } else {
                                tmpl << html!{
                                    : Raw(" / ");
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
                                            AppRoute::Card(this_card,
                                                CardRoute::Settings(CardSettings::Move(
                                                    MoveDecksPageQuery::SourceOfDecks(
                                                        DecksPageQuery::default_with_deck(*deck_id)),
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
fn CardMoveDecksList(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    card_id: CardID,
    deck_page_query: &MoveDecksPageQuery,
    search: &Search) {

    match *deck_page_query {
        MoveDecksPageQuery::Root(root_deck_id) => {
            // case: card is inside root deck

            // TODO: complete
            tmpl << html!{
                |tmpl| MoveCardToDeckListItemComponent(tmpl,
                    context.clone(), card_id, root_deck_id, true);
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
                                            AppRoute::Card(card_id,
                                                CardRoute::Settings(
                                                    CardSettings::Move(go_back_up_deck(context.clone(), deck_id),
                                                        Default::default())))),
                                class=Raw("is-bold button is-primary is-fullwidth is-outlined")
                            ) {
                                // TODO: phrasing?
                                : Raw("Go up one deck")
                            }
                        }
                    }

                    div(class="columns") {
                        div(class="column") {
                            article(class="message") {
                                div(class="message-body") {
                                    : Raw("There are no decks to display.")
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
                                    AppRoute::Card(card_id,
                                        CardRoute::Settings(
                                            CardSettings::Move(go_back_up_deck(context.clone(), deck_id),
                                                Default::default())))),
                            class=Raw("is-bold button is-primary is-fullwidth is-outlined")
                        ) {
                            // TODO: phrasing?
                            : Raw("Go up one deck")
                        }
                    }
                }

                @ for (index, deck_id) in children.iter().enumerate() {
                    |tmpl| MoveCardToDeckListItemComponent(tmpl,
                        context.clone(), card_id, *deck_id, (index + 1) >= number_of_items);
                }
            };


        }
    }

}

#[inline]
fn MoveCardToDeckListItemComponent(tmpl: &mut TemplateBuffer, context: Rc<RefCell<Context>>,
    card_id: CardID, deck_id: DeckID, is_bottom: bool) {

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
                    id = Raw(format!("{}-level", deck_id))) {

                    div(class="level-left") {

                        |tmpl| {
                            if is_cards_parent {

                                tmpl << html!{
                                    div(class="level-item") {
                                        span(class="tag is-dark is-bold") {
                                            : Raw("Current Deck")
                                        }
                                    }
                                }
                            }
                        }

                        |tmpl| {

                            if !is_cards_parent {
                                tmpl << html!{
                                    div(class="level-item", id = Raw(format!("{}-confirm", deck_id))) {
                                        // NOTE: confirm move button here; will be populated by react component
                                    }
                                }
                            }

                        }

                        div(class="level-item") {

                            h5(class="title is-5 is-bold", style="margin-bottom: 10px;") {
                                a(href = view_route_to_link(context.clone(),
                                            AppRoute::Card(card_id,
                                                CardRoute::Settings(CardSettings::Move(
                                                    MoveDecksPageQuery::SourceOfDecks(
                                                        DecksPageQuery::default_with_deck(deck_id)),
                                                    Default::default()))))

                                ) {
                                    |tmpl| MathJaxInline(tmpl, deck.name.clone(), is_cards_parent);
                                }
                            }

                            |tmpl| DeckListItemDetailComponent(tmpl, context.clone(), deck.id);

                        }

                    }

                    |tmpl| {
                        if !is_cards_parent {

                            tmpl << html!{
                                div(class="level-right") {
                                    div(class="level-item") {
                                        div(class="move_to_deck",
                                            data-deck-id = Raw(format!("{}", deck_id))) {

                                            // TODO: server rendered component
                                            a(class="button is-primary is-bold is-outlined") {
                                                : Raw("Move to this deck")
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
                            div(id = Raw(format!("{}-error", deck_id))) {
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

#[inline]
fn DeckListItemDetailComponent(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    deck_id: DeckID) {

    let deck = match decks::get_deck(context.clone(), deck_id) {
        Err(_why) => {
            // TODO: logging
            panic!();
        },
        Ok(deck) => deck
    };

    tmpl << html!{

        div(class="level") {

            div(class="level-left") {
                div(class="level-item") {
                    span(style="font-size:12px;") {
                        : Raw(format!("Deck #{}", deck.id));
                    }
                }
            }

            div(class="level-right") {
                div(class="level-item") {

                    div(class="control is-grouped") {

                        p(class="control", style="font-size:12px;") {
                            a(href = view_route_to_link(context.clone(),
                                            AppRoute::Deck(deck_id, DeckRoute::Description)),
                                class = Raw("button is-small")
                            ) {
                                : Raw("Description")
                            }
                        }

                        p(class="control", style="font-size:12px;") {
                            a(href = view_route_to_link(context.clone(),
                                            AppRoute::Deck(deck_id,
                                                DeckRoute::Cards(CardsPageQuery::default_with_deck(deck_id)))),
                                class = Raw("button is-small")
                            ) {
                                : Raw("View Cards")
                            }
                        }

                        p(class="control", style="font-size:12px;") {
                            a(href = view_route_to_link(context.clone(),
                                            AppRoute::Deck(deck_id, DeckRoute::NewCard(None))),
                                class = Raw("button is-small")
                            ) {
                                : Raw("New Card")
                            }
                        }

                        p(class="control", style="font-size:12px;") {
                            a(href = view_route_to_link(context.clone(),
                                            AppRoute::Deck(deck_id,
                                                DeckRoute::Decks(DecksPageQuery::default_with_deck(deck_id),
                                                    Default::default()))),
                                class = Raw("button is-small")
                            ) {
                                : Raw("View Decks")
                            }
                        }

                        p(class="control", style="font-size:12px;") {
                            a(href = view_route_to_link(context.clone(),
                                            AppRoute::Deck(deck_id, DeckRoute::NewDeck)),
                                class = Raw("button is-small")
                            ) {
                                : Raw("New Deck")
                            }
                        }

                        p(class="control", style="font-size:12px;") {
                            a(href = view_route_to_link(context.clone(),
                                            AppRoute::Deck(deck_id, DeckRoute::Stats)),
                                class = Raw("button is-small")
                            ) {
                                : Raw("Stats")
                            }
                        }

                        p(class="control", style="font-size:12px;") {
                            a(href = view_route_to_link(context.clone(),
                                            AppRoute::Deck(deck_id, DeckRoute::Settings(Default::default()))),
                                class = Raw("button is-small")
                            ) {
                                : Raw("Settings")
                            }
                        }

                        p(class="control", style="font-size:12px;") {
                            a(href = view_route_to_link(context.clone(),
                                            AppRoute::Deck(deck_id, DeckRoute::Review(None))),
                                class = Raw("button is-small is-primary is-outlined")
                            ) {
                                : Raw("Review")
                            }
                        }
                    }

                }
            }
        }
    };
}

#[inline]
fn GenericPaginationComponent<PageQuery: Pagination + Clone, LinkMaker>(
    tmpl: &mut TemplateBuffer,
    context: Rc<RefCell<Context>>,
    deck_page_query: &PageQuery,
    make_link: &LinkMaker)
    where LinkMaker: Fn(PageQuery) -> AppRoute {

    if !deck_page_query.should_show_pagination(context.clone()) {
        return;
    }

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
                                        : Raw("Previous")
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
                                        : Raw("Next")
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
