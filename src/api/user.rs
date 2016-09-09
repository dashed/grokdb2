/* rust lib imports */

use std::cell::RefCell;
use std::rc::Rc;

/* local imports */

use context::Context;
use types::{DeckID};

/* ////////////////////////////////////////////////////////////////////////// */

#[inline]
pub fn get_root_deck(context: Rc<RefCell<Context>>) -> DeckID {

    assert!(context.borrow().is_read_locked());

    // TODO: change this for monetized version by fetching from a user table

    let context = context.borrow();

    return context.root_deck_id;
}
