/* 3rd-party imports */

use templates::{RenderOnce, TemplateBuffer, Template, FnRenderer};

/* local imports */

use contexts::{Context};
use route::helpers::{view_route_to_link, view_route_to_pre_render_state};
use route::constants::{AppRoute, DeckRoute, CardRoute, DeckID, CardID, StashID};
use types::{DecksPageQuery, Search, SortOrder, DecksPageSort};
use decks::{DecksPageRequest, Deck};

////////////////////////////////////////////////////////////////////////////////

pub fn AppComponent<'a, 'b>(tmpl: &mut TemplateBuffer, mut context: &mut Context<'a, 'b>, title: String) {

    tmpl << html! {
        : raw!("<!DOCTYPE html>");
        html {
            head {
                title { : &title }
                link (
                    rel="stylesheet",
                    href="/assets/spectre.min.css"
                );

                // custom stylesheet for specific views
                |tmpl| {

                    // TODO: css minify using build.rs

                    tmpl << html! {
                        style {

                            // custom styles for view
                            |tmpl| {
                                match context.view_route {
                                    AppRoute::Deck(_, DeckRoute::NewCard) |
                                    AppRoute::Deck(_, DeckRoute::NewDeck) =>  {
                                        tmpl << html! {

                                            : raw!("\
                                                .btn-success {\
                                                    border-color: #30ae40;\
                                                    color: #32b643;\
                                                }\
                                                a.btn-success:hover {\
                                                    background: #32b643;\
                                                    border-color: #30ae40;\
                                                    color: #fff;\
                                                }\
                                            ");

                                        };

                                    },
                                    _ => {}
                                };
                            }
                            : raw!("\
                                body {\
                                    display: flex;\
                                    min-height: 100vh;\
                                    flex-direction: column;\
                                }\
                                #grokdb {\
                                    flex: 1;\
                                }\
                                ");
                            : raw!("\
                                ul.pagination li {\
                                    margin-top: 0;\
                                }\
                                ");
                            : raw!("\
                                .grokdb-menu {\
                                    box-shadow: none;\
                                    border: .1rem solid #c5c5c5;\
                                }\
                                ");
                            : raw!("\
                                a:hover,\
                                a:active,\
                                .menu .menu-item a:hover,\
                                .menu .menu-item a:active\
                                {\
                                    text-decoration: underline;\
                                }\
                                ");
                            : raw!("\
                                hr {\
                                    height: 1px;\
                                    background-color: #c5c5c5;\
                                    border:none;\
                                }\
                                ");
                        }
                    };


                }
            }
            body {
                section(class="container grid-960", id="grokdb") {
                    header(class="navbar") {
                        section(class="navbar-section") {
                            a(href = view_route_to_link(AppRoute::Home, &context), class="navbar-brand") {
                                : "grokdb"
                            }
                        }
                        section(class="navbar-section") {
                            a(
                                href = view_route_to_link(AppRoute::Home, &context),

                                // TODO: fix
                                style? = stylenames!("font-weight:bold;" => {
                                    matches!(context.view_route, AppRoute::Deck(_, _)) ||
                                    matches!(context.view_route, AppRoute::Home) ||
                                    matches!(context.view_route, AppRoute::Card(_, _)) ||
                                    matches!(context.view_route, AppRoute::CardInDeck(_, _, _))
                                }
                                ),
                                class? = classnames!("btn btn-link badge", "active" => {
                                    matches!(context.view_route, AppRoute::Deck(_, _)) ||
                                    matches!(context.view_route, AppRoute::Home) ||
                                    matches!(context.view_route, AppRoute::Card(_, _)) ||
                                    matches!(context.view_route, AppRoute::CardInDeck(_, _, _))
                                }
                                ),

                                data-badge="9"
                            ) {
                                : "decks"
                            }
                            a(
                                href = view_route_to_link(AppRoute::Stashes, &context),

                                style? = stylenames!("font-weight:bold;" =>
                                    matches!(context.view_route, AppRoute::Stashes)
                                ),
                                class? = classnames!("btn btn-link badge", "active" =>
                                    matches!(context.view_route, AppRoute::Stashes)
                                )

                            ) {
                                : "stashes"
                            }
                            a(
                                href = view_route_to_link(AppRoute::Settings, &context),

                                style? = stylenames!("font-weight:bold;" =>
                                    matches!(context.view_route, AppRoute::Settings)
                                ),
                                class? = classnames!("btn btn-link badge", "active" =>
                                    matches!(context.view_route, AppRoute::Settings)
                                )
                            ) {
                                : "settings"
                            }
                            : " ";
                            input(type="text", class="form-input input-inline", placeholder="search");
                            : " ";
                            a(href="#", class="btn btn-primary") {
                                : "login"
                            }
                        }
                    }
                    noscript {
                        div(class="toast toast-danger") {
                            : "grokdb requires JavaScript to function properly. Go to ";
                            a(href="http://enable-javascript.com/", target="_blank", style="color: #000000") {
                                : "http://enable-javascript.com/"
                            }
                            : " to enable JavaScript for your browser.";
                        }
                    }

                    // section {
                    // // section(class="container") {
                    //     // : ViewRouteResolver::new(&context)
                    |tmpl| {
                        match context.view_route.clone() {
                            AppRoute::Home => {
                                // TODO: fix
                                // NOTE: goes to DeckDetailComponent
                                unreachable!();
                                // tmpl << DeckDetailComponent::new(&context)
                            }
                            AppRoute::Settings => {
                                SettingsComponent(tmpl, &mut context);
                            }
                            AppRoute::Stashes => {
                                StashesComponent(tmpl, &context);
                            }
                            AppRoute::Deck(_deck_id, ref _deck_route) => {

                                DeckDetailComponent(tmpl, context);

                                // match deck_route {
                                //     &DeckRoute::New => tmpl << NewDeckComponent::new(&context)
                                // }

                            },
                            AppRoute::Card(_card_id, ref _card_route) => {
                                CardDetailComponent(tmpl, &mut context);
                            },
                            AppRoute::CardInDeck(_deck_id, _card_id, ref _card_route) => {
                                CardDetailComponent(tmpl, &mut context);
                            }
                        };
                    }

                    // }
                    // p : Page::new(format!("boop"))
                }

                footer(class="container grid-960") {
                    : "footer component"
                }

                |tmpl| {
                    match context.view_route {
                        AppRoute::Deck(_, DeckRoute::Review) =>  {
                            tmpl << html! {
                                script(type="text/javascript", src="https://cdnjs.cloudflare.com/ajax/libs/babel-polyfill/6.9.1/polyfill.min.js") {}
                                script(type="text/javascript", src="https://cdnjs.cloudflare.com/ajax/libs/react/15.1.0/react.js") {}
                                script(type="text/javascript", src="https://cdnjs.cloudflare.com/ajax/libs/react/15.1.0/react-dom.js") {}

                                // script(type="text/javascript", src="/assets/vendor.js") {}
                                script(type="text/javascript", src="/assets/deck_review.js") {}
                            };

                        },
                        AppRoute::Deck(_, DeckRoute::NewDeck) =>  {
                            tmpl << html! {
                                script(type="text/javascript", src="https://cdnjs.cloudflare.com/ajax/libs/babel-polyfill/6.9.1/polyfill.min.js") {}
                                script(type="text/javascript", src="https://cdnjs.cloudflare.com/ajax/libs/react/15.1.0/react.js") {}
                                script(type="text/javascript", src="https://cdnjs.cloudflare.com/ajax/libs/react/15.1.0/react-dom.js") {}

                                // script(type="text/javascript", src="/assets/vendor.js") {}
                                script(type="text/javascript") {
                                    // needs to be raw b/c of html escaping
                                    : raw!(format!("window.__PRE_RENDER_STATE__ = {};",
                                        view_route_to_pre_render_state(context.view_route.clone(), context)))
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

// components/ViewRouteResolver
// TODO: bikeshed

// struct ViewRouteResolver<'component, C: 'component> {
//     context: &'component C
// }

// impl<'component, 'a, 'b> ViewRouteResolver<'component, Context<'a, 'b>> {
//     fn new(context: &'component Context<'a, 'b>) -> Self {
//         ViewRouteResolver {
//             context: context
//         }
//     }
// }

// macro_rules! render_component(
//     ($tmpl: expr, $component: expr) => (
//         // $tmpl << html! {
//         //     : $component
//         // };
//         $tmpl << $component;
//     )
// );

// impl<'component, 'a, 'b> RenderOnce for ViewRouteResolver<'component, Context<'a, 'b>> {

//     fn render_once(self, tmpl: &mut TemplateBuffer) {

//         let ViewRouteResolver {context} = self;

//         match context.view_route {
//             AppRoute::Home => {
//                 render_component!(tmpl, DeckDetail::new(&context))
//             }
//             AppRoute::Settings => {
//                 render_component!(tmpl, Settings::new(&context))
//             }
//             AppRoute::Deck(_, ref deck_route) => {

//                 match deck_route {
//                     &DeckRoute::New => render_component!(tmpl, Settings::new(&context))
//                 }

//             }
//         }

//     }
// }

// components/StashesComponent
fn StashesComponent<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>) {
    println!("StashesComponent");
    tmpl << html! {
        div(class="container") {
            div(class="columns") {
                section(class="col-12") {
                    : "Stashes (work in progress)"
                }
            }
        }
    };
}

// components/SettingsComponent
fn SettingsComponent<'a, 'b>(tmpl: &mut TemplateBuffer, context: &mut Context<'a, 'b>) {
    println!("SettingsComponent");
    tmpl << html! {
        div(class="container") {
            div(class="columns") {
                section(class="col-12") {
                    : "Settings (work in progress)"
                }
            }
            // div(class="columns") {
            //     div(class="col-12") {
            //         button(class="btn btn-primary") {
            //             : "Edit"
            //         }
            //     }
            // }
        }
    };
}

fn BreadCrumbComponent<'a, 'b>(tmpl: &mut TemplateBuffer, context: &mut Context<'a, 'b>) {
    tmpl << html! {
        ul(class="breadcrumb") {

            li(class="breadcrumb-item") {
                a(href="#") {
                    : "Library"
                }
            }

            li(class="breadcrumb-item") {
                a(href="#", class="text-bold") {
                    : "Math"
                }
            }

            // last breadcrumb item
            li(class="breadcrumb-item") {
            }
        }

    };
}

fn DeckNavComponent<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>) {

    // derive deck_id from view_route
    let deck_id = match context.view_route {
        AppRoute::Deck(deck_id, _) => {
            deck_id
        },
        _ => {
            unreachable!();
        }
    };

    tmpl << html! {
        ul(class="menu grokdb-menu") {
            // li(class="menu-header") {
            //     h2(class="menu-header-text") {
            //         : "Deck #123"
            //     }

            // }
            // li(class="menu-header") {
            //     div(class="menu-header-text") {
            //         : "Deck #123"
            //     }
            // }

            li(class="menu-item") {
                div(class="chip") {
                    div(class="chip-content text-center text-bold") {
                        : "Deck #123"
                    }
                }
            }
            // li(class="menu-header") {
            //     div(class="menu-header-text") {
            //         : "#123"
            //     }
            // }

            li(class="menu-item") {
                a(href = view_route_to_link(AppRoute::Deck(deck_id, DeckRoute::Description),
                    &context),
                    style? = stylenames!("font-weight:bold;" =>
                        matches!(context.view_route, AppRoute::Deck(_, DeckRoute::Description))
                        ),
                    class? = classnames!("active" =>
                        matches!(context.view_route, AppRoute::Deck(_, DeckRoute::Description)))
                ) {
                    : "Description"
                }
            }
            li(class="menu-item") {
                a(
                    href = view_route_to_link(AppRoute::Deck(deck_id, DeckRoute::Decks(DecksPageQuery::NoQuery, Search::NoQuery)), &context),
                    style? = stylenames!("font-weight:bold;" => {
                        matches!(context.view_route, AppRoute::Deck(_, DeckRoute::Decks(_, _))) ||
                        matches!(context.view_route, AppRoute::Deck(_, DeckRoute::NewDeck))
                    }),
                    class? = classnames!("active" => {
                        matches!(context.view_route, AppRoute::Deck(_, DeckRoute::Decks(_, _))) ||
                        matches!(context.view_route, AppRoute::Deck(_, DeckRoute::NewDeck))
                    })
                ) {
                    : "Child Decks"
                }
            }
            li(class="menu-item") {
                a(
                    href = view_route_to_link(AppRoute::Deck(deck_id, DeckRoute::Cards), &context),
                    style? = stylenames!("font-weight:bold;" => {
                        matches!(context.view_route, AppRoute::Deck(_, DeckRoute::Cards)) ||
                        matches!(context.view_route, AppRoute::Deck(_, DeckRoute::NewCard))
                    }),
                    class? = classnames!("active" => {
                        matches!(context.view_route, AppRoute::Deck(_, DeckRoute::Cards)) ||
                        matches!(context.view_route, AppRoute::Deck(_, DeckRoute::NewCard))
                    })
                ) {
                    : "Cards"
                }
            }
            li(class="menu-item") {
                a(href = view_route_to_link(AppRoute::Deck(deck_id, DeckRoute::Review),
                    &context),
                    style? = stylenames!("font-weight:bold;" =>
                        matches!(context.view_route, AppRoute::Deck(_, DeckRoute::Review))
                        ),
                    class? = classnames!("active" =>
                        matches!(context.view_route, AppRoute::Deck(_, DeckRoute::Review)))
                ) {
                    : "Review this Deck"
                }
            }
            // li(class="divider") {}
            li(class="menu-item") {
                a(href = view_route_to_link(AppRoute::Deck(deck_id, DeckRoute::Meta),
                    &context),
                    style? = stylenames!("font-weight:bold;" =>
                        matches!(context.view_route, AppRoute::Deck(_, DeckRoute::Meta))
                        ),
                    class? = classnames!("active" =>
                        matches!(context.view_route, AppRoute::Deck(_, DeckRoute::Meta)))
                ) {
                    : "Meta"
                }
            }
            li(class="menu-item") {
                a(href = view_route_to_link(AppRoute::Deck(deck_id, DeckRoute::Settings),
                    &context),
                    style? = stylenames!("font-weight:bold;" =>
                        matches!(context.view_route, AppRoute::Deck(_, DeckRoute::Settings))
                        ),
                    class? = classnames!("active" =>
                        matches!(context.view_route, AppRoute::Deck(_, DeckRoute::Settings)))
                ) {
                    : "Settings"
                }
            }
        }
        div(class="divider") {}
        ul(class="menu grokdb-menu") {
            li(class="menu-item") {
                : "9000 cards"
            }
            li(class="menu-item") {
                : "9000 decks"
            }
            li(class="menu-item") {
                : "Last reviewed yesterday."
            }
            li(class="menu-item") {
                : "Last created a new card yesterday."
            }
        }
    };
}

fn DeckDetailComponent<'a, 'b>(tmpl: &mut TemplateBuffer, mut context: &mut Context<'a, 'b>) {

    let view_route = context.view_route.clone();

    let (deck_id, deck_route) = match view_route {
        AppRoute::Deck(deck_id, ref deck_route) => (deck_id, deck_route),
        _ => unreachable!()
    };

    // TODO: check if deck exists...

    tmpl << html! {

        div(class="container") {
            div(class="columns") {
                div(class="col-12") {
                    |tmpl| BreadCrumbComponent(tmpl, &mut context);
                }
            }
            section(class="columns") {
                section(class="column col-9") {
                    |tmpl| match deck_route {
                        &DeckRoute::NewCard => NewCardComponent(tmpl, &context),
                        &DeckRoute::NewDeck => NewDeckComponent(tmpl, &context),
                        &DeckRoute::Description => DeckDescriptionComponent(tmpl, &context),
                        &DeckRoute::Decks(_, _) => ChildDecksComponent(tmpl, &mut context),
                        &DeckRoute::Cards => DeckCardsComponent(tmpl, &context),
                        &DeckRoute::Meta => DeckMetaComponent(tmpl, &context),
                        &DeckRoute::Settings => DeckSettingsComponent(tmpl, &context),
                        &DeckRoute::Review => DeckReviewComponent(tmpl, &context)
                    };
                }
                section(class="column col-3") {
                    |tmpl| DeckNavComponent(tmpl, &context);
                }
            }
        }


    };
}

// components/NewCardComponent
fn NewCardComponent<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>) {
    tmpl << html! {

        div(class="container") {
            div(class="columns") {
                div(class="column") {
                    h5(style="margin-top:0;margin-bottom:0;", class="text-break") {
                        : "New Card"
                    }
                }
            }

            div(class="columns") {
                div(class="column") {
                    div(class="form-group") {
                        label(class="form-label", for="input-card-title") {
                            : "Title of card"
                        }
                        input(class="form-input", type="text", id="input-card-title", placeholder="Title of card");
                    }
                }
            }

            div(class="columns") {
                div(class="column") {
                    div(class="form-group") {
                        label(class="form-checkbox") {
                            input(type="checkbox");
                            i(class="form-icon") {}
                            : " ";
                            : "Disable from review.";
                        }
                    }
                }
            }

            div(class="columns") {
                div(class="column") {
                    div(class="btn-group btn-group-block") {
                        a(href="#", class="btn btn-primary")  {
                            : "Question"
                        }
                        : " ";
                        a(href="#", class="btn")  {
                            : "Answer"
                        }
                        : " ";
                        a(href="#", class="btn")  {
                            : "Description"
                        }
                    }
                }
            }

            div(class="columns") {
                div(class="column") {
                    ul(class="tab") {
                        li(class="tab-item active") {
                            a(href="#") {
                                : "Source"
                            }
                        }
                        li(class="tab-item") {
                            a(href="#") {
                                : "Render"
                            }
                        }
                    }
                }
            }

            div(class="columns") {
                div(class="column") {
                    div(class="form-group") {
                        label(class="form-checkbox") {
                            input(type="checkbox");
                            i(class="form-icon") {}
                            : " ";
                            : "Hide description during review.";
                        }
                    }
                }
            }

            div(class="columns") {
                div(class="column") {
                    div(class="form-group") {
                        textarea(
                            class="form-input",
                            id="input-deck-description",
                            placeholder="Question",
                            rows="6"
                        ) {
                        }
                    }
                }
            }

            div(class="columns") {
                div(class="column") {
                    a(href="#", class="btn btn-success") {
                        : "Add new Card"
                    }
                }
            }
        }

    };
}

fn NewDeckComponent<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>) {
    tmpl << html! {

        div(class="container") {
            div(class="columns") {
                div(class="column") {
                    h5(style="margin-top:0;margin-bottom:0;", class="text-break") {
                        : "New Deck"
                    }
                }
            }

            div(id="new-deck-container") {
                // : raw!(include_str!("react_components/deck_review"))

                div(class="columns") {
                    div(class="column") {
                        div(class="loading") {}
                    }
                }
            }

            // div(class="columns") {
            //     div(class="column") {
            //         div(class="form-group") {
            //             label(class="form-label", for="input-deck-name") {
            //                 : "Name"
            //             }
            //             input(class="form-input", type="text", id="input-deck-name", placeholder="Name for new deck");
            //         }
            //     }
            // }

            // div(class="columns") {
            //     div(class="column") {
            //         ul(class="tab") {
            //             li(class="tab-item active") {
            //                 a(href="#") {
            //                     : "Source"
            //                 }
            //             }
            //             li(class="tab-item") {
            //                 a(href="#") {
            //                     : "Render"
            //                 }
            //             }
            //         }
            //     }
            // }

            // div(class="columns") {
            //     div(class="column") {
            //         div(class="form-group") {
            //             label(class="form-label", for="input-deck-description") {
            //                 : "Description"
            //             }
            //             textarea(
            //                 class="form-input",
            //                 id="input-deck-description",
            //                 placeholder="Description for new deck",
            //                 rows="6"
            //             ) {
            //             }
            //         }
            //     }
            // }

            // div(class="columns") {
            //     div(class="column") {
            //         a(href="#", class="btn btn-success") {
            //             : "Add new Deck"
            //         }
            //     }
            // }
        }

    };
}

// components/DeckDescriptionComponent
fn DeckDescriptionComponent<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>) {
    tmpl << html! {

        div(class="container") {
            div(class="columns") {
                div(class="col-12") {
                    h5(style="margin-top:0;") {
                        : "Description for Mathematics"
                    }
                }
            }
            div(class="columns") {
                div(class="col-12") {
                    // button(class="btn btn-lg btn-primary") {
                    //     : "Edit"
                    // }
                    button(class="btn btn-primary") {
                        : "Edit"
                    }
                    // button(class="btn btn-sm btn-primary") {
                    //     : "Edit"
                    // }

                }
            }
        }

    };
}

fn DeckListItem<'a, 'b>(tmpl: &mut TemplateBuffer, context: &mut Context<'a, 'b>, deck_id: DeckID) {

    let deck: Deck = match context.api.get_deck(deck_id) {
        Err(_) => panic!(),
        Ok(deck) => deck
    };

    tmpl << html! {
        div(class = "card") {
            div(class="card-header") {
                h4(class="card-title") {
                    a(href = view_route_to_link(
                        AppRoute::Deck(deck_id, DeckRoute::Decks(DecksPageQuery::NoQuery, Search::NoQuery)), &context)) {

                        : &deck.name
                    }
                }
                h6(class="card-meta") {
                    : "Deck #";
                    : deck_id;
                }
            }
            div(class="card-body") {
                : "Last reviewed yesterday.";
                : " ";
                : "500 cards.";
                : " ";
                : "500 decks.";
            }
        }
    };
}

fn ChildDecksComponent<'a, 'b>(tmpl: &mut TemplateBuffer, mut context: &mut Context<'a, 'b>) {

    // derive deck_id from view_route
    let (deck_id, decks_page_request): (DeckID, DecksPageRequest) = match context.view_route {
        AppRoute::Deck(deck_id, DeckRoute::Decks(ref page_query, ref search_query)) => {

            match page_query {
                &DecksPageQuery::NoQuery => {

                    let number_of_pages = match context.api.children_of_deck_count(deck_id) {
                        Err(_) => {
                            panic!("Panicked on context.api.children_of_deck_count()");
                        },
                        Ok(number_of_pages) => number_of_pages
                    };

                    let decks_page_request = DecksPageRequest {
                        page: 1,
                        per_page: 25,
                        sort: DecksPageSort::DeckTitle(SortOrder::Ascending),
                        number_of_pages: number_of_pages
                    };

                    (deck_id, decks_page_request)
                },
                &DecksPageQuery::Query(page, ref sort) => {

                    let number_of_pages = match context.api.children_of_deck_count(deck_id) {
                        Err(_) => {
                            panic!("Panicked on context.api.children_of_deck_count()");
                        },
                        Ok(number_of_pages) => number_of_pages
                    };

                    let decks_page_request = DecksPageRequest {
                        page: page,
                        per_page: 25,
                        sort: sort.clone(),
                        number_of_pages: number_of_pages
                    };

                    (deck_id, decks_page_request)
                }
            }

        },
        _ => {
            unreachable!();
        }
    };

    tmpl << html! {

        div(class="container") {

            div(class="columns") {
                div(class="column") {
                    h5(style="margin-top:0;margin-bottom:0;", class="text-break") {
                        small(class="label") {
                            : "Decks within"
                        }
                        : " ";
                        : "Math";
                    }

                }
                // div(class="column") {
                //     div(class="input-group inline float-right") {
                //         input(type="text", class="form-input input-inline", placeholder="Search within this deck");
                //         button(class="btn btn-primary input-group-btn") {
                //             : "Search"
                //         }
                //     }
                // }
            }

            div(class="columns") {
                div(class="column") {
                    div(class="input-group") {
                        input(type="text", class="form-input", placeholder="Search decks within this deck");
                        button(class="btn btn-primary input-group-btn") {
                            : "Search"
                        }
                    }
                }
            }

            div(class="columns") {
                div(class="column") {
                    a(href = view_route_to_link(AppRoute::Deck(deck_id, DeckRoute::NewDeck),
                            &context),
                        style="background-color: #32b643;border-color: #30ae40;",
                        class="btn btn-primary") {
                        : "New Deck"
                    }
                }

                div(class="column", style="display: flex;justify-content: flex-end") {
                    : " ";
                    select(class="form-select") {
                        option {
                            : "Order by Recent"
                        }
                        option {
                            : "Order by Least Recent"
                        }
                    }
                    : " ";
                    select(class="form-select", style="margin-left:5px;") {
                        option {
                            : "Sort by Updated"
                        }
                        option {
                            : "Choose an option"
                        }
                        option {
                            : "Choose an option"
                        }
                    }
                    : " ";
                }
            }

            div(class="columns") {
                div(class="column") {
                    ul(class="pagination", style="width:100%;text-align:center;padding:0;margin:0;") {
                        li(class="page-item") {
                            a(href="#") {
                                : "Previous"
                            }
                        }
                        li(class="page-item active") {
                            a(href="#") {
                                : "1"
                            }
                        }
                        li(class="page-item") {
                            a(href="#") {
                                : "2"
                            }
                        }
                        li(class="page-item") {
                            a(href="#") {
                                : "3"
                            }
                        }
                        li(class="page-item") {
                            a(href="#") {
                                : "Next"
                            }
                        }
                    }
                }
            }

            // div(class="divider") {}

            |tmpl| {

                // fetch list of decks
                let decks = match (&mut context).api.children_of_deck(deck_id, decks_page_request) {
                    Err(_) => {
                        // TODO: panic error
                        panic!();
                    },
                    Ok(decks) => decks
                };

                for deck in &decks {
                    &mut *tmpl << html! {
                        div(class="columns") {
                            div(class="col-12") {
                                |tmpl| DeckListItem(tmpl, &mut context, deck.id);
                            }
                        }
                    }
                };
            }

            // div(class="divider") {}

            div(class="columns") {
                div(class="column") {
                    ul(class="pagination", style="width:100%;text-align:center;padding:0;margin:0;") {
                        li(class="page-item") {
                            a(href="#") {
                                : "Previous"
                            }
                        }
                        li(class="page-item active") {
                            a(href="#") {
                                : "1"
                            }
                        }
                        li(class="page-item") {
                            a(href="#") {
                                : "2"
                            }
                        }
                        li(class="page-item") {
                            a(href="#") {
                                : "3"
                            }
                        }
                        li(class="page-item") {
                            a(href="#") {
                                : "Next"
                            }
                        }
                    }
                }
            }
        }
    };
}

fn CardListItem<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>, deck_id: DeckID, card_id: CardID) {

    tmpl << html! {
        div(class = "card") {
            div(class="card-header") {
                h4(class="card-title") {
                    a(href = view_route_to_link(
                        AppRoute::CardInDeck(deck_id, card_id, CardRoute::Profile), &context)) {

                        : "Microsoft"
                    }
                }
                h6(class="card-meta") {
                    : "Card #";
                    : card_id;
                }
            }
            div(class="card-body") {
                : "Last reviewed yesterday"
            }
        }
    };
}

fn DeckCardsComponent<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>) {

    // derive deck_id from view_route
    let deck_id = match context.view_route {
        AppRoute::Deck(deck_id, _) => {
            deck_id
        },
        _ => {
            unreachable!();
        }
    };

    tmpl << html! {

        div(class="container") {

            div(class="columns") {
                // div(class="col-12") {
                //     h5(style="margin-top:0;") {
                //         : "Cards within Mathematics"
                //     }
                // }
                div(class="column") {
                    h5(style="margin-top:0;margin-bottom:0;", class="text-break") {
                        small(class="label") {
                            : "Cards within"
                        }
                        : " ";
                        : "Math";
                    }
                }
                // div(class="column") {
                //     div(class="input-group inline float-right") {
                //         input(type="text", class="form-input input-inline", placeholder="Search within this deck");
                //         button(class="btn btn-primary input-group-btn") {
                //             : "Search"
                //         }
                //     }
                // }
            }

            div(class="columns") {
                div(class="column") {
                    div(class="input-group") {
                        input(type="text", class="form-input", placeholder="Search cards within this deck");
                        button(class="btn btn-primary input-group-btn") {
                            : "Search"
                        }
                    }
                }
            }

            div(class="columns") {
                div(class="column") {
                    a(href = view_route_to_link(AppRoute::Deck(deck_id, DeckRoute::NewCard),
                            &context),
                        style="background-color: #32b643;border-color: #30ae40;",
                        class="btn btn-primary") {
                        : "New Card"
                    }
                    : " ";
                    a(href = view_route_to_link(AppRoute::Deck(deck_id, DeckRoute::Review),
                        &context), class="btn") {
                        : "Review this Deck"
                    }
                }
            // div(class="column") {
            //     div(class="input-group inline float-right") {
            //         input(type="text", class="form-input input-inline", placeholder="Search within this deck");
            //         button(class="btn btn-primary input-group-btn") {
            //             : "Search"
            //         }
            //     }
            // }

                div(class="column", style="display: flex;justify-content: flex-end") {
                    : " ";
                    select(class="form-select") {
                        option {
                            : "Order by Recent"
                        }
                        option {
                            : "Order by Least Recent"
                        }
                    }
                    : " ";
                    select(class="form-select", style="margin-left:5px;") {
                        option {
                            : "Sort by Updated"
                        }
                        option {
                            : "Choose an option"
                        }
                        option {
                            : "Choose an option"
                        }
                    }
                    : " ";
                }

                // div(class="column") {
                //     div(class="input-group") {
                //         input(type="text", class="form-input input-inline", placeholder="search");
                //         button(class="btn btn-primary input-group-btn") {
                //             : "Search"
                //         }
                //     }
                // }
            }
            // div(class="columns") {
            //     div(class="column", style="display: flex;justify-content: flex-end") {
            //         : " ";
            //         select(class="form-select") {
            //             option {
            //                 : "Order by Recent"
            //             }
            //             option {
            //                 : "Order by Least Recent"
            //             }
            //         }
            //         : " ";
            //         select(class="form-select", style="margin-left:5px;") {
            //             option {
            //                 : "Sort by Updated"
            //             }
            //             option {
            //                 : "Choose an option"
            //             }
            //             option {
            //                 : "Choose an option"
            //             }
            //         }
            //         : " ";
            //     }



            //     // div(class="column") {
            //     //     select(class="form-select") {
            //     //         option {
            //     //             : "Choose an option"
            //     //         }
            //     //         option {
            //     //             : "Choose an option"
            //     //         }
            //     //         option {
            //     //             : "Choose an option"
            //     //         }
            //     //     }
            //     // }
            // }


            div(class="columns") {
                div(class="column") {
                    ul(class="pagination", style="width:100%;text-align:center;padding:0;margin:0;") {
                        li(class="page-item") {
                            a(href="#") {
                                : "Previous"
                            }
                        }
                        li(class="page-item active") {
                            a(href="#") {
                                : "1"
                            }
                        }
                        li(class="page-item") {
                            a(href="#") {
                                : "2"
                            }
                        }
                        li(class="page-item") {
                            a(href="#") {
                                : "3"
                            }
                        }
                        li(class="page-item") {
                            a(href="#") {
                                : "Next"
                            }
                        }
                    }
                }
            }

            // div(class="divider") {}


            |tmpl| {
                for i in 1..25 {
                    &mut *tmpl << html! {
                        div(class="columns") {
                            div(class="col-12") {
                                |tmpl| CardListItem(tmpl, &context, deck_id, i);
                            }
                        }
                    }
                };
            }

            // div(class="divider") {}

            div(class="columns") {
                div(class="column") {
                    ul(class="pagination", style="width:100%;text-align:center;padding:0;margin:0;") {
                        li(class="page-item") {
                            a(href="#") {
                                : "Previous"
                            }
                        }
                        li(class="page-item active") {
                            a(href="#") {
                                : "1"
                            }
                        }
                        li(class="page-item") {
                            a(href="#") {
                                : "2"
                            }
                        }
                        li(class="page-item") {
                            a(href="#") {
                                : "3"
                            }
                        }
                        li(class="page-item") {
                            a(href="#") {
                                : "Next"
                            }
                        }
                    }
                }
            }

            // @ for i in 1..25 {
            //     div(class="columns") {
            //         div(class="col-12") {
            //             |tmpl| CardItem(tmpl, &context, &i);
            //         }
            //     }

            // }
        }
    };
}

fn DeckMetaComponent<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>) {
    tmpl << html! {

        : "deck meta"
    };
}

fn DeckSettingsComponent<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>) {
    tmpl << html! {

        : "deck settings"
    };
}

