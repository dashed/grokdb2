/* 3rd-party imports */

use templates::{RenderOnce, TemplateBuffer, Template, FnRenderer};

/* local imports */

use contexts::{Context};
use route::helpers::{view_route_to_link};
use route::constants::{AppRoute, DeckRoute};

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
                                    matches!(context.view_route, AppRoute::Home)
                                }
                                ),
                                class? = classnames!("btn btn-link badge", "active" => {
                                    matches!(context.view_route, AppRoute::Deck(_, _)) ||
                                    matches!(context.view_route, AppRoute::Home)
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

// components/BreadCrumbComponent
fn BreadCrumbComponent<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>) {
    tmpl << html! {
        ul(class="breadcrumb") {
            li(class="breadcrumb-item") {
                a(href="#") {
                    : "Library"
                }
            }
            li(class="breadcrumb-item") {
                a(href="#") {
                    : "Math"
                }
            }
        }

    };
}

// components/DeckNavComponent
fn DeckNavComponent<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>) {

    let deck_id = default!();

    tmpl << html! {
        ul(class="menu") {
            // li(class="menu-header") {
            //     h2(class="menu-header-text") {
            //         : "Deck #123"
            //     }

            // }
            li(class="menu-header") {
                div(class="menu-header-text") {
                    : "Deck #123"
                }
            }

            li(class="menu-item") {
                div(class="chip") {
                    div(class="chip-content") {
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
                a(href = view_route_to_link(AppRoute::Deck(deck_id, DeckRoute::Decks),
                    &context),
                    style? = stylenames!("font-weight:bold;" =>
                        matches!(context.view_route, AppRoute::Deck(_, DeckRoute::Decks))
                        ),
                    class? = classnames!("active" =>
                        matches!(context.view_route, AppRoute::Deck(_, DeckRoute::Decks)))
                    // class = "active"
                ) {
                    : "Child Decks"
                }
            }
            li(class="menu-item") {
                a(href = view_route_to_link(AppRoute::Deck(deck_id, DeckRoute::Cards),
                    &context),
                    style? = stylenames!("font-weight:bold;" =>
                        matches!(context.view_route, AppRoute::Deck(_, DeckRoute::Cards))
                        ),
                    class? = classnames!("active" =>
                        matches!(context.view_route, AppRoute::Deck(_, DeckRoute::Cards)))
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

// components/DeckDetailComponent
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
                        &DeckRoute::Review => DeckReviewComponent(tmpl, &context),
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
        : "new card"
    };
}

// components/NewDeckComponent
fn NewDeckComponent<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>) {
    tmpl << html! {
        : "new deck"
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

// components/ChildDecksComponent
fn ChildDecksComponent<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>) {
    tmpl << html! {

        : "deck children"
    };
}

// components/DeckCardsComponent
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
                        : "Cards within MathematicsMathematicsMathematicsMathematicsMathematicsMathematicsMathematicsMathematicsMathematicsMathematicsMathematicsMathematics"
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
            @ for i in 1..25 {
                div(class="columns") {
                    div(class="col-12") {
                        |tmpl| CardItem(tmpl, &context, &i);
                    }
                }

            }
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

// components/CardItem
fn CardItem<'a, 'b>(tmpl: &mut TemplateBuffer, context: &Context<'a, 'b>, card_id: &u32) {
    tmpl << html! {
        div(class = "card") {
            div(class="card-header") {
                h4(class="card-title") {
                    a(href="#") {
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

