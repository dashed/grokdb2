/* rust lib imports */

use std::cell::RefCell;
use std::rc::Rc;

/* local imports */

use context::Context;
use errors::RawAPIError;
use types::{ItemCount, CardID};

/* ////////////////////////////////////////////////////////////////////////// */

type Percent = f64;

// limits cards to: active, inactive, or either/any
#[derive(Serialize, Deserialize)]
pub enum ActiveSelection {
    Active,
    Inactive,
    All
}

#[derive(Serialize, Deserialize)]
pub enum SubSelection {
    NewCards,
    LeastRecentlyReviewed,
    LastReviewedForAtLeast
}

#[derive(Serialize, Deserialize)]
pub enum ChosenCard {
    // 0 < Percent <= 1
    RandomTop(Percent),
    Random
}

// struct containing the information on how a card was chosen for review
#[derive(Serialize, Deserialize)]
pub struct ReviewProcedureCache {

    // within a group of cards (e.g. deck or stash)

    // within a group of cards, choose active/inactive/any cards
    active_selection: ActiveSelection,

    // pick subselection
    sub_selection: SubSelection,

    // sort subselection in descending order according to its score

    // how the card was chose in the sub selection
    card_chosen_by: ChosenCard
}



pub trait Reviewable {

    // faster version than number_of_cards_for_review
    fn have_cards_for_review(&self, context: Rc<RefCell<Context>>) -> Result<bool, RawAPIError>;

    // fn number_of_cards_for_review(&self, context: Rc<RefCell<Context>>) -> Result<ItemCount, RawAPIError>;

    /* caching */
    // fn cache_card(&self, card_id: i64) -> Result<(), QueryError>;
    fn get_cached_card(&self, context: Rc<RefCell<Context>>) -> Result<Option<CardID>, RawAPIError>;
    fn remove_cache(&self, context: Rc<RefCell<Context>>) -> Result<(), RawAPIError>;

    // // remove any cached entry by card id, regardless of sub-selection,
    // // of container type (e.g. Decks or Stash)
    // fn remove_cached_card(&self, card_id: i64) -> Result<(), QueryError>;

    // /* new cards */

    // fn has_new_cards(&self) -> Result<bool, QueryError>;

    // fn number_of_new_cards(&self) -> Result<i64, QueryError>;

    // // returns card id
    // fn get_new_card(&self, index: i64) -> Result<i64, QueryError>;

    // /* Top scoring cards that are reviewed for more than N hours ago and have score of at least M */

    // fn has_reviewable_cards(&self, age_in_hours: i64, min_score: f64) -> Result<bool, QueryError>;

    // fn number_of_reviewable_cards(&self, age_in_hours: i64, min_score: f64) -> Result<i64, QueryError>;

    // // TODO: adapt using http://blog.ssokolow.com/archives/2009/12/23/sql-pagination-without-offset/
    // // returns card id
    // fn get_reviewable_card(&self, age_in_hours: i64, min_score: f64, index: i64) -> Result<i64, QueryError>;

    // /* Top N least recently reviewed cards and have score of at least M */

    // fn has_old_cards(&self, purgatory_size: i64, min_score: f64) -> Result<bool, QueryError>;

    // // - get cards reviewed sorted by age (desc)
    // // - get top purgatory_size cards
    // // - discards less than min_score
    // // - sort by score (desc) [optional; if false, cards are implicitly sorted by age]
    // fn number_of_old_cards(&self, purgatory_size: i64, min_score: f64, sort_by_score: bool) -> Result<i64, QueryError>;

    // // returns card id
    // // - get cards reviewed sorted by age (desc)
    // // - get top purgatory_size cards
    // // - discards less than min_score
    // // - sort by score (desc) [optional; if false, cards are implicitly sorted by age]
    // fn get_old_card(&self, purgatory_size: i64, min_score: f64, index: i64, sort_by_score: bool) -> Result<i64, QueryError>;
}

// pub fn get_review_card<T>(selection: &T, context: Rc<RefCell<Context>>)
// -> Result<Option<CardID, ReviewProcedureCache>, RawAPIError>
//     where T: Reviewable {

//     match selection.get_cached_card() {
//         Err(why) => {
//             return Err(why);
//         },
//         Ok(Some(card_id)) => {
//             return Ok(Some(card_id));
//         }
//         Ok(None) => {
//             // no cached card for review
//         }
//     }

//     // ensure there are cards to review
//     match selection.have_cards_for_review(context.clone()) {
//         Err(why) => {
//             return Err(why);
//         },
//         Ok(false) => {
//             return Ok(None);
//         },
//         Ok(true) => {
//             // has cards to review
//         }
//     }


//     let review_procedure = ReviewProcedureCache {
//         active_selection: ActiveSelection::Active,
//         sub_selection:
//     };

// }
