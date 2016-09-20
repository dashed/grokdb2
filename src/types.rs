/* rust lib imports */

use std::rc::Rc;
use std::cell::RefCell;

/* 3rd-party imports */

use url::percent_encoding::{QUERY_ENCODE_SET, utf8_percent_encode};
use serde_json;
use hyper;

/* local imports */

use context::{self, Context};
use route::QueryString;
use constants;
use api::{decks, cards, user};

/* Types */

// time
pub type Minutes = u64;
pub type Seconds = u64;
pub type UnixTimestamp = i64;

// TODO: change to u64? it's i64 b/c sqlite requires it.
// resource id
pub type DeckID = i64;
pub type CardID = i64;
pub type StashID = i64;
pub type UserID = i64;

// pagination
// TODO: need compile-time check to ensure >= 1 constraint (rust doesn't support this yet)
pub type Page = u64; // >= 1
pub type PerPage = u64; // >= 1
pub type Offset = u64; // >= 0
pub type ItemCount = u64; // >= 0

pub type ReviewCount = u64; // >= 0

#[derive(Serialize, Debug)]
pub struct JSONResponse {
    pub error: Option<String>,
    pub payload: Option<serde_json::Value>
}

// src: https://github.com/WhiteHouse/api-standards#error-handling
//
// 200 - OK
// 400 - Bad Request
// 500 - Internal Server Error
#[derive(Debug, PartialEq)]
pub enum APIStatus {
    Ok, // 200
    BadRequest, // 400
    MethodNotAllowed, // 405
    ServerError, // 500
}

