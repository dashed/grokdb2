/* rust lib imports */

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::ops::DerefMut;

/* 3rd-party imports */

use random_wheel::RandomWheel;
use rand::{Rng, SeedableRng};
use rand::distributions::{Exp, IndependentSample};
use pcg::PcgRng;
use rusqlite::Connection;
use rusqlite::types::ToSql;

/* local imports */

use context::{self, Context};
use errors::RawAPIError;
use types::{ItemCount, CardID, DeckID, Offset, Minutes};
use api::cards::{self, Card};
use api::decks::{self, Deck};
use route::{AppRoute, DeckRoute};
use components::{generate_post_to};

/* ////////////////////////////////////////////////////////////////////////// */

lazy_static! {
    static ref RAND_GENERATOR: Arc<Mutex<PcgRng>> = {
        use rand::{self, Rng, SeedableRng};
        use pcg::PcgRng;
        use std::sync::{Arc, Mutex};

        // generate seed for pcg
        let mut rng = rand::thread_rng();
        let s = rng.gen_iter::<u64>().take(2).collect::<Vec<u64>>();

        // init pcg generator
        let pcg = PcgRng::from_seed([s[0], s[1]]);

        Arc::new(Mutex::new(pcg))

    };

}

static TOP_N_PERCENT: f64 = 0.5;
static DEFAULT_TIME_TILL_AVAILABLE_FOR_REVIEW: Minutes = 180; // 180 minutes = 3 hours
static DEFAULT_CARDS_TILL_AVAILABLE_FOR_REVIEW: ItemCount = 1;


type Percent = f64;

// limits cards to: active, inactive, or either/any
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActiveSelection {
    Active,
    Inactive,
    All
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SubSelection {
    NewCards,
    LeastRecentlyReviewed,
    ReadyForReview
}

pub type Probability = f64;

#[derive(Serialize, Deserialize, Clone, Debug)]
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

    fn gen_wheel(&self) -> RandomWheel<Probability, SubSelection, PcgRng> {

        // generate seed for pcg
        let mut guard = (*RAND_GENERATOR).lock().unwrap();
        let pcg: &mut PcgRng = guard.deref_mut();

        let s = pcg.gen_iter::<u64>().take(2).collect::<Vec<u64>>();

        // init pcg generator
        let mut pcg = PcgRng::from_seed([s[0], s[1]]);

        let mut rw: RandomWheel<Probability, SubSelection, PcgRng> = RandomWheel::new(pcg);

        rw.push(self.new_cards, SubSelection::NewCards);
        rw.push(self.ready_for_review, SubSelection::ReadyForReview);
        rw.push(self.least_recently_reviewed, SubSelection::LeastRecentlyReviewed);

        return rw;
    }
}

impl Default for SubSelectionProbabilities {
    fn default() -> Self {
        SubSelectionProbabilities {
            new_cards: 30.0,
            least_recently_reviewed: 25.0,
            ready_for_review: 45.0
        }
    }
}

// TODO: needed?
// #[derive(Serialize, Deserialize)]
// pub enum ChosenCard {
//     // 0 < Percent <= 1
//     RandomTop(Percent),
//     Random
// }

// struct containing the information on how a card was chosen for review
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CachedReviewProcedure {

    // within a group of cards (e.g. deck or stash)

    // within a group of cards, choose active/inactive/any cards
    active_selection: ActiveSelection,

    // pick subselection
    sub_selection: SubSelection,
    sub_selection_prob: SubSelectionProbabilities,

    // sort subselection in descending order according to its score

    // TODO: needed?
    // how the card was chose in the sub selection
    // card_chosen_by: ChosenCard
}

#[derive(Deserialize, Clone)]
pub enum ReviewAction {
    Right,
    Wrong,
    Forgot,
    Skip,
    Reset
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawDeckReviewRequest {
    card_id: CardID,
    review_action: ReviewAction,

    time_till_available_for_review: Option<Minutes>,
    cards_till_available_for_review: Option<ItemCount>,
}

impl RawDeckReviewRequest {

