/* 3rd-party imports */

use templates::{RenderOnce, TemplateBuffer, Template, FnRenderer};

/* local imports */

use contexts::{Context};
use route::helpers::{view_route_to_link};
use route::constants::{AppRoute, DeckRoute, CardRoute};

////////////////////////////////////////////////////////////////////////////////

pub fn AppComponent<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>, title: String) {

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

                    match context.view_route {
                        AppRoute::Deck(_, DeckRoute::NewCard) |
                        AppRoute::Deck(_, DeckRoute::NewDeck) =>  {
                            tmpl << html! {
                                style {
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
                                }
                            };

                        },
                        _ => {}
                    };
                }
            }
            body {
                section(class="container grid-960") {
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
                        match context.view_route {
                            AppRoute::Home => {
                                // TODO: fix
                                // NOTE: goes to DeckDetailComponent
                                unreachable!();
                                // tmpl << DeckDetailComponent::new(&context)
                            }
                            AppRoute::Settings => {
                                SettingsComponent(tmpl, &context);
                            }
                            AppRoute::Stashes => {
                                StashesComponent(tmpl, &context);
                            }
                            AppRoute::Deck(_deck_id, ref _deck_route) => {

                                DeckDetailComponent(tmpl, &context);

                                // match deck_route {
                                //     &DeckRoute::New => tmpl << NewDeckComponent::new(&context)
                                // }

                            },
                            AppRoute::Card(_card_id, ref _card_route) => {
                                CardDetailComponent(tmpl, &context);
                            },
                            AppRoute::CardInDeck(_deck_id, _card_id, ref _card_route) => {
                                CardDetailComponent(tmpl, &context);
                            }
                        };
                    }

                    // }
                    // p : Page::new(format!("boop"))
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
fn SettingsComponent<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>) {
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

fn BreadCrumbComponent<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>) {
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

    let deck_id = default!();

    tmpl << html! {
        ul(class="menu") {
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
                    href = view_route_to_link(AppRoute::Deck(deck_id, DeckRoute::Decks), &context),
                    style? = stylenames!("font-weight:bold;" => {
                        matches!(context.view_route, AppRoute::Deck(_, DeckRoute::Decks)) ||
                        matches!(context.view_route, AppRoute::Deck(_, DeckRoute::NewDeck))
                    }),
                    class? = classnames!("active" => {
                        matches!(context.view_route, AppRoute::Deck(_, DeckRoute::Decks)) ||
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

fn DeckDetailComponent<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>) {
    let (deck_id, deck_route) = {
        match context.view_route {
            AppRoute::Deck(deck_id, ref deck_route) => (deck_id, deck_route),
            _ => unreachable!()
        }
    };

    // TODO: check if deck exists...

    tmpl << html! {

        div(class="container") {
            div(class="columns") {
                div(class="col-12") {
                    |tmpl| BreadCrumbComponent(tmpl, &context);
                }
            }
            section(class="columns") {
                section(class="column col-9") {
                    |tmpl| match deck_route {
                        &DeckRoute::NewCard => NewCardComponent(tmpl, &context),
                        &DeckRoute::NewDeck => NewDeckComponent(tmpl, &context),
                        &DeckRoute::Description => DeckDescriptionComponent(tmpl, &context),
                        &DeckRoute::Decks => ChildDecksComponent(tmpl, &context),
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

// components/NewDeckComponent
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

            div(class="columns") {
                div(class="column") {
                    div(class="form-group") {
                        label(class="form-label", for="input-deck-name") {
                            : "Name"
                        }
                        input(class="form-input", type="text", id="input-deck-name", placeholder="Name for new deck");
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
                        label(class="form-label", for="input-deck-description") {
                            : "Description"
                        }
                        textarea(
                            class="form-input",
                            id="input-deck-description",
                            placeholder="Description for new deck",
                            rows="6"
                        ) {
                        }
                    }
                }
            }

            div(class="columns") {
                div(class="column") {
                    a(href="#", class="btn btn-success") {
                        : "Add new Deck"
                    }
                }
            }
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

fn DeckListItem<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>, deck_id: u64) {

    // let deck_id = default!();

    tmpl << html! {
        div(class = "card") {
            div(class="card-header") {
                h4(class="card-title") {
                    a(href = view_route_to_link(
                        AppRoute::Deck(deck_id, DeckRoute::Decks), &context)) {

                        : "Microsoft"
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

fn ChildDecksComponent<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>) {

    let deck_id = default!();

    tmpl << html! {

        div(class="container") {
            div(class="columns") {
                div(class="column") {
                    h5(style="margin-top:0;", class="text-break") {
                        : "Decks within Math"
                    }
                }
                div(class="column") {
                    div(class="input-group inline float-right") {
                        input(type="text", class="form-input input-inline", placeholder="Search within this deck");
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
            }
            div(class="divider") {}
            |tmpl| {
                for i in 1..25 {
                    &mut *tmpl << html! {
                        div(class="columns") {
                            div(class="col-12") {
                                |tmpl| DeckListItem(tmpl, &context, i);
                            }
                        }
                    }
                };
            }
        }
    };
}

fn CardListItem<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>, card_id: u64) {

    let deck_id = default!();

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

    let deck_id = default!();

    tmpl << html! {

        div(class="container") {
            div(class="columns") {
                // div(class="col-12") {
                //     h5(style="margin-top:0;") {
                //         : "Cards within Mathematics"
                //     }
                // }
                div(class="column") {
                    h5(style="margin-top:0;", class="text-break") {
                        : "Cards within Math"
                    }
                }
                div(class="column") {
                    div(class="input-group inline float-right") {
                        input(type="text", class="form-input input-inline", placeholder="Search within this deck");
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
            div(class="divider") {}
            |tmpl| {
                for i in 1..25 {
                    &mut *tmpl << html! {
                        div(class="columns") {
                            div(class="col-12") {
                                |tmpl| CardListItem(tmpl, &context, i);
                            }
                        }
                    }
                };
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

// components/DeckMetaComponent
fn DeckMetaComponent<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>) {
    tmpl << html! {

        : "deck meta"
    };
}


// components/DeckSettingsComponent
fn DeckSettingsComponent<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>) {
    tmpl << html! {

        : "deck settings"
    };
}

// components/DeckReviewComponent
fn DeckReviewComponent<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>) {
    tmpl << html! {
        : "review deck"
    };
}

fn CardDetailComponent<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>) {
    let (card_id, card_route) = {
        match context.view_route {
            AppRoute::Card(card_id, ref card_route) => (card_id, card_route),
            AppRoute::CardInDeck(_deck_id, card_id, ref card_route) => (card_id, card_route),
            _ => unreachable!()
        }
    };

    // TODO: check if card exists...

    tmpl << html! {

        div(class="container") {
            div(class="columns") {
                div(class="col-12") {
                    |tmpl| BreadCrumbComponent(tmpl, &context);
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
                    div(class="btn-group btn-group-block") {
                        a(href="#", class="btn btn-primary")  {
                            : "Question"
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
