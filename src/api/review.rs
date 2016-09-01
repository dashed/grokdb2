/* rust lib imports */

use std::cell::RefCell;
use std::rc::Rc;

/* 3rd-party imports */

use random_wheel::RandomWheel;
use rand::{thread_rng, Rng};

/* local imports */

use context::{self, Context};
use errors::RawAPIError;
use types::{ItemCount, CardID, Offset};

/* ////////////////////////////////////////////////////////////////////////// */

static TOP_N_PERCENT: f64 = 0.5;

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

    fn num_of_new_cards_for_review(&self,
        context: Rc<RefCell<Context>>,
        active_selection: &ActiveSelection) -> Result<ItemCount, RawAPIError>;

    fn get_new_card_for_review(&self,
        context: Rc<RefCell<Context>>,
        active_selection: &ActiveSelection,
        card_idx: Offset) -> Result<CardID, RawAPIError>;

    /* cards ready for review */

    fn have_cards_ready_for_review(&self,
        context: Rc<RefCell<Context>>,
        active_selection: &ActiveSelection) -> Result<bool, RawAPIError>;

    fn num_of_cards_ready_for_review(&self,
        context: Rc<RefCell<Context>>,
        active_selection: &ActiveSelection) -> Result<ItemCount, RawAPIError>;

    fn get_card_ready_for_review(&self,
        context: Rc<RefCell<Context>>,
        active_selection: &ActiveSelection,
        card_idx: Offset) -> Result<CardID, RawAPIError>;

    /* least recently reviewed */

    fn get_least_recently_reviewed_card(&self,
        context: Rc<RefCell<Context>>,
        active_selection: &ActiveSelection) -> Result<CardID, RawAPIError>;
    // TODO: complete
}

/* helpers */

pub fn get_review_card<T>(context: Rc<RefCell<Context>>, selection: &T)
-> Result<Option<(CardID, Option<CachedReviewProcedure>)>, RawAPIError>
    where T: Reviewable {

    // NOTE: this is necessary since get_cached_card also needs write lock
    let _guard = context::write_lock(context.clone());

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

    let card_id = match sub_selection {
        SubSelection::NewCards => {
            match get_new_card(
                context.clone(),
                selection,
                &active_selection) {
                Ok(card_id) => card_id,
                Err(why) => {
                    return Err(why);
                }
            }
        },
        SubSelection::LeastRecentlyReviewed => {
            match get_card_ready_for_review(
                context.clone(),
                selection,
                &active_selection) {
                Ok(card_id) => card_id,
                Err(why) => {
                    return Err(why);
                }
            }
        },
        SubSelection::ReadyForReview => {
            // TODO: complete
            1
        }
    };

    // TODO: cache

    // TODO: complete
    let review_procedure = CachedReviewProcedure {
        active_selection: active_selection,
        sub_selection: sub_selection,
        sub_selection_prob: probabilities
    };

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

fn get_new_card<T>(
    context: Rc<RefCell<Context>>,
    selection: &T,
    active_selection: &ActiveSelection) -> Result<CardID, RawAPIError>
    where T: Reviewable {

    let num_of_cards = match selection.num_of_new_cards_for_review(
        context.clone(), active_selection) {
        Ok(num_of_cards) => num_of_cards,
        Err(why) => {
            return Err(why);
        }
    };

    // Generate a random value in the range [0, num_of_cards)
    let card_idx = thread_rng().gen_range(0, num_of_cards);

    match selection.get_new_card_for_review(context, active_selection, card_idx) {
        Ok(card_id) => {
            return Ok(card_id);
        },
        Err(why) => {
            return Err(why);
        }
    }
}


fn get_card_ready_for_review<T>(
    context: Rc<RefCell<Context>>,
    selection: &T,
    active_selection: &ActiveSelection) -> Result<CardID, RawAPIError>
    where T: Reviewable {

    let num_of_cards = match selection.num_of_cards_ready_for_review(
        context.clone(),active_selection) {
        Ok(num_of_cards) => num_of_cards,
        Err(why) => {
            return Err(why);
        }
    };

    let upper_bound = (TOP_N_PERCENT * (num_of_cards as f64)).ceil() as ItemCount;
    // TODO: dev mode
    assert!(upper_bound > 0);
    let upper_bound = upper_bound + 1;

    // Generate a random value in the range [0, num_of_cards)
    let card_idx = thread_rng().gen_range(0, upper_bound);

    match selection.get_card_ready_for_review(context, active_selection, card_idx) {
        Ok(card_id) => {
            return Ok(card_id);
        },
        Err(why) => {
            return Err(why);
        }
    }

}