/* rust lib imports */

use std::cell::RefCell;
use std::rc::Rc;

/* local imports */

use context::{self, Context};
use types::{DeckID, ReviewCount};
use errors::RawAPIError;
use api::{configs, decks};

/* ////////////////////////////////////////////////////////////////////////// */

#[inline]
pub fn set_up_user(context: Rc<RefCell<Context>>) -> Result<(), RawAPIError> {

    assert!(context.borrow().is_write_locked());

    // set up root deck

    let should_create_root_deck: bool = match get_root_deck(context.clone()) {
        Ok(maybe_deck_id) => {
            match maybe_deck_id {
                None => true,
                Some(deck_id) => {
                    match is_root_deck(context.clone(), deck_id) {
                        Ok(confirm) => {
                            !confirm
                        },
                        Err(why) => {
                            return Err(why);
                        }
                    }
                }
            }
        },
        Err(why) => {
            return Err(why);
        }
    };

    if should_create_root_deck {
        // root deck not found.
        // create a root deck.
        let request = decks::CreateDeck {
            name: "Library".to_string(),
            description: "".to_string(),
        };

        let root_deck = try!(decks::create_deck(context.clone(), request));

        try!(set_root_deck(context.clone(), root_deck.id));

    }

    // set up review count

    match try!(get_review_count(context.clone())) {
        None => {
            try!(set_review_count(context, 0));
        },
        Some(_) => {
            // review count exists
        }
    }

    return Ok(());
}

#[inline]
pub fn set_review_count(context: Rc<RefCell<Context>>,
    review_count: ReviewCount) -> Result<ReviewCount, RawAPIError> {

    assert!(context.borrow().is_write_locked());

    // TODO: change this for monetized version by fetching from a user table

    let encoded = format!("{}", review_count);

    match configs::set_config(context.clone(), configs::CONFIG_REVIEW_COUNT_KEY.to_string(), encoded) {
        Ok(_) => {
            // config successful
        },
        Err(why) => {
            return Err(why);
        }
    }

    return Ok(review_count);
}

#[inline]
pub fn get_review_count(context: Rc<RefCell<Context>>) -> Result<Option<ReviewCount>, RawAPIError> {

    assert!(context.borrow().is_read_locked());

    // TODO: change this for monetized version by fetching from a user table

    match configs::get_config(context.clone(), configs::CONFIG_REVIEW_COUNT_KEY.to_string()) {
        Ok(config) => {

            match config {
                None => {
                    return Ok(None);
                },
                Some(config) => {

                    let review_count = config.value;

                    match review_count.parse::<ReviewCount>() {
                        Ok(review_count) => {
                            return Ok(Some(review_count));
                        }
                        Err(_) => {
                            return Ok(None);
                        }
                    }
                }
            }

        },
        Err(why) => {
            return Err(why);
        }
    }

}

#[inline]
pub fn set_root_deck(context: Rc<RefCell<Context>>, root_deck_id: DeckID) -> Result<DeckID, RawAPIError> {

    assert!(context.borrow().is_write_locked());

    // TODO: change this for monetized version by fetching from a user table

    let exists = match decks::deck_exists(context.clone(), root_deck_id) {
        Ok(exists) => exists,
        Err(why) => {
            return Err(why);
        }
    };

    if !exists {
        let reason = RawAPIError::BadInput("set_root_deck", "requires deck with given root_deck_id to exist.");
        return Err(reason);
    }

    let encoded = format!("{}", root_deck_id);

    match configs::set_config(context.clone(), configs::CONFIG_ROOT_DECK_ID_KEY.to_string(), encoded) {
        Ok(_) => {
            // config successful
        },
        Err(why) => {
            return Err(why);
        }
    }

    return Ok(root_deck_id);
}

#[inline]
pub fn get_root_deck(context: Rc<RefCell<Context>>) -> Result<Option<DeckID>, RawAPIError> {

    assert!(context.borrow().is_read_locked());

    // TODO: change this for monetized version by fetching from a user table

    match configs::get_config(context.clone(), configs::CONFIG_ROOT_DECK_ID_KEY.to_string()) {
        Ok(config) => {

            match config {
                None => {
                    return Ok(None);
                },
                Some(config) => {

                    let deck_id = config.value;

                    match deck_id.parse::<DeckID>() {
                        Ok(deck_id) => {
                            return Ok(Some(deck_id));
                        }
                        Err(_) => {
                            return Ok(None);
                        }
                    }
                }
            }

        },
        Err(why) => {
            return Err(why);
        }
    }

}

#[inline]
pub fn is_root_deck(context: Rc<RefCell<Context>>, deck_id: DeckID) -> Result<bool, RawAPIError> {

    let _guard = context::read_lock(context.clone());

    match get_root_deck(context.clone()) {
        Ok(maybe_root_deck_id) => {
            let result = match maybe_root_deck_id {
                None => false,
                Some(root_deck_id) => root_deck_id == deck_id
            };

            return Ok(result);
        },
        Err(why) => {
            return Err(why);
        }
    }

}