fn DeckReviewComponent<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>) {
    CardProfileReviewComponent(tmpl, context);
}

fn CardDetailComponent<'a, 'b>(tmpl: &mut TemplateBuffer, mut context: &mut Context<'a, 'b>) {

    let view_route = context.view_route.clone();

    let (card_id, card_route) = match view_route {
        AppRoute::Card(card_id, ref card_route) => (card_id, card_route),
        AppRoute::CardInDeck(_deck_id, card_id, ref card_route) => (card_id, card_route),
        _ => unreachable!()
    };

    // TODO: check if card exists...

    tmpl << html! {

        div(class="container") {
            div(class="columns") {
                div(class="col-12") {
                    |tmpl| BreadCrumbComponent(tmpl, &mut context);
                }
            }
            div(class="columns") {
                div(class="column col-9") {
                    |tmpl| match card_route {
                        &CardRoute::Profile => CardProfileComponent(tmpl, &context),
                        &CardRoute::Review => CardProfileReviewComponent(tmpl, &context)
                    };
                }
                div(class="column col-3") {
                    |tmpl| CardNavComponent(tmpl, &context);
                }
            }

        }


    };
}

fn CardProfileComponent<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>) {
    tmpl << html! {
        div(class="container") {
            div(class="columns") {
                div(class="column") {
                    // a(href="#", class="btn btn-primary float-left") {
                    //     : "Edit"
                    // }
                    // : " ";
                    h5(style="margin-top:0;margin-bottom:0;", class="text-break") {
                        : "What does the fox say?"
                    }
                }
            }

            div(class="columns") {
                div(class="column") {
                    a(href="#", class="btn btn-primary btn-sm") {
                        : "Edit"
                    }
                    : " ";

                    a(href = view_route_to_link(
                        resolve_card_route_link(&context, CardRoute::Review),
                        &context),
                        class="btn btn-sm"
                    ) {
                        : "Review this card"
                    }
                }
            }

            div(class="columns") {
                div(class="column") {
                    div(class="btn-group btn-group-block") {
                        a(href="#", class="btn btn-primary")  {
                            : "Question"
                        }
                        : " ";
                        a(href="#", class="btn")  {
                            : "Answer"
                        }
                        : " ";
                        a(href="#", class="btn")  {
                            : "Description"
                        }
                    }
                }
            }

            div(class="columns") {
                div(class="column") {
                    ul(class="tab") {
                        li(class="tab-item active") {
                            a(href="#") {
                                : "Source"
                            }
                        }
                        li(class="tab-item") {
                            a(href="#") {
                                : "Render"
                            }
                        }
                    }
                }
            }

        }

    };
}