impl APIStatus {
    pub fn status_code(&self) -> hyper::status::StatusCode {
        match *self {
            APIStatus::Ok => hyper::status::StatusCode::Ok,
            APIStatus::BadRequest => hyper::status::StatusCode::BadRequest,
            APIStatus::ServerError => hyper::status::StatusCode::InternalServerError,
            APIStatus::MethodNotAllowed => hyper::status::StatusCode::MethodNotAllowed,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Search {
    NoQuery,
    Query(String),
}

impl Default for Search {
    fn default() -> Self {
        Search::NoQuery
    }
}

impl Search {

    // TODO: test
    pub fn parse(query_string: &QueryString) -> Self {
        match query_string.get("search") {
            None => Search::NoQuery,
            Some(maybe_query) => {
                match *maybe_query {
                    None => Search::NoQuery,
                    Some(ref query) => Search::Query(query.clone())
                }
            }
        }
    }

    // TODO: test
    pub fn generate_query_string(&self) -> Option<String> {
        match *self {
            Search::NoQuery => None,
            Search::Query(ref search_query) => {

                let search_query = utf8_percent_encode(search_query, QUERY_ENCODE_SET)
                    .collect::<String>();

                Some(format!("search={}", search_query))
            }
        }
    }
}

// TODO: test for Search::parse

// conventions:
//
// timestamps:
//      - ascending: oldest to newest
//      - descending: newest to oldest
#[derive(Debug, Clone)]
pub enum SortOrder {
    Ascending,
    Descending,
}

pub trait SortOrderable {
    fn order_by(&self) -> SortOrder;
    fn ascending(&self) -> Self;
    fn descending(&self) -> Self;
    fn sort_order_string(&self) -> String;
}

#[derive(Debug, Clone)]
pub enum CardsPageSort {
    CardTitle(SortOrder),
    CreatedAt(SortOrder),
    UpdatedAt(SortOrder)
    // TODO: more sort options
}

impl Default for CardsPageSort {
    fn default() -> Self {
        CardsPageSort::UpdatedAt(SortOrder::Descending)
    }
}

impl SortOrderable for CardsPageSort {

    fn order_by(&self) -> SortOrder {
        match *self {
            CardsPageSort::CardTitle(ref order_by) |
            CardsPageSort::CreatedAt(ref order_by) |
            CardsPageSort::UpdatedAt(ref order_by) => {
                order_by.clone()
            }
        }
    }

    fn ascending(&self) -> Self {
        let new_value = SortOrder::Ascending;
        match *self {
            CardsPageSort::CardTitle(_) => CardsPageSort::CardTitle(new_value),
            CardsPageSort::CreatedAt(_) => CardsPageSort::CreatedAt(new_value),
            CardsPageSort::UpdatedAt(_) => CardsPageSort::UpdatedAt(new_value)
        }
    }

    fn descending(&self) -> Self {
        let new_value = SortOrder::Descending;
        match *self {
            CardsPageSort::CardTitle(_) => CardsPageSort::CardTitle(new_value),
            CardsPageSort::CreatedAt(_) => CardsPageSort::CreatedAt(new_value),
            CardsPageSort::UpdatedAt(_) => CardsPageSort::UpdatedAt(new_value)
        }
    }

    fn sort_order_string(&self) -> String {
        match *self {
            CardsPageSort::CardTitle(ref order_by) => {
                match *order_by {
                    SortOrder::Ascending => "Ascending".to_owned(),
                    SortOrder::Descending => "Descending".to_owned(),
                }
            },
            CardsPageSort::CreatedAt(ref order_by) | CardsPageSort::UpdatedAt(ref order_by) => {
                match *order_by {
                    SortOrder::Ascending => "Least Recent".to_owned(),
                    SortOrder::Descending => "Most Recent".to_owned(),
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum DecksPageSort {
    DeckName(SortOrder),
    CreatedAt(SortOrder),
    UpdatedAt(SortOrder), /* TODO: number of cards
                           * TODO: number of decks
                           *
                           * last time user reviewed this deck;
                           * not based on the cards the deck contains
                           * ReviewedAt(SortOrder) */
}

impl Default for DecksPageSort {
    fn default() -> Self {
        DecksPageSort::UpdatedAt(SortOrder::Descending)
    }
}

impl SortOrderable for DecksPageSort {

    fn order_by(&self) -> SortOrder {
        match *self {
            DecksPageSort::DeckName(ref order_by) |
            DecksPageSort::CreatedAt(ref order_by) |
            DecksPageSort::UpdatedAt(ref order_by) => {
                order_by.clone()
            }
        }
    }

    fn ascending(&self) -> Self {
        let new_value = SortOrder::Ascending;
        match *self {
            DecksPageSort::DeckName(_) => DecksPageSort::DeckName(new_value),
            DecksPageSort::CreatedAt(_) => DecksPageSort::CreatedAt(new_value),
            DecksPageSort::UpdatedAt(_) => DecksPageSort::UpdatedAt(new_value)
        }
    }

    fn descending(&self) -> Self {
        let new_value = SortOrder::Descending;
        match *self {
            DecksPageSort::DeckName(_) => DecksPageSort::DeckName(new_value),
            DecksPageSort::CreatedAt(_) => DecksPageSort::CreatedAt(new_value),
            DecksPageSort::UpdatedAt(_) => DecksPageSort::UpdatedAt(new_value)
        }
    }

    fn sort_order_string(&self) -> String {
        match *self {
            DecksPageSort::DeckName(ref order_by) => {
                match *order_by {
                    SortOrder::Ascending => "Ascending".to_owned(),
                    SortOrder::Descending => "Descending".to_owned(),
                }
            },
            DecksPageSort::CreatedAt(ref order_by) | DecksPageSort::UpdatedAt(ref order_by) => {
                match *order_by {
                    SortOrder::Ascending => "Least Recent".to_owned(),
                    SortOrder::Descending => "Most Recent".to_owned(),
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum MoveDecksPageQuery {
    Root(DeckID),
    SourceOfDecks(DecksPageQuery)
}

// helper to reduce repetition
#[inline]
fn get_card_deck_id(context: Rc<RefCell<Context>>, card_id: CardID) -> DeckID {

    let _guard = context::read_lock(context.clone());

    let card = match cards::get_card(context.clone(), card_id) {
        Ok(card) => card,
        Err(_) => {
            // TODO: internal error logging
            panic!();
        }
    };

    return card.deck_id;
}

enum ParentDeck {
    Root(DeckID),
    Parent(DeckID)
}

// TODO: remove
// #[inline]
fn get_parent_of_deck(context: Rc<RefCell<Context>>, deck_id: DeckID) -> ParentDeck {

    match decks::get_parent_id_of_deck(context.clone(), deck_id) {
        Ok(Some(deck_id)) => {
            return ParentDeck::Parent(deck_id);
        },
        Ok(None) => {
            match user::get_root_deck(context) {
                Ok(Some(root_deck_id)) => {

                    if root_deck_id == deck_id {
                        return ParentDeck::Root(root_deck_id);
                    } else {

                        // TODO: internal server error
                        // TODO: this should never occur

                        panic!();
                    }

                },
                Ok(None) => {
                    // TODO: internal server error
                    panic!();
                },
                Err(_why) => {
                    // TODO: internal server error
                    panic!();
                }
            }
        },
        Err(_why) => {
            // TODO: internal server error
            panic!();
        }
    }
}

impl MoveDecksPageQuery {

    pub fn default_with_parent_of_deck(context: Rc<RefCell<Context>>, deck_id: DeckID) -> Self {

        // TODO: refactor

        match get_parent_of_deck(context.clone(), deck_id) {
            ParentDeck::Parent(parent_id) => {

                match get_parent_of_deck(context, parent_id) {
                    ParentDeck::Parent(parent_id) => {

                        return MoveDecksPageQuery::SourceOfDecks(DecksPageQuery::default_with_deck(parent_id));
                    },
                    ParentDeck::Root(root_deck_id) => {
                        return MoveDecksPageQuery::Root(root_deck_id);
                    }
                }

            },
            ParentDeck::Root(root_deck_id) => {
                return MoveDecksPageQuery::Root(root_deck_id);
            }
        }
    }

    pub fn default_with_deck(_context: Rc<RefCell<Context>>, deck_id: DeckID) -> Self {
        return MoveDecksPageQuery::SourceOfDecks(DecksPageQuery::default_with_deck(deck_id));
    }

    pub fn default_with_card(context: Rc<RefCell<Context>>, card_id: CardID) -> Self {

        let deck_id = get_card_deck_id(context.clone(), card_id);

        match get_parent_of_deck(context, deck_id) {
            ParentDeck::Parent(parent_id) => {
                return MoveDecksPageQuery::SourceOfDecks(DecksPageQuery::default_with_deck(parent_id));
            },
            ParentDeck::Root(root_deck_id) => {
                return MoveDecksPageQuery::Root(root_deck_id);
            }
        }

    }

    pub fn parse_with_card(query_string: &QueryString, context: Rc<RefCell<Context>>, card_id: CardID) -> Self {

        let fallback_deck_id = get_card_deck_id(context.clone(), card_id);

        return MoveDecksPageQuery::parse_with_deck(query_string, context, fallback_deck_id);
    }

    pub fn parse_with_deck(query_string: &QueryString, context: Rc<RefCell<Context>>, fallback_deck_id: DeckID) -> Self {

        match query_string.get("deck") {
            None => {

                match get_parent_of_deck(context.clone(), fallback_deck_id) {
                    ParentDeck::Parent(parent_id) => {
                        return MoveDecksPageQuery::SourceOfDecks(
                            DecksPageQuery::parse(query_string, context, parent_id));
                    },
                    ParentDeck::Root(root_deck_id) => {
                        return MoveDecksPageQuery::Root(root_deck_id);
                    }
                }

            },
            Some(maybe_deck_id) => {
                match *maybe_deck_id {
                    None => {

                        match get_parent_of_deck(context.clone(), fallback_deck_id) {
                            ParentDeck::Parent(parent_id) => {
                                return MoveDecksPageQuery::SourceOfDecks(
                                    DecksPageQuery::parse(query_string, context, parent_id));
                            },
                            ParentDeck::Root(root_deck_id) => {
                                return MoveDecksPageQuery::Root(root_deck_id);
                            }
                        }

                    },
                    Some(ref deck_id_string) => {

                        if deck_id_string == "top" {

                            match user::get_root_deck(context) {
                                Ok(Some(root_deck_id)) => {
                                    return MoveDecksPageQuery::Root(root_deck_id);
                                },
                                Ok(None) => {
                                    // TODO: internal server error
                                    panic!();
                                },
                                Err(_why) => {
                                    // TODO: internal server error
                                    panic!();
                                }
                            }

                        }

                        match deck_id_string.parse::<DeckID>() {
                            Err(_) => {

                                match get_parent_of_deck(context.clone(), fallback_deck_id) {
                                    ParentDeck::Parent(parent_id) => {
                                        return MoveDecksPageQuery::SourceOfDecks(
                                            DecksPageQuery::parse(query_string, context, parent_id));
                                    },
                                    ParentDeck::Root(root_deck_id) => {
                                        return MoveDecksPageQuery::Root(root_deck_id);
                                    }
                                }

                            },
                            Ok(deck_id) => {

                                match decks::deck_exists(context.clone(), deck_id) {
                                    Ok(exists) => {

                                        if exists {

                                            return MoveDecksPageQuery::SourceOfDecks(
                                                DecksPageQuery::parse(query_string, context, deck_id));

                                        } else {

                                            match get_parent_of_deck(context.clone(), fallback_deck_id) {
                                                ParentDeck::Parent(parent_id) => {
                                                    return MoveDecksPageQuery::SourceOfDecks(
                                                        DecksPageQuery::parse(query_string, context, parent_id));
                                                },
                                                ParentDeck::Root(root_deck_id) => {
                                                    return MoveDecksPageQuery::Root(root_deck_id);
                                                }
                                            }

                                        }
                                    },
                                    Err(_) => {
                                        // TODO: internal error logging
                                        panic!();
                                    }
                                }

                            }
                        }
                    }
                }
            }
        }

    }

    // TODO: test
    pub fn generate_query_string(&self) -> String {

        match *self {
            MoveDecksPageQuery::Root(_) => {
                return format!("deck=top");
            },
            MoveDecksPageQuery::SourceOfDecks(ref page_query) => {

                let &DecksPageQuery(deck_id, _, _) = page_query;

                return format!("deck={deck_id}&{rest}",
                    deck_id = deck_id, rest = page_query.generate_query_string());

            }
        }

    }
}

impl Pagination for MoveDecksPageQuery {

    fn first(&self) -> Self {

        match *self {
            MoveDecksPageQuery::Root(_) => {
                self.clone()
            },
            MoveDecksPageQuery::SourceOfDecks(ref page_query) => {
                MoveDecksPageQuery::SourceOfDecks(page_query.first())
            }
        }

    }

    fn previous(&self) -> Option<Self> {
        let page_num = self.current_page();

        if page_num <= 1 {
            return None;
        }

        match *self {
            MoveDecksPageQuery::Root(_) => {
                return None;
            },
            MoveDecksPageQuery::SourceOfDecks(ref page_query) => {
                match page_query.previous() {
                    None => None,
                    Some(foo) => {
                        Some(MoveDecksPageQuery::SourceOfDecks(foo))
                    }
                }
            }
        }
    }

    fn next(&self, context: Rc<RefCell<Context>>) -> Option<Self> {

        match *self {
            MoveDecksPageQuery::Root(_) => {
                return None;
            },
            MoveDecksPageQuery::SourceOfDecks(ref page_query) => {
                match page_query.next(context) {
                    None => None,
                    Some(foo) => {
                        Some(MoveDecksPageQuery::SourceOfDecks(foo))
                    }
                }
            }
        }

    }

    fn current_page(&self) -> Page {

        match *self {
            MoveDecksPageQuery::Root(_) => 1,
            MoveDecksPageQuery::SourceOfDecks(ref page_query) => {
                page_query.current_page()
            }
        }
    }

    fn num_of_pages(&self, context: Rc<RefCell<Context>>) -> Page {

        match *self {
            MoveDecksPageQuery::Root(_) => 1,
            MoveDecksPageQuery::SourceOfDecks(ref page_query) => {
                page_query.num_of_pages(context)
            }
        }

    }

    fn should_show_pagination(&self, context: Rc<RefCell<Context>>) -> bool {
        return self.num_of_pages(context) > 1;
    }

    fn get_trailing_left_side(&self) -> Option<Vec<Self>> {

        match *self {
            MoveDecksPageQuery::Root(_) => None,
            MoveDecksPageQuery::SourceOfDecks(ref page_query) => {

                match page_query.get_trailing_left_side() {
                    None => None,
                    Some(list) => {

                        let collected = list
                            .into_iter()
                            .map(|elem| {
                                MoveDecksPageQuery::SourceOfDecks(elem)
                            })
                            .collect();

                        return Some(collected);
                    }
                }

            }
        }
    }

    fn has_trailing_left_side_delimeter(&self) -> bool {

        match *self {
            MoveDecksPageQuery::Root(_) => false,
            MoveDecksPageQuery::SourceOfDecks(ref page_query) => {
                page_query.has_trailing_left_side_delimeter()
            }
        }
    }

    fn get_left_side(&self) -> Vec<Self> {

        match *self {
            MoveDecksPageQuery::Root(_) => vec![],
            MoveDecksPageQuery::SourceOfDecks(ref page_query) => {
                page_query
                    .get_left_side()
                    .into_iter()
                    .map(|elem| {
                        MoveDecksPageQuery::SourceOfDecks(elem)
                    })
                    .collect()
            }
        }

    }

    fn get_right_side(&self, context: Rc<RefCell<Context>>) -> Vec<Self> {

        match *self {
            MoveDecksPageQuery::Root(_) => vec![],
            MoveDecksPageQuery::SourceOfDecks(ref page_query) => {
                page_query
                    .get_right_side(context)
                    .into_iter()
                    .map(|elem| {
                        MoveDecksPageQuery::SourceOfDecks(elem)
                    })
                    .collect()
            }
        }

    }

    fn has_trailing_right_side_delimeter(&self, context: Rc<RefCell<Context>>) -> bool {

        match *self {
            MoveDecksPageQuery::Root(_) => false,
            MoveDecksPageQuery::SourceOfDecks(ref page_query) => {
                page_query.has_trailing_right_side_delimeter(context)
            }
        }
    }

    fn get_trailing_right_side(&self, context: Rc<RefCell<Context>>) -> Option<Vec<Self>> {

        match *self {
            MoveDecksPageQuery::Root(_) => None,
            MoveDecksPageQuery::SourceOfDecks(ref page_query) => {
                match page_query.get_trailing_right_side(context) {
                    None => None,
                    Some(list) => {
                        let collected: Vec<Self> = list
                            .into_iter()
                            .map(|elem| {
                                MoveDecksPageQuery::SourceOfDecks(elem)
                            })
                            .collect();

                        return Some(collected);
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct DecksPageQuery(pub DeckID, pub Page, pub DecksPageSort);

impl DecksPageQuery {

    pub fn default_with_deck(deck_id: DeckID) -> Self {
        return DecksPageQuery(deck_id, 1, DecksPageSort::UpdatedAt(SortOrder::Descending));
    }

    pub fn parse(query_string: &QueryString, context: Rc<RefCell<Context>>, deck_id: DeckID) -> Self {

        let default_per_page = constants::DECKS_PER_PAGE;

        let page_num: Page = match query_string.get("page") {
            None => 1,
            Some(maybe_page_num_string) => {
                match *maybe_page_num_string {
                    None => 1,
                    Some(ref page_num_string) => {
                        match page_num_string.parse::<Page>() {
                            Err(_) => 1,
                            Ok(page_num) => {

                                let page_num = if page_num <= 0 {
                                    1
                                } else {
                                    page_num
                                };

                                let _guard = context::read_lock(context.clone());

                                let children_count = match decks::get_deck_children_total_count(context, deck_id) {
                                    Ok(count) => count,
                                    Err(_) => {
                                        // TODO: internal error logging
                                        panic!();
                                    }
                                };

                                let num_of_pages = get_num_pages(children_count, default_per_page);

                                get_page_num(num_of_pages, page_num)

                            }
                        }
                    }
                }
            }
        };

        let sort_by = match query_string.get("sort_by") {
            None => SortOrder::Descending,
            Some(maybe_sort_by) => {
                match *maybe_sort_by {
                    None => SortOrder::Descending,
                    Some(ref sort_by_string) => {
                        match sort_by_string.as_ref() {
                            "desc" => SortOrder::Descending,
                            "asc" => SortOrder::Ascending,
                            _ => SortOrder::Descending
                        }
                    }
                }
            }
        };

        let decks_page_sort = match query_string.get("order_by") {
            None => DecksPageSort::UpdatedAt(sort_by),
            Some(maybe_order_by) => {
                match *maybe_order_by {
                    None => DecksPageSort::UpdatedAt(sort_by),
                    Some(ref order_by_string) => {
                        match order_by_string.as_ref() {
                            "deck_name" => DecksPageSort::DeckName(sort_by),
                            "created_at" => DecksPageSort::CreatedAt(sort_by),
                            "updated_at" => DecksPageSort::UpdatedAt(sort_by),
                            _ => DecksPageSort::UpdatedAt(sort_by)
                        }
                    }
                }
            }
        };

        return DecksPageQuery(deck_id, page_num, decks_page_sort);
    }

    // TODO: test
    // TODO: move to some trait
    pub fn get_offset(&self) -> Offset {
        let page = self.1;
        let offset = (page - 1) * self.get_per_page();
        return offset;
    }

    // TODO: test
    // TODO: move to pagination trait
    pub fn get_per_page(&self) -> PerPage {
        return constants::DECKS_PER_PAGE;
    }

    // TODO: test
    pub fn generate_query_string(&self) -> String {

        let &DecksPageQuery(_deck_id, page, ref page_sort) = self;

        let (order_by, sort_order) = match *page_sort {
            DecksPageSort::DeckName(ref sort_order) => ("deck_name", sort_order),
            DecksPageSort::CreatedAt(ref sort_order) => ("created_at", sort_order),
            DecksPageSort::UpdatedAt(ref sort_order) => ("updated_at", sort_order)
        };

        let sort_by = match *sort_order {
            SortOrder::Ascending => "asc",
            SortOrder::Descending => "desc"
        };

        format!("page={page}&order_by={order_by}&sort_by={sort_by}",
            page = page, order_by = order_by, sort_by = sort_by)
    }

    pub fn sort_by_string(&self) -> String {
        match self.2 {
            DecksPageSort::DeckName(_) => "Deck Name".to_owned(),
            DecksPageSort::CreatedAt(_) => "Created At".to_owned(),
            DecksPageSort::UpdatedAt(_) => "Updated At".to_owned()
        }
    }

    pub fn updated_at(&self) -> Self {
        return DecksPageQuery(self.0, self.1, DecksPageSort::UpdatedAt(self.2.order_by()))
    }

    pub fn created_at(&self) -> Self {
        return DecksPageQuery(self.0, self.1, DecksPageSort::CreatedAt(self.2.order_by()))
    }

    pub fn deck_name(&self) -> Self {
        return DecksPageQuery(self.0, self.1, DecksPageSort::DeckName(self.2.order_by()))
    }
}

impl SortOrderable for DecksPageQuery {

    fn order_by(&self) -> SortOrder {
        self.2.order_by()
    }

    fn ascending(&self) -> Self {
        DecksPageQuery(self.0, self.1, self.2.ascending())
    }

    fn descending(&self) -> Self {
        DecksPageQuery(self.0, self.1, self.2.descending())
    }

    fn sort_order_string(&self) -> String {
        self.2.sort_order_string()
    }
}

impl Pagination for DecksPageQuery {

    fn first(&self) -> Self {
        DecksPageQuery(self.0, 1, self.2.clone())
    }

    fn previous(&self) -> Option<Self> {
        let page_num = self.current_page();

        if page_num <= 1 {
            return None;
        }

        let prev_page = page_num - 1;

        return Some(DecksPageQuery(self.0, prev_page, self.2.clone()));
    }

    fn next(&self, context: Rc<RefCell<Context>>) -> Option<Self> {

        let _guard = context::read_lock(context.clone());

        let deck_id = self.0;

        let children_count = match decks::get_deck_children_total_count(context, deck_id) {
            Ok(count) => count,
            Err(_) => {
                // TODO: internal error logging
                panic!();
            }
        };

        let num_of_pages = get_num_pages(children_count, self.get_per_page());

        let next_page = self.current_page() + 1;

        if next_page > num_of_pages {
            return None;
        }

        return Some(DecksPageQuery(deck_id, next_page, self.2.clone()));
    }

    fn current_page(&self) -> Page {
        self.1
    }

    fn num_of_pages(&self, context: Rc<RefCell<Context>>) -> Page {

        let _guard = context::read_lock(context.clone());

        let deck_id = self.0;

        let children_count = match decks::get_deck_children_total_count(context, deck_id) {
            Ok(count) => count,
            Err(_) => {
                // TODO: internal error logging
                panic!();
            }
        };

        let num_of_pages = get_num_pages(children_count, self.get_per_page());

        return num_of_pages;
    }

    fn should_show_pagination(&self, context: Rc<RefCell<Context>>) -> bool {
        return self.num_of_pages(context) > 1;
    }

    fn get_trailing_left_side(&self) -> Option<Vec<Self>> {

        let current_page = self.current_page() as i64;
        let start: i64 = current_page - __PAGINATION__ALPHA - 1 - __PAGINATION_TRAIL_SIZE;

        let mut trailing_left_side: Vec<Self> = Vec::new();

        // overlapping exists; if possible, populate with rest of buttons
        if start <= 0 {

            let end = current_page - __PAGINATION__ALPHA - 1;

            if end <= 0 {
                return None;
            }

            // 1 to end
            for page in 1..(end + 1) {
                let page_query = DecksPageQuery(self.0, page as Page, self.2.clone());
                trailing_left_side.push(page_query);
            }

            return Some(trailing_left_side);
        }


        // 1 to __PAGINATION_TRAIL_SIZE
        for page in 1..(__PAGINATION_TRAIL_SIZE + 1) {
            let page_query = DecksPageQuery(self.0, page as Page, self.2.clone());
            trailing_left_side.push(page_query);
        }

        // add extra button
        if start == 1 {

            let extra_page_num = 1 + __PAGINATION_TRAIL_SIZE;

            let page_query = DecksPageQuery(self.0, extra_page_num as Page, self.2.clone());
            trailing_left_side.push(page_query);

        }

        return Some(trailing_left_side);
    }

    fn has_trailing_left_side_delimeter(&self) -> bool {

        let current_page = self.current_page() as i64;
        let start: i64 = current_page - __PAGINATION__ALPHA - 1 - __PAGINATION_TRAIL_SIZE;

        return start > 1;
    }

    fn get_left_side(&self) -> Vec<Self> {

        let current_page = self.current_page() as i64;

        let beta = current_page - __PAGINATION__ALPHA;

        let mut left_side: Vec<Self> = Vec::new();

        let start = if beta <= 1 {
            1
        } else {
            beta
        };

        let end = current_page - 1;

        // start to end
        for page in start..(end + 1) {
            let page_query = DecksPageQuery(self.0, page as Page, self.2.clone());
            left_side.push(page_query);
        }

        return left_side;
    }

    fn get_right_side(&self, context: Rc<RefCell<Context>>) -> Vec<Self> {

        let deck_id = self.0;

        let current_page = self.current_page() as i64;
        let num_of_pages = self.num_of_pages(context) as i64;

        let beta = current_page + __PAGINATION__ALPHA;

        let mut right_side: Vec<Self> = Vec::new();

        let start = current_page + 1;
        let end = if beta >= num_of_pages {
            num_of_pages
        } else {
            beta
        };

        for page in start..(end + 1) {
            let page_query = DecksPageQuery(deck_id, page as Page, self.2.clone());
            right_side.push(page_query);
        }

        return right_side;

    }

    fn has_trailing_right_side_delimeter(&self, context: Rc<RefCell<Context>>) -> bool {

        let current_page = self.current_page() as i64;
        let num_of_pages = self.num_of_pages(context) as i64;

        let end = current_page + __PAGINATION__ALPHA + 1 + __PAGINATION_TRAIL_SIZE;

        return end < num_of_pages;
    }

    fn get_trailing_right_side(&self, context: Rc<RefCell<Context>>) -> Option<Vec<Self>> {

        let current_page = self.current_page() as i64;
        let num_of_pages = self.num_of_pages(context) as i64;

        let mut trailing_right_side: Vec<Self> = Vec::new();

        let end = current_page + __PAGINATION__ALPHA + 1 + __PAGINATION_TRAIL_SIZE;

        // overlapping exists; if possible, populate with rest of buttons
        if end > num_of_pages {

            let start = current_page + __PAGINATION__ALPHA + 1;

            if start > num_of_pages {
                return None;
            }

            for page in start..(num_of_pages + 1) {
                let page_query = DecksPageQuery(self.0, page as Page, self.2.clone());
                trailing_right_side.push(page_query);
            }

            return Some(trailing_right_side);
        }

        // invariant: end <= num_of_pages

        // add extra button
        if end == num_of_pages {

            let extra_page_num = (num_of_pages - __PAGINATION_TRAIL_SIZE + 1) - 1;


            let page_query = DecksPageQuery(self.0, extra_page_num as Page, self.2.clone());
            trailing_right_side.push(page_query);

        } else {
            // insert delimeter
        }

        let start = num_of_pages - __PAGINATION_TRAIL_SIZE + 1;

        for page in start..(num_of_pages + 1) {
            let page_query = DecksPageQuery(self.0, page as Page, self.2.clone());
            trailing_right_side.push(page_query);
        }

        return Some(trailing_right_side);

    }
}

#[derive(Debug, Clone)]
pub struct CardsPageQuery(pub DeckID, pub Page, pub CardsPageSort, pub Search);

impl CardsPageQuery {

    pub fn default_with_deck(deck_id: DeckID) -> Self {
        CardsPageQuery(deck_id, 1, CardsPageSort::UpdatedAt(SortOrder::Descending), Search::NoQuery)
    }

    pub fn parse(query_string: &QueryString, context: Rc<RefCell<Context>>, deck_id: DeckID) -> Self {

        let default_per_page = constants::CARDS_PER_PAGE;

        let search = Search::parse(query_string);

        let page_num: Page = match query_string.get("page") {
            None => 1,
            Some(maybe_page_num_string) => {
                match *maybe_page_num_string {
                    None => 1,
                    Some(ref page_num_string) => {
                        match page_num_string.parse::<Page>() {
                            Err(_) => 1,
                            Ok(page_num) => {

                                // normalize
                                let page_num = if page_num <= 0 {
                                    1
                                } else {
                                    page_num
                                };

                                let _guard = context::read_lock(context.clone());

                                let children_count = match cards::total_num_of_cards_in_deck(context, deck_id, &search) {
                                    Ok(count) => count,
                                    Err(why) => {
                                        // TODO: internal error logging
                                        panic!("{:?}", why);
                                    }
                                };

                                let num_of_pages = get_num_pages(children_count, default_per_page);

                                get_page_num(num_of_pages, page_num)
                            }
                        }
                    }
                }
            }
        };

        let sort_by = match query_string.get("sort_by") {
            None => SortOrder::Descending,
            Some(maybe_sort_by) => {
                match *maybe_sort_by {
                    None => SortOrder::Descending,
                    Some(ref sort_by_string) => {
                        match sort_by_string.as_ref() {
                            "desc" => SortOrder::Descending,
                            "asc" => SortOrder::Ascending,
                            _ => SortOrder::Descending
                        }
                    }
                }
            }
        };

        let cards_page_sort = match query_string.get("order_by") {
            None => CardsPageSort::UpdatedAt(sort_by),
            Some(maybe_order_by) => {
                match *maybe_order_by {
                    None => CardsPageSort::UpdatedAt(sort_by),
                    Some(ref order_by_string) => {
                        match order_by_string.as_ref() {
                            "card_title" => CardsPageSort::CardTitle(sort_by),
                            "created_at" => CardsPageSort::CreatedAt(sort_by),
                            "updated_at" => CardsPageSort::UpdatedAt(sort_by),
                            _ => CardsPageSort::UpdatedAt(sort_by)
                        }
                    }
                }
            }
        };

        return CardsPageQuery(deck_id, page_num, cards_page_sort, search);
    }

    // TODO: test
    // TODO: move to some trait
    pub fn get_offset(&self) -> Offset {
        let page = self.1;
        let offset = (page - 1) * self.get_per_page();
        return offset;
    }

    // TODO: test
    // TODO: move to pagination trait
    pub fn get_per_page(&self) -> PerPage {
        return constants::CARDS_PER_PAGE;
    }

    // TODO: test
    pub fn generate_query_string(&self) -> String {

        let &CardsPageQuery(_deck_id, page, ref page_sort, ref _search) = self;

        let (order_by, sort_order) = match *page_sort {
            CardsPageSort::CardTitle(ref sort_order) => ("card_title", sort_order),
            CardsPageSort::CreatedAt(ref sort_order) => ("created_at", sort_order),
            CardsPageSort::UpdatedAt(ref sort_order) => ("updated_at", sort_order)
        };

        let sort_by = match *sort_order {
            SortOrder::Ascending => "asc",
            SortOrder::Descending => "desc"
        };

        let mut query = format!("page={page}&order_by={order_by}&sort_by={sort_by}",
            page = page, order_by = order_by, sort_by = sort_by);

        let ref search = self.3;

        if let Some(search_query) = search.generate_query_string() {
            query = query + &format!("&{}", search_query);
        }

        return query;
    }

    pub fn sort_by_string(&self) -> String {
        match self.2 {
            CardsPageSort::CardTitle(_) => "Card Title".to_owned(),
            CardsPageSort::CreatedAt(_) => "Created At".to_owned(),
            CardsPageSort::UpdatedAt(_) => "Updated At".to_owned()
        }
    }

    pub fn updated_at(&self) -> Self {
        return CardsPageQuery(self.0, self.1, CardsPageSort::UpdatedAt(self.2.order_by()), self.3.clone())
    }

    pub fn created_at(&self) -> Self {
        return CardsPageQuery(self.0, self.1, CardsPageSort::CreatedAt(self.2.order_by()), self.3.clone())
    }

    pub fn card_title(&self) -> Self {
        return CardsPageQuery(self.0, self.1, CardsPageSort::CardTitle(self.2.order_by()), self.3.clone())
    }
}

impl SortOrderable for CardsPageQuery {

    fn order_by(&self) -> SortOrder {
        self.2.order_by()
    }

    fn ascending(&self) -> Self {
        CardsPageQuery(self.0, self.1, self.2.ascending(), self.3.clone())
    }

    fn descending(&self) -> Self {
        CardsPageQuery(self.0, self.1, self.2.descending(), self.3.clone())
    }

    fn sort_order_string(&self) -> String {
        self.2.sort_order_string()
    }
}

impl Pagination for CardsPageQuery {

    fn first(&self) -> Self {
        CardsPageQuery(self.0, 1, self.2.clone(), self.3.clone())
    }

    fn previous(&self) -> Option<Self> {
        let page_num = self.current_page();

        if page_num <= 1 {
            return None;
        }

        let prev_page = page_num - 1;

        return Some(CardsPageQuery(self.0, prev_page, self.2.clone(), self.3.clone()));
    }

    fn next(&self, context: Rc<RefCell<Context>>) -> Option<Self> {

        let deck_id = self.0;

        let _guard = context::read_lock(context.clone());

        let ref search = self.3;

        let children_count = match cards::total_num_of_cards_in_deck(context, deck_id, search) {
            Ok(count) => count,
            Err(_) => {
                // TODO: internal error logging
                panic!();
            }
        };

        let num_of_pages = get_num_pages(children_count, self.get_per_page());

        let next_page = self.current_page() + 1;

        if next_page > num_of_pages {
            return None;
        }

        return Some(CardsPageQuery(deck_id, next_page, self.2.clone(), self.3.clone()));
    }

    fn current_page(&self) -> Page {
        self.1
    }

    fn num_of_pages(&self, context: Rc<RefCell<Context>>) -> Page {

        let deck_id = self.0;
        let ref search = self.3;

        let _guard = context::read_lock(context.clone());

        let children_count = match cards::total_num_of_cards_in_deck(context, deck_id, search) {
            Ok(count) => count,
            Err(_) => {
                // TODO: internal error logging
                panic!();
            }
        };

        let num_of_pages = get_num_pages(children_count, self.get_per_page());

        return num_of_pages;
    }

    fn should_show_pagination(&self, context: Rc<RefCell<Context>>) -> bool {
        return self.num_of_pages(context) > 1;
    }

    fn get_trailing_left_side(&self) -> Option<Vec<Self>> {

        let current_page = self.current_page() as i64;
        let start: i64 = current_page - __PAGINATION__ALPHA - 1 - __PAGINATION_TRAIL_SIZE;

        let mut trailing_left_side: Vec<Self> = Vec::new();

        // overlapping exists; if possible, populate with rest of buttons
        if start <= 0 {

            let end = current_page - __PAGINATION__ALPHA - 1;

            if end <= 0 {
                return None;
            }

            // 1 to end
            for page in 1..(end + 1) {
                let page_query = CardsPageQuery(self.0, page as Page, self.2.clone(), self.3.clone());
                trailing_left_side.push(page_query);
            }

            return Some(trailing_left_side);
        }



        // 1 to __PAGINATION_TRAIL_SIZE
        for page in 1..(__PAGINATION_TRAIL_SIZE + 1) {
            let page_query = CardsPageQuery(self.0, page as Page, self.2.clone(), self.3.clone());
            trailing_left_side.push(page_query);
        }

        // add extra button
        if start == 1 {

            let extra_page_num = 1 + __PAGINATION_TRAIL_SIZE;

            let page_query = CardsPageQuery(self.0, extra_page_num as Page, self.2.clone(), self.3.clone());
            trailing_left_side.push(page_query);

        }

        return Some(trailing_left_side);
    }

    fn has_trailing_left_side_delimeter(&self) -> bool {

        let current_page = self.current_page() as i64;
        let start: i64 = current_page - __PAGINATION__ALPHA - 1 - __PAGINATION_TRAIL_SIZE;

        return start > 1;
    }

    fn get_left_side(&self) -> Vec<Self> {

        let current_page = self.current_page() as i64;

        let beta = current_page - __PAGINATION__ALPHA;

        let mut left_side: Vec<Self> = Vec::new();

        let start = if beta <= 1 {
            1
        } else {
            beta
        };

        let end = current_page - 1;

        // start to end
        for page in start..(end + 1) {
            let page_query = CardsPageQuery(self.0, page as Page, self.2.clone(), self.3.clone());
            left_side.push(page_query);
        }

        return left_side;
    }

    fn get_right_side(&self, context: Rc<RefCell<Context>>) -> Vec<Self> {

        let current_page = self.current_page() as i64;
        let num_of_pages = self.num_of_pages(context) as i64;

        let beta = current_page + __PAGINATION__ALPHA;

        let mut right_side: Vec<Self> = Vec::new();

        let start = current_page + 1;
        let end = if beta >= num_of_pages {
            num_of_pages
        } else {
            beta
        };

        for page in start..(end + 1) {
            let page_query = CardsPageQuery(self.0, page as Page, self.2.clone(), self.3.clone());
            right_side.push(page_query);
        }

        return right_side;

    }

    fn has_trailing_right_side_delimeter(&self, context: Rc<RefCell<Context>>) -> bool {

        let current_page = self.current_page() as i64;
        let num_of_pages = self.num_of_pages(context) as i64;

        let end = current_page + __PAGINATION__ALPHA + 1 + __PAGINATION_TRAIL_SIZE;

        return end < num_of_pages;
    }

    fn get_trailing_right_side(&self, context: Rc<RefCell<Context>>) -> Option<Vec<Self>> {

        let current_page = self.current_page() as i64;
        let num_of_pages = self.num_of_pages(context) as i64;

        let mut trailing_right_side: Vec<Self> = Vec::new();

        let end = current_page + __PAGINATION__ALPHA + 1 + __PAGINATION_TRAIL_SIZE;

        // overlapping exists; if possible, populate with rest of buttons
        if end > num_of_pages {

            let start = current_page + __PAGINATION__ALPHA + 1;

            if start > num_of_pages {
                return None;
            }

            for page in start..(num_of_pages + 1) {
                let page_query = CardsPageQuery(self.0, page as Page, self.2.clone(), self.3.clone());
                trailing_right_side.push(page_query);
            }

            return Some(trailing_right_side);
        }

        // invariant: end <= num_of_pages

        // add extra button
        if end == num_of_pages {

            let extra_page_num = (num_of_pages - __PAGINATION_TRAIL_SIZE + 1) - 1;


            let page_query = CardsPageQuery(self.0, extra_page_num as Page, self.2.clone(), self.3.clone());
            trailing_right_side.push(page_query);

        } else {
            // insert delimeter
        }

        let start = num_of_pages - __PAGINATION_TRAIL_SIZE + 1;

        for page in start..(num_of_pages + 1) {
            let page_query = CardsPageQuery(self.0, page as Page, self.2.clone(), self.3.clone());
            trailing_right_side.push(page_query);
        }

        return Some(trailing_right_side);

    }
}

// helper to get number of pages
// TODO: test
#[inline]
fn get_num_pages(item_count: ItemCount, per_page: PerPage) -> Page {
    let item_count = item_count as f64;
    let per_page = per_page as f64;

    let num_of_pages = (item_count / per_page).ceil();

    return num_of_pages as Page;
}

// TODO: test
#[inline]
fn get_page_num(num_of_pages: Page, page_num: Page) -> Page {
    if num_of_pages <= 0 {
        1
    } else if page_num <= num_of_pages {
        page_num
    } else {
        1
    }
}

// amount of page buttons after/before current
const __PAGINATION__ALPHA: i64 = 3;
// amount of page buttons on the ends
const __PAGINATION_TRAIL_SIZE: i64 = 2;

pub trait Pagination where Self: Sized {

    fn first(&self) -> Self;

    fn previous(&self) -> Option<Self>;
    fn next(&self, context: Rc<RefCell<Context>>) -> Option<Self>;
    fn current_page(&self) -> Page;
    fn num_of_pages(&self, context: Rc<RefCell<Context>>) -> Page;

    fn should_show_pagination(&self, context: Rc<RefCell<Context>>) -> bool;

    fn get_trailing_left_side(&self) -> Option<Vec<Self>>;
    fn has_trailing_left_side_delimeter(&self) -> bool;
    fn get_left_side(&self) -> Vec<Self>;

    fn get_right_side(&self, context: Rc<RefCell<Context>>) -> Vec<Self>;
    fn has_trailing_right_side_delimeter(&self, context: Rc<RefCell<Context>>) -> bool;
    fn get_trailing_right_side(&self, context: Rc<RefCell<Context>>) -> Option<Vec<Self>>;
}