    pub fn normalize(&self) -> CardReviewRequest {

        let request = RawCardReviewRequest {
            review_action: self.review_action.clone(),

            time_till_available_for_review: self.time_till_available_for_review,
            cards_till_available_for_review: self.cards_till_available_for_review
        };

        request.normalize(self.card_id)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawCardReviewRequest {
    review_action: ReviewAction,

    time_till_available_for_review: Option<Minutes>,
    cards_till_available_for_review: Option<ItemCount>,
}

impl RawCardReviewRequest {
    pub fn normalize(&self, card_id: CardID) -> CardReviewRequest {

        let time_till_available_for_review = match self.time_till_available_for_review {
            None => DEFAULT_TIME_TILL_AVAILABLE_FOR_REVIEW,
            Some(val) => val
        };

        let cards_till_available_for_review = match self.cards_till_available_for_review {
            None => DEFAULT_CARDS_TILL_AVAILABLE_FOR_REVIEW,
            Some(val) => val
        };

        CardReviewRequest {
            card_id: card_id,
            review_action: self.review_action.clone(),
            time_till_available_for_review: time_till_available_for_review,
            cards_till_available_for_review: cards_till_available_for_review
        }

    }
}

pub struct CardReviewRequest {
    pub card_id: CardID,
    review_action: ReviewAction,

    time_till_available_for_review: Minutes,
    cards_till_available_for_review: ItemCount,
}

impl CardReviewRequest {

    pub fn commit(&self, context: Rc<RefCell<Context>>) -> Result<(), RawAPIError> {

        let _guard = context::write_lock(context.clone());

        match remove_cache_in_all_sources(context.clone(), self.card_id) {
            Ok(_) => {},
            Err(why) => {
                return Err(why);
            }
        }

        match self.review_card(context) {
            Ok(_) => {},
            Err(why) => {
                return Err(why);
            }
        }

        return Ok(());
    }

    fn review_card(
        &self,
        context: Rc<RefCell<Context>>,
        ) -> Result<(), RawAPIError> {

        assert!(context.borrow().is_write_locked());

        let (changelog,
            success,
            fail,
            times_reviewed,
            times_seen,
            seen_at,
            reviewed_at) = match self.review_action {
            ReviewAction::Right => {

                let changelog = "Answered card correctly.";
                let success = "success + 1";
                let fail = "fail"; // no change
                let times_reviewed = "times_reviewed + 1";
                let times_seen = "times_seen + 1";
                let seen_at = "strftime('%s', 'now')";
                let reviewed_at = "strftime('%s', 'now')";

                (changelog, success, fail, times_reviewed, times_seen, seen_at, reviewed_at)
            },
            ReviewAction::Wrong => {

                let changelog = "Answered card incorrectly.";
                let success = "success"; // no change
                let fail = "fail + 1";
                let times_reviewed = "times_reviewed + 1";
                let times_seen = "times_seen + 1";
                let seen_at = "strftime('%s', 'now')";
                let reviewed_at = "strftime('%s', 'now')";

                (changelog, success, fail, times_reviewed, times_seen, seen_at, reviewed_at)
            },
            ReviewAction::Forgot => {

                let changelog = "Forgotten answer to the card.";
                let success = "0";
                let fail = "1";
                let times_reviewed = "times_reviewed + 1";
                let times_seen = "times_seen + 1";
                let seen_at = "strftime('%s', 'now')";
                let reviewed_at = "strftime('%s', 'now')";

                (changelog, success, fail, times_reviewed, times_seen, seen_at, reviewed_at)
            },
            ReviewAction::Skip => {

                let changelog = "Skipped card.";
                let success = "success"; // no change
                let fail = "fail"; // no change
                let times_reviewed = "times_reviewed"; // no change
                let times_seen = "times_seen + 1";
                let seen_at = "strftime('%s', 'now')";
                let reviewed_at = "reviewed_at"; // no change

                (changelog, success, fail, times_reviewed, times_seen, seen_at, reviewed_at)
            },
            ReviewAction::Reset => {

                let changelog = "Reset score.";
                let success = "0";
                let fail = "0";
                let times_reviewed = "times_reviewed"; // no change
                let times_seen = "times_seen"; // no change
                let seen_at = "seen_at)"; // no change
                let reviewed_at = "reviewed_at"; // no change

                (changelog, success, fail, times_reviewed, times_seen, seen_at, reviewed_at)
            }
        };

        // NOTE: convert from minutes to seconds
        let review_after = self.time_till_available_for_review * 60;

        let query = format!(indoc!("
            UPDATE
                CardsScore
            SET
                changelog = :changelog,
                success = {success},
                fail = {fail},
                times_reviewed = {times_reviewed},
                times_seen = {times_seen},
                seen_at = {seen_at},
                reviewed_at = {reviewed_at},
                review_after = {review_after}
            WHERE card_id = {card_id};
        "),
        success = success,
        fail = fail,
        times_reviewed = times_reviewed,
        times_seen = times_seen,
        seen_at = seen_at,
        reviewed_at = reviewed_at,
        review_after = review_after,
        card_id = self.card_id);

        let params: &[(&str, &ToSql)] = &[
            (":changelog", &changelog),
        ];

        let context = context.borrow();
        db_write_lock!(db_conn; context.database());
        let db_conn: &Connection = db_conn;

        match db_conn.execute_named(&query, params) {
            Err(sqlite_error) => {
                return Err(RawAPIError::SQLError(sqlite_error, query));
            }
            _ => {
                /* query sucessfully executed */
            }
        }

        return Ok(());
    }

}

#[derive(Serialize)]
pub struct CardForReview {
    post_to: String,
    card: Card,
    card_meta: Option<CachedReviewProcedure>
}

#[derive(Serialize)]
pub struct ReviewResponse {
    has_card_for_review: bool,
    card_for_review: Option<CardForReview>
}

impl ReviewResponse {
    pub fn new(context: Rc<RefCell<Context>>, deck: Deck) -> Result<Self, RawAPIError> {

        let _guard = context::write_lock(context.clone());

        let result = match get_review_card(context.clone(), &deck) {
            Ok(result) => result,
            Err(why) => {
                return Err(why);
            }
        };

        // generat JSON response

        let (card, cached_review_procedure) = match result {
            None => (None, None),
            Some((card_id, cached_review_procedure)) => {

                match cards::get_card(context, card_id) {
                    Ok(card) => (Some(card), cached_review_procedure),
                    Err(why) => {
                        return Err(why);
                    }
                }
            }
        };

        let has_card_for_review = card.is_some();

        let card_for_review = if let Some(card) = card {
            let card_for_review = CardForReview {
                post_to: generate_post_to(&AppRoute::Deck(deck.id, DeckRoute::Review(None))),
                card: card,
                card_meta: cached_review_procedure
            };

            Some(card_for_review)
        } else {
            None
        };

        let review_response = ReviewResponse {
            has_card_for_review: has_card_for_review,
            card_for_review: card_for_review

        };

        return Ok(review_response);
    }
}

pub trait Reviewable {

    /* caching */

    fn get_cached_card(&self, context: Rc<RefCell<Context>>)
        -> Result<Option<(CardID, Option<CachedReviewProcedure>)>, RawAPIError>;
    fn set_cache_card(&self,
        context: Rc<RefCell<Context>>,
        card_id: CardID,
        cached_review_procedure: CachedReviewProcedure)
        -> Result<(), RawAPIError>;

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
        active_selection: &ActiveSelection,
        top_n: ItemCount,
        card_idx: Offset) -> Result<CardID, RawAPIError>;

    /* cards for review */

    fn have_cards_for_review(&self,
        context: Rc<RefCell<Context>>,
        active_selection: &ActiveSelection) -> Result<bool, RawAPIError>;

    fn deck_num_of_cards_for_review(&self,
        context: Rc<RefCell<Context>>,
        active_selection: &ActiveSelection) -> Result<ItemCount, RawAPIError>;
}

/* helpers */

#[inline]
// remove card review entry by card_id in all cache sources
pub fn remove_cache_in_all_sources(context: Rc<RefCell<Context>>, card_id: CardID) -> Result<(), RawAPIError> {

    // TODO: move this into separate function when adding stash cache

    assert!(context.borrow().is_write_locked());

    let query = format!(indoc!("
        DELETE FROM
            CachedDeckReview
        WHERE
            card_id = {card_id};
    "), card_id = card_id);

    let context = context.borrow();
    db_read_lock!(db_conn; context.database());
    let db_conn: &Connection = db_conn;

    match db_conn.execute_named(&query, &[]) {
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        },
        _ => {/* query sucessfully executed */},
    }

    return Ok(());

}


#[inline]
pub fn get_review_card<T>(context: Rc<RefCell<Context>>, selection: &T)
-> Result<Option<(CardID, Option<CachedReviewProcedure>)>, RawAPIError>
    where T: Reviewable {

    // NOTE: this is necessary since get_cached_card also needs write lock
    let _guard = context::write_lock(context.clone());

    match selection.get_cached_card(context.clone()) {
        Err(why) => {
            // TODO: internal error logging
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
        // TODO: internal error logging
        let err = RawAPIError::BadInput("get_review_card", "Invalid sub-selection probabilities.");
        return Err(err);
    }

    // ensure there are cards to review
    match selection.have_cards_for_review(context.clone(), &active_selection) {
        Err(why) => {
            // TODO: internal error logging
            return Err(why);
        },
        Ok(false) => {
            return Ok(None);
        },
        Ok(true) => {
            // has cards to review
        }
    }

    let sub_selection = match choose_subselection(
        context.clone(),
        selection,
        &active_selection,
        probabilities.clone()) {
        Err(why) => {
            // TODO: internal error logging
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
                    // TODO: internal error logging
                    return Err(why);
                }
            }
        },
        SubSelection::ReadyForReview => {
            match get_card_ready_for_review(
                context.clone(),
                selection,
                &active_selection) {
                Ok(card_id) => card_id,
                Err(why) => {
                    // TODO: internal error logging
                    return Err(why);
                }
            }
        },
        SubSelection::LeastRecentlyReviewed => {
            match get_least_recently_reviewed_card(
                context.clone(),
                selection,
                &active_selection) {
                Ok(card_id) => card_id,
                Err(why) => {
                    // TODO: internal error logging
                    return Err(why);
                }
            }
        },
    };

    let cached_review_procedure = CachedReviewProcedure {
        active_selection: active_selection,
        sub_selection: sub_selection,
        sub_selection_prob: probabilities
    };

    match selection.set_cache_card(context.clone(), card_id, cached_review_procedure.clone()) {
        Ok(_) => {},
        Err(why) => {
            // TODO: internal error logging
            return Err(why);
        }
    }

    return Ok(Some((card_id, Some(cached_review_procedure))));

}

#[inline]
fn choose_subselection<T>(
    context: Rc<RefCell<Context>>,
    selection: &T,
    active_selection: &ActiveSelection,
    probabilities: SubSelectionProbabilities)
-> Result<SubSelection, RawAPIError> where T: Reviewable {

    // invariant: selection have cards for review

    let wheel = probabilities.gen_wheel();

    // TODO: dev only
    assert!(wheel.len() > 0);

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

#[inline]
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

    // TODO: dev mode
    assert!(num_of_cards > 0);

    // TODO: top N percent of least recently created

    // Generate a random value in the range [0, num_of_cards)
    let card_idx = gen_card_select(num_of_cards);

    match selection.get_new_card_for_review(context, active_selection, card_idx) {
        Ok(card_id) => {
            return Ok(card_id);
        },
        Err(why) => {
            return Err(why);
        }
    }
}

#[inline]
fn get_card_ready_for_review<T>(
    context: Rc<RefCell<Context>>,
    selection: &T,
    active_selection: &ActiveSelection) -> Result<CardID, RawAPIError>
    where T: Reviewable {

    let num_of_cards = match selection.num_of_cards_ready_for_review(
        context.clone(), active_selection) {
        Ok(num_of_cards) => num_of_cards,
        Err(why) => {
            return Err(why);
        }
    };

    // TODO: dev mode
    assert!(num_of_cards > 0);

    let upper_bound = (TOP_N_PERCENT * (num_of_cards as f64)).ceil() as ItemCount;
    // TODO: dev mode
    assert!(upper_bound > 0);

    // Generate a random value in the range [0, num_of_cards)
    let card_idx = gen_card_select(upper_bound);

    match selection.get_card_ready_for_review(context, active_selection, card_idx) {
        Ok(card_id) => {
            return Ok(card_id);
        },
        Err(why) => {
            return Err(why);
        }
    }

}

#[inline]
fn get_least_recently_reviewed_card<T>(
    context: Rc<RefCell<Context>>,
    selection: &T,
    active_selection: &ActiveSelection) -> Result<CardID, RawAPIError>
    where T: Reviewable {

    let num_of_cards = match selection.deck_num_of_cards_for_review(
        context.clone(), active_selection) {
        Ok(num_of_cards) => num_of_cards,
        Err(why) => {
            return Err(why);
        }
    };

    // TODO: dev mode
    assert!(num_of_cards > 0);

    let upper_bound = (TOP_N_PERCENT * (num_of_cards as f64)).ceil() as ItemCount;
    // TODO: dev mode
    assert!(upper_bound > 0);

    // Generate a random value in the range [0, num_of_cards)
    let card_idx = gen_card_select(upper_bound);

    match selection.get_least_recently_reviewed_card(context, active_selection, upper_bound, card_idx) {
        Ok(card_id) => {
            return Ok(card_id);
        },
        Err(why) => {
            return Err(why);
        }
    }

}

// generate number from
#[inline]
fn gen_card_select(upper_bound: ItemCount) -> Offset {

    // TODO: dev mode
    assert!(upper_bound > 0);

    let exp = Exp::new(5.0);

    let mut guard = (*RAND_GENERATOR).lock().unwrap();
    let pcg: &mut PcgRng = guard.deref_mut();

    let pin: f64 = exp.ind_sample(pcg);

    if pin.is_infinite() {
        return upper_bound - 1;
    }

    // normalize to [0, 100)
    let pin = if pin >= 100.0 {
        pin % 100.0
    } else {
        pin
    };

    let card_idx = (pin * (upper_bound as f64)).floor() as Offset;

    // TODO: dev mode
    assert!(0 <= card_idx);
    assert!(card_idx < upper_bound);

    return card_idx;

}