fn CardProfileReviewComponent<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>) {

    // derive deck_id from view_route
    let (deck_id, card_id) = match context.view_route {
        AppRoute::Card(card_id, _) => {

            let deck_id = 9000;

            (deck_id, card_id)
        },
        AppRoute::CardInDeck(deck_id, card_id, _) => {
            (deck_id, card_id)
        }
        _ => {
            unreachable!();
        }
    };

    tmpl << html! {

        div(class="container") {
            div(class="columns") {
                div(class="column") {
                    h5(style="margin-top:0;margin-bottom:0;", class="text-break") {
                        small(class="label") {
                            : "Reviewing"
                        }
                        : " ";
                        : "What does the fox say?"
                    }
                }
            }

            div(class="columns") {
                div(class="column") {
                    span(class="label") {
                        : "Card #123"
                    }
                    : " ";
                    a(href = view_route_to_link(
                        AppRoute::CardInDeck(deck_id, card_id, CardRoute::Profile),
                        &context),
                        class="btn btn-sm"
                    ) {
                        : "Go to card profile"
                    }
                }
            }

            div(id="deck-review-container") {
                : raw!(include_str!("react_components/deck_review"))
                // div(class="columns") {
                //     div(class="column") {
                //         div(class="loading") {}
                //     }
                // }
            }

            // TODO: clean up

            // div(class="columns") {
            //     div(class="column") {
            //         div(class="btn-group btn-group-block") {
            //             a(href="#", class="btn btn-primary")  {
            //                 : "Question"
            //             }
            //             : " ";
            //             a(href="#", class="btn")  {
            //                 : "Description"
            //             }
            //         }
            //     }
            // }

            // div(class="columns") {
            //     div(class="column") {
            //         ul(class="tab") {
            //             li(class="tab-item active") {
            //                 a(href="#") {
            //                     : "Source"
            //                 }
            //             }
            //             li(class="tab-item") {
            //                 a(href="#") {
            //                     : "Render"
            //                 }
            //             }
            //         }
            //     }
            // }

            // div(class="columns") {
            //     div(class="column col-9") {
            //         a(href = "#",
            //             class="btn btn-block"
            //         ) {
            //             : "Reveal Answer"
            //         }
            //     }

            //     div(class="column col-3") {
            //         a(href = "#",
            //             class="btn btn-block"
            //         ) {
            //             : "Skip Card"
            //         }
            //     }
            // }

            hr;

            div(class="columns") {
                div(class="column col-12") {
                    a(href = "#",
                        class="btn btn-block"
                    ) {
                        : "Custom Score"
                    }
                }

            }


            div(class="columns") {
                div(class="column col-4") {
                    a(href = "#",
                        class="btn btn-block"
                    ) {
                        : "Fail"
                    }
                }


                div(class="column col-4") {
                    a(href = "#",
                        class="btn btn-block"
                    ) {
                        : "Success"
                    }
                }

                div(class="column col-4") {
                    a(href = "#",
                        class="btn btn-block"
                    ) {
                        : "Reset Score"
                    }
                }

            }

            hr;

            div(class="columns") {
                div(class="column col-12") {
                    a(href = "#",
                        class="btn btn-block"
                    ) {
                        : "Cancel Custom Score"
                    }
                }

            }

            div(class="columns") {
                div(class="column col-6") {
                    a(href = "#",
                        class="btn btn-block btn-primary"
                    ) {
                        : "Append To Score"
                    }
                }


                div(class="column col-6") {
                    a(href = "#",
                        class="btn btn-block"
                    ) {
                        : "Set Score"
                    }
                }

            }

            div(class="columns") {
                div(class="column col-6") {
                    div(class="input-group") {

                        span(class="input-group-addon") {
                            : "Success Votes"
                        }
                        input(type="text", class="form-input", placeholder="Success Votes");
                        button(class="btn btn-primary input-group-btn") {
                            : "-"
                        }
                        button(class="btn btn-primary input-group-btn") {
                            : "+"
                        }
                    }
                }

                div(class="column col-6") {
                    div(class="input-group") {
                        span(class="input-group-addon") {
                            : "Fail Votes"
                        }
                        input(type="text", class="form-input", placeholder="Fail Votes");
                        button(class="btn btn-primary input-group-btn") {
                            : "-"
                        }
                        button(class="btn btn-primary input-group-btn") {
                            : "+"
                        }
                    }
                }


            }

            hr;

            div(class="columns") {
                div(class="column col-9") {
                    a(href = "#",
                        class="btn btn-block"
                    ) {
                        : "Commit Score & Next Card"
                    }
                }

                div(class="column col-3") {
                    a(href = "#",
                        class="btn btn-block"
                    ) {
                        : "Skip Card"
                    }
                }
            }

            hr;

            div(class="columns") {

                div(class="column col-6") {
                    a(href = "#",
                        class="btn btn-block"
                    ) {
                        : "Yes, skip"
                    }
                }

                div(class="column col-6") {
                    a(href = "#",
                        class="btn btn-block btn-primary"
                    ) {
                        : "No, don't skip"
                    }
                }
            }


        }

    };
}

