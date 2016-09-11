/* rust lib imports */

use std::cell::RefCell;
use std::rc::Rc;

/* local imports */

use context::{self, Context};
use types::{DeckID};
use errors::RawAPIError;

/* ////////////////////////////////////////////////////////////////////////// */

#[inline]
pub fn get_root_deck(context: Rc<RefCell<Context>>) -> Result<DeckID, RawAPIError> {

    assert!(context.borrow().is_read_locked());

    // TODO: change this for monetized version by fetching from a user table

    let context = context.borrow();

    return Ok(context.root_deck_id);
}

#[inline]
pub fn is_root_deck(context: Rc<RefCell<Context>>, deck_id: DeckID) -> Result<bool, RawAPIError> {

    let _guard = context::read_lock(context.clone());

    match get_root_deck(context.clone()) {
        Ok(root_deck_id) => {
            return Ok(root_deck_id == deck_id);
        },
        Err(why) => {
            return Err(why);
        }
    }

}
