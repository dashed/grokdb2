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
use chrono::naive::datetime::NaiveDateTime;

/* local imports */

use context::{self, Context};
use errors::RawAPIError;
use types::{ItemCount, CardID, DeckID, Offset, Minutes, ReviewCount, UnixTimestamp};
use api::cards::{self, Card};
use api::decks::{Deck};
use api::user;
use route::{AppRoute, DeckRoute};
use components::{generate_post_to, view_route_to_link};
use timestamp;

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

static TOP_N_PERCENT: f64 = 0.375; // 0.25 + 0.25 / 2
// static DEFAULT_TIME_TILL_AVAILABLE_FOR_REVIEW: Minutes = 180; // 180 minutes = 3 hours
// static DEFAULT_CARDS_TILL_AVAILABLE_FOR_REVIEW: ItemCount = 1;


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
        let pcg = PcgRng::from_seed([s[0], s[1]]);

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
    Easy,
    Right,
    Hard,
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
    cards_till_available_for_review: Option<ItemCount>
}

impl RawCardReviewRequest {
    pub fn normalize(&self, card_id: CardID) -> CardReviewRequest {

        CardReviewRequest {
            card_id: card_id,
            review_action: self.review_action.clone(),
            time_till_available_for_review: self.time_till_available_for_review.clone(),
            cards_till_available_for_review: self.cards_till_available_for_review.clone()
        }

    }
}

pub struct CardReviewRequest {
    pub card_id: CardID,
    review_action: ReviewAction,

    time_till_available_for_review: Option<Minutes>,
    cards_till_available_for_review: Option<ItemCount>,
}

impl CardReviewRequest {

