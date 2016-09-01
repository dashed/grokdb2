/* rust lib imports */

use std::cell::RefCell;
use std::rc::Rc;

/* 3rd-party imports */

use random_wheel::RandomWheel;

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
    ReadyForReview
}

pub type Probability = f64;

#[derive(Serialize, Deserialize, Clone)]
pub struct SubSelectionProbabilities {
    new_cards: Probability,
    least_recently_reviewed: Probability,
    ready_for_review: Probability
}

impl SubSelectionProbabilities {
    fn valid(&self) -> bool {
        return self.new_cards > 0.0 &&
            self.least_recently_reviewed > 0.0 &&
            self.ready_for_review > 0.0;
    }

    fn gen_wheel(&self) -> RandomWheel<Probability, SubSelection> {

        let mut rw: RandomWheel<Probability, SubSelection> = RandomWheel::new();

        rw.push(self.new_cards, SubSelection::NewCards);
        rw.push(self.least_recently_reviewed, SubSelection::LeastRecentlyReviewed);
        rw.push(self.ready_for_review, SubSelection::ReadyForReview);

        return rw;
    }
}

impl Default for SubSelectionProbabilities {
    fn default() -> Self {
        SubSelectionProbabilities {
            new_cards: 30.0,
            least_recently_reviewed: 30.0,
            ready_for_review: 40.0
        }
    }
}

// #[derive(Serialize, Deserialize)]
// pub enum ChosenCard {
//     // 0 < Percent <= 1
//     RandomTop(Percent),
//     Random
// }

// struct containing the information on how a card was chosen for review
#[derive(Serialize, Deserialize)]
pub struct CachedReviewProcedure {

    // within a group of cards (e.g. deck or stash)

    // within a group of cards, choose active/inactive/any cards
    active_selection: ActiveSelection,

    // pick subselection
    sub_selection: SubSelection,
    sub_selection_prob: SubSelectionProbabilities,

    // sort subselection in descending order according to its score

    // how the card was chose in the sub selection
    // card_chosen_by: ChosenCard
}


pub trait Reviewable {

    // faster version than number_of_cards_for_review
    fn have_cards_for_review(&self,
        context: Rc<RefCell<Context>>,
        active_selection: &ActiveSelection) -> Result<bool, RawAPIError>;

    // fn number_of_cards_for_review(&self, context: Rc<RefCell<Context>>) -> Result<ItemCount, RawAPIError>;

    /* caching */
    // fn cache_card(&self, card_id: i64) -> Result<(), QueryError>;
    fn get_cached_card(&self, context: Rc<RefCell<Context>>)
        -> Result<Option<(CardID, Option<CachedReviewProcedure>)>, RawAPIError>;

    fn remove_cache(&self, context: Rc<RefCell<Context>>) -> Result<(), RawAPIError>;

    // // remove any cached entry by card id, regardless of sub-selection,
    // // of container type (e.g. Decks or Stash)
    // fn remove_cached_card(&self, card_id: i64) -> Result<(), QueryError>;

    /* new cards */

    fn have_new_cards(&self,
        context: Rc<RefCell<Context>>,
        active_selection: &ActiveSelection) -> Result<bool, RawAPIError>;

    // fn number_of_new_cards(&self) -> Result<i64, QueryError>;

    // // returns card id
    // fn get_new_card(&self, index: i64) -> Result<i64, QueryError>;

    // /* Top scoring cards that are reviewed for more than N hours ago and have score of at least M */

    // fn has_reviewable_cards(&self, age_in_hours: i64, min_score: f64) -> Result<bool, QueryError>;

    // fn number_of_reviewable_cards(&self, age_in_hours: i64, min_score: f64) -> Result<i64, QueryError>;

    // // TODO: adapt using http://blog.ssokolow.com/archives/2009/12/23/sql-pagination-without-offset/
    // // returns card id
    // fn get_reviewable_card(&self, age_in_hours: i64, min_score: f64, index: i64) -> Result<i64, QueryError>;

    /* Cards ready for review */

    fn have_cards_ready_for_review(&self,
        context: Rc<RefCell<Context>>,
        active_selection: &ActiveSelection) -> Result<bool, RawAPIError>;

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



pub fn get_review_card<T>(context: Rc<RefCell<Context>>, selection: &T)
-> Result<Option<(CardID, Option<CachedReviewProcedure>)>, RawAPIError>
    where T: Reviewable {

    match selection.get_cached_card(context.clone()) {
        Err(why) => {
            return Err(why);
        },
        Ok(Some((card_id, cached_review_procedure))) => {
            return Ok(Some((card_id, cached_review_procedure)));
        }
        Ok(None) => {
            // no cached card for review
        }
    }

    // TODO: be able to toggle selection for active/inactive cards from client-side
    let active_selection = ActiveSelection::Active;
    let probabilities: SubSelectionProbabilities = Default::default();

    if !probabilities.valid() {
        let err = RawAPIError::BadInput("get_review_card", "Invalid sub-selection probabilities.");
        return Err(err);
    }

    // ensure there are cards to review
    match selection.have_cards_for_review(context.clone(), &active_selection) {
        Err(why) => {
            return Err(why);
        },
        Ok(false) => {
            return Ok(None);
        },
        Ok(true) => {
            // has cards to review
        }
    }

    //
    let sub_selection = match choose_subselection(context.clone(), selection, &active_selection, probabilities.clone()) {
        Err(why) => {
            return Err(why);
        },
        Ok(sub_selection) => sub_selection
    };

    // TODO: complete
    let review_procedure = CachedReviewProcedure {
        active_selection: active_selection,
        sub_selection: sub_selection,
        sub_selection_prob: probabilities
    };

    let card_id = 1;

    return Ok(Some((card_id, Some(review_procedure))));

}

fn choose_subselection<T>(
    context: Rc<RefCell<Context>>,
    selection: &T,
    active_selection: &ActiveSelection,
    probabilities: SubSelectionProbabilities)
-> Result<SubSelection, RawAPIError> where T: Reviewable {

    // invariant: selection have cards for review

    let wheel = probabilities.gen_wheel();

    // TODO: dev only
    assert!(wheel.len() == 3);

    for (_, subselection) in wheel {

        match subselection {
            SubSelection::NewCards => {

                match selection.have_new_cards(context.clone(), active_selection) {
                    Ok(result) => {
                        if result {
                            return Ok(SubSelection::NewCards);
                        }
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }

            },
            SubSelection::LeastRecentlyReviewed => {
                return Ok(SubSelection::LeastRecentlyReviewed);
            },
            SubSelection::ReadyForReview => {
                match selection.have_cards_ready_for_review(context.clone(), active_selection) {
                    Ok(result) => {
                        if result {
                            return Ok(SubSelection::ReadyForReview);
                        }
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
        }
    }

    // NOTE: this is unreachable

    return Ok(SubSelection::LeastRecentlyReviewed);

}