// resolver function to resolve to specific routes
fn resolve_card_route_link<'a, 'b>(context: &Context<'a, 'b>, card_route: CardRoute) -> AppRoute {
    match context.view_route {
        AppRoute::Card(card_id, ref _card_route) => {
            AppRoute::Card(card_id, card_route)
        },
        AppRoute::CardInDeck(deck_id, card_id, ref _card_route) => {
            AppRoute::CardInDeck(deck_id, card_id, card_route)
        },
        _ => unreachable!()
    }
}

fn CardNavComponent<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>) {

    let deck_id = default!();

    tmpl << html! {
        ul(class="menu") {

            li(class="menu-item") {
                div(class="chip") {
                    div(class="chip-content text-center text-bold") {
                        : "Card #123"
                    }
                }
            }

            li(class="menu-item") {
                a(href = view_route_to_link(
                    resolve_card_route_link(&context, CardRoute::Profile),
                    &context),
                    style? = stylenames!("font-weight:bold;" =>
                        matches!(context.view_route, AppRoute::Card(_, CardRoute::Profile)) ||
                        matches!(context.view_route, AppRoute::CardInDeck(_, _, CardRoute::Profile))
                        ),
                    class? = classnames!("active" =>
                        matches!(context.view_route, AppRoute::Card(_, CardRoute::Profile)) ||
                        matches!(context.view_route, AppRoute::CardInDeck(_, _, CardRoute::Profile))
                    )
                ) {
                    : "Detail"
                }
            }

            li(class="menu-item") {
                a(href = view_route_to_link(
                    resolve_card_route_link(&context, CardRoute::Review),
                    &context),
                    style? = stylenames!("font-weight:bold;" =>
                        matches!(context.view_route, AppRoute::Card(_, CardRoute::Review)) ||
                        matches!(context.view_route, AppRoute::CardInDeck(_, _, CardRoute::Review))
                        ),
                    class? = classnames!("active" =>
                        matches!(context.view_route, AppRoute::Card(_, CardRoute::Review)) ||
                        matches!(context.view_route, AppRoute::CardInDeck(_, _, CardRoute::Review))
                    )
                ) {
                    : "Review this card"
                }
            }
            // li(class="divider") {}
            li(class="menu-item") {
                a(href = view_route_to_link(AppRoute::Deck(deck_id, DeckRoute::Meta),
                    &context),
                    style? = stylenames!("font-weight:bold;" =>
                        matches!(context.view_route, AppRoute::Deck(_, DeckRoute::Meta))
                        ),
                    class? = classnames!("active" =>
                        matches!(context.view_route, AppRoute::Deck(_, DeckRoute::Meta)))
                ) {
                    : "Meta"
                }
            }
            li(class="menu-item") {
                a(href = view_route_to_link(AppRoute::Deck(deck_id, DeckRoute::Settings),
                    &context),
                    style? = stylenames!("font-weight:bold;" =>
                        matches!(context.view_route, AppRoute::Deck(_, DeckRoute::Settings))
                        ),
                    class? = classnames!("active" =>
                        matches!(context.view_route, AppRoute::Deck(_, DeckRoute::Settings)))
                ) {
                    : "Settings"
                }
            }
        }
        div(class="divider") {}
        ul(class="menu") {
            li(class="menu-item") {
                : "9000 cards"
            }
            li(class="menu-item") {
                : "9000 decks"
            }
            li(class="menu-item") {
                : "Last reviewed yesterday."
            }
            li(class="menu-item") {
                : "Last created a new card yesterday."
            }
        }
    };
}