    pub fn commit_on_deck(&self, context: Rc<RefCell<Context>>, deck_id: DeckID) -> Result<(), RawAPIError> {

        // a deck is only reviewed only on certain specific review actions
        match self.review_action {
            ReviewAction::Right |
            ReviewAction::Wrong |
            ReviewAction::Forgot => {

                let _guard = context::write_lock(context.clone());

                // TODO: group this into a transaction

                let query = format!(indoc!("
                    UPDATE Decks
                    SET
                    reviewed_at = strftime('%s', 'now')
                    WHERE
                        deck_id = {deck_id};
                "), deck_id = deck_id);

                let context = context.borrow();
                db_write_lock!(db_conn; context.database());
                let db_conn: &Connection = db_conn;

                match db_conn.execute_named(&query, &[]) {
                    Err(sqlite_error) => {
                        return Err(RawAPIError::SQLError(sqlite_error, query));
                    }
                    _ => {
                        /* query sucessfully executed */
                    }
                }

            },
            _ => {}
        }

        return self.commit_on_card(context.clone());

    }

    pub fn commit_on_card(&self, context: Rc<RefCell<Context>>) -> Result<(), RawAPIError> {

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

        let review_count = match try!(user::get_review_count(context.clone())) {
            None => 0,
            Some(review_count) => review_count
        };

        let (changelog,
            success,
            fail,
            times_reviewed,
            times_seen,
            seen_at,
            reviewed_at,
            reviewed_at_count) = match self.review_action {
            ReviewAction::Easy => {

                let changelog = "Answered card easily.";
                let success = "success + 2";
                let fail = "fail"; // no change
                let times_reviewed = "times_reviewed + 1";
                let times_seen = "times_seen + 1";
                let seen_at = "strftime('%s', 'now')";
                let reviewed_at = "strftime('%s', 'now')";
                let reviewed_at_count = review_count + 1;

                (changelog, success, fail, times_reviewed, times_seen, seen_at, reviewed_at, reviewed_at_count)
            },
            ReviewAction::Right => {

                let changelog = "Answered card correctly.";
                let success = "success + 1";
                let fail = "fail"; // no change
                let times_reviewed = "times_reviewed + 1";
                let times_seen = "times_seen + 1";
                let seen_at = "strftime('%s', 'now')";
                let reviewed_at = "strftime('%s', 'now')";
                let reviewed_at_count = review_count + 1;

                (changelog, success, fail, times_reviewed, times_seen, seen_at, reviewed_at, reviewed_at_count)
            },
            ReviewAction::Hard => {

                let changelog = "Answered card, but found it hard.";
                let success = "success + 0.5";
                let fail = "fail"; // no change
                let times_reviewed = "times_reviewed + 1";
                let times_seen = "times_seen + 1";
                let seen_at = "strftime('%s', 'now')";
                let reviewed_at = "strftime('%s', 'now')";
                let reviewed_at_count = review_count + 1;

                (changelog, success, fail, times_reviewed, times_seen, seen_at, reviewed_at, reviewed_at_count)
            },
            ReviewAction::Wrong => {

                let changelog = "Answered card incorrectly.";
                let success = "success"; // no change
                let fail = "fail + 1";
                let times_reviewed = "times_reviewed + 1";
                let times_seen = "times_seen + 1";
                let seen_at = "strftime('%s', 'now')";
                let reviewed_at = "strftime('%s', 'now')";
                let reviewed_at_count = review_count + 1;

                (changelog, success, fail, times_reviewed, times_seen, seen_at, reviewed_at, reviewed_at_count)
            },
            ReviewAction::Forgot => {

                let changelog = "Forgotten answer to the card.";
                let success = "0";
                let fail = "1";
                let times_reviewed = "times_reviewed + 1";
                let times_seen = "times_seen + 1";
                let seen_at = "strftime('%s', 'now')";
                let reviewed_at = "strftime('%s', 'now')";
                let reviewed_at_count = review_count + 1;

                (changelog, success, fail, times_reviewed, times_seen, seen_at, reviewed_at, reviewed_at_count)
            },
            ReviewAction::Skip => {

                let changelog = "Skipped card.";
                let success = "success"; // no change
                let fail = "fail"; // no change
                let times_reviewed = "times_reviewed"; // no change
                let times_seen = "times_seen + 1";
                let seen_at = "strftime('%s', 'now')";
                let reviewed_at = "reviewed_at"; // no change
                let reviewed_at_count = review_count; // no change

                (changelog, success, fail, times_reviewed, times_seen, seen_at, reviewed_at, reviewed_at_count)
            },
            ReviewAction::Reset => {

                let changelog = "Reset score.";
                let success = "0";
                let fail = "0";
                let times_reviewed = "times_reviewed"; // no change
                let times_seen = "times_seen"; // no change
                let seen_at = "seen_at)"; // no change
                let reviewed_at = "reviewed_at"; // no change
                let reviewed_at_count = review_count; // no change

                (changelog, success, fail, times_reviewed, times_seen, seen_at, reviewed_at, reviewed_at_count)
            }
        };

        // update review count
        try!(user::set_review_count(context.clone(), reviewed_at_count));

        // NOTE: convert from minutes to seconds
        let review_after = match self.time_till_available_for_review {
            Some(time_till_available_for_review) => {
                // stringify numbers
                format!("{}", time_till_available_for_review * 60)
            },
            None => {
                // no-change
                "review_after".to_string()
            }
        };

        let cards_till_ready_for_review = match self.cards_till_available_for_review {
            Some(cards_till_ready_for_review) => {
                // stringify numbers
                format!("{}", cards_till_ready_for_review)
            },
            None => {
                // no-change
                "cards_till_ready_for_review".to_string()
            }
        };

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
                review_after = {review_after},
                reviewed_at_count = {reviewed_at_count},
                cards_till_ready_for_review = {cards_till_ready_for_review}
            WHERE card_id = {card_id};
        "),
        success = success,
        fail = fail,
        times_reviewed = times_reviewed,
        times_seen = times_seen,
        seen_at = seen_at,
        reviewed_at = reviewed_at,
        review_after = review_after,
        reviewed_at_count = reviewed_at_count,
        cards_till_ready_for_review = cards_till_ready_for_review,
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
    post_to: String, // TODO: doesn't seem to be useful; remove
    profile_url: String,
    card: Card,
    card_meta: Option<CachedReviewProcedure>,
    card_score: CardScore
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

                match cards::get_card(context.clone(), card_id) {
                    Ok(card) => (Some(card), cached_review_procedure),
                    Err(why) => {
                        return Err(why);
                    }
                }
            }
        };

        let has_card_for_review = card.is_some();

        let card_for_review = if let Some(card) = card {

            let card_score = match get_card_score(context.clone(), card.id) {
                Ok(card_score) => card_score,
                Err(why) => {
                    return Err(why);
                }
            };

            let card_for_review = CardForReview {
                post_to: generate_post_to(&AppRoute::Deck(deck.id, DeckRoute::Review(None))),
                profile_url: view_route_to_link(context,
                                    AppRoute::Card(card.id, Default::default())),
                card: card,
                card_meta: cached_review_procedure,
                card_score: card_score
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

#[derive(Debug, Serialize)]
pub struct CardScore {

    pub card_id: CardID,

    pub changelog: String,

    pub raw_score: f64, // NOTE: not in database table
    pub success: f64,
    pub fail: f64,

    pub times_reviewed: u64,
    pub times_seen: u64,

    pub seen_at: String,
    pub seen_at_actual: UnixTimestamp,

    pub reviewed_at: String,
    pub reviewed_at_actual: UnixTimestamp,

    pub review_after_normalized: u64,
    pub review_after_time_control: String,
    pub review_after: u64,
    pub ready_for_review_at: String,

    pub reviewed_at_count: ReviewCount,
    pub cards_till_ready_for_review: ReviewCount
}

impl CardScore {

    pub fn get_perf_score(&self) -> f64 {
        let perf_score = 1.0 - self.raw_score;
        return perf_score;
    }

    pub fn get_perf_score_string(&self) -> String {
        format!("{:.*}", 4, self.get_perf_score())
    }

    pub fn get_perf_score_percent_string(&self) -> String {
        format!("{:.*}", 2, self.get_perf_score() * 100.0)
    }

    pub fn get_max_perf_score_string(&self) -> String {
        format!("1")
    }

    pub fn was_reviewed(&self, context: Rc<RefCell<Context>>) -> bool {

        assert!(context.borrow().is_read_locked());

        let card = match cards::get_card(context, self.card_id) {
            Ok(card) => card,
            Err(_why) => {
                panic!("{:?}", _why);
            }
        };

        return card.created_at_actual != self.reviewed_at_actual;
    }

    pub fn was_picked_for_review(&self, context: Rc<RefCell<Context>>) -> bool {

        assert!(context.borrow().is_read_locked());

        let card = match cards::get_card(context, self.card_id) {
            Ok(card) => card,
            Err(_why) => {
                panic!("{:?}", _why);
            }
        };

        return card.created_at_actual != self.seen_at_actual;
    }
}

#[inline]
pub fn get_card_score(context: Rc<RefCell<Context>>, card_id: CardID) -> Result<CardScore, RawAPIError> {

    assert!(context.borrow().is_read_locked());

    let query = format!(indoc!("
        SELECT
            changelog,

            success,
            fail,

            times_reviewed,
            times_seen,

            seen_at,

            reviewed_at,
            review_after,

            reviewed_at_count,
            cards_till_ready_for_review

        FROM CardsScore
        WHERE
            card_id = {card_id}
        LIMIT 1;
    "), card_id = card_id);

    let results = {
        let context = context.borrow();
        db_read_lock!(db_conn; context.database());
        let db_conn: &Connection = db_conn;

        let results = db_conn.query_row(&query, &[], |row| -> CardScore {

            let success: f64 =  row.get(1);

            let fail: f64 =  row.get(2);
            let raw_score = calculate_raw_score(success, fail);

            let times_reviewed: i64 =  row.get(3);
            let times_seen: i64 =  row.get(4);

            let review_after: i64 =  row.get(7);

            let reviewed_at_count: i64 =  row.get(8);
            let cards_till_ready_for_review: i64 =  row.get(9);

            let seen_at: UnixTimestamp = row.get(5);
            let reviewed_at: UnixTimestamp = row.get(6);


            let (review_after_normalized, review_after_time_control) = if review_after >= 3600 {
                // more than hour, convert to hours
                let review_after_normalized = (review_after as f64) / 60.0 / 60.0;
                let time_control = "HOURS".to_string();

                (review_after_normalized, time_control)
            } else {
                // less than an hour, convert to minutes
                let review_after_normalized = (review_after as f64) / 60.0;
                let time_control = "MINUTES".to_string();

                (review_after_normalized, time_control)
            };

            return CardScore {

                card_id: card_id,

                changelog: row.get(0),

                raw_score: raw_score,
                success: success,
                fail: fail,

                times_reviewed: times_reviewed as u64,
                times_seen: times_seen as u64,

                seen_at: timestamp::to_string(NaiveDateTime::from_timestamp(seen_at, 0)),
                seen_at_actual: seen_at,

                reviewed_at: timestamp::to_string(NaiveDateTime::from_timestamp(reviewed_at, 0)),
                reviewed_at_actual: reviewed_at,

                review_after_normalized: review_after_normalized as u64,
                review_after_time_control: review_after_time_control,
                review_after: review_after as u64,
                ready_for_review_at: timestamp::to_string(NaiveDateTime::from_timestamp(
                    reviewed_at + review_after, 0)),

                reviewed_at_count: reviewed_at_count as u64,
                cards_till_ready_for_review: cards_till_ready_for_review as u64
            };
        });

        results
    };

    match results {
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        }
        Ok(card_score) => {

            return Ok(card_score);
        }
    };
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

    // normalize to [0, 1.0)
    let pin: f64 = pin % 1.0;

    assert!(pin < 1.0);

    let card_idx = (pin * (upper_bound as f64)).floor() as Offset;

    // TODO: dev mode

    // assert!(0 <= card_idx); // TODO: useless invariant due to type
    assert!(card_idx < upper_bound);

    return card_idx;

}

#[inline]
fn calculate_raw_score(success: f64, fail: f64) -> f64 {
    let total: f64 = success + fail;
    (fail + 0.5f64) / (total + 1.0f64)
}
