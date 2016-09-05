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
use api::{decks, cards};

/* Types */

pub type Minutes = u64;
pub type Seconds = u64;
pub type UnixTimestamp = i64;

// TODO: change to u64? it's i64 b/c sqlite requires it.
pub type DeckID = i64;
pub type CardID = i64;
pub type StashID = i64;

// pagination
// TODO: need compile-time check to ensure >= 1 constraint (rust doesn't support this yet)
pub type Page = u64; // >= 1
pub type PerPage = u64; // >= 1
pub type Offset = u64; // >= 0
pub type ItemCount = u64;

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
pub struct DecksPageQuery(pub Page, pub DecksPageSort);

impl Default for DecksPageQuery {
    fn default() -> Self {
        DecksPageQuery(1, DecksPageSort::UpdatedAt(SortOrder::Descending))
    }
}

impl DecksPageQuery {

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

        return DecksPageQuery(page_num, decks_page_sort);
    }

    // TODO: test
    // TODO: move to some trait
    pub fn get_offset(&self) -> Offset {
        let page = self.0;
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

        let &DecksPageQuery(page, ref page_sort) = self;

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
        match self.1 {
            DecksPageSort::DeckName(_) => "Deck Name".to_owned(),
            DecksPageSort::CreatedAt(_) => "Created At".to_owned(),
            DecksPageSort::UpdatedAt(_) => "Updated At".to_owned()
        }
    }

    pub fn updated_at(&self) -> Self {
        return DecksPageQuery(self.0, DecksPageSort::UpdatedAt(self.1.order_by()))
    }

    pub fn created_at(&self) -> Self {
        return DecksPageQuery(self.0, DecksPageSort::CreatedAt(self.1.order_by()))
    }

    pub fn deck_name(&self) -> Self {
        return DecksPageQuery(self.0, DecksPageSort::DeckName(self.1.order_by()))
    }
}

impl SortOrderable for DecksPageQuery {

    fn order_by(&self) -> SortOrder {
        self.1.order_by()
    }

    fn ascending(&self) -> Self {
        DecksPageQuery(self.0, self.1.ascending())
    }

    fn descending(&self) -> Self {
        DecksPageQuery(self.0, self.1.descending())
    }

    fn sort_order_string(&self) -> String {
        self.1.sort_order_string()
    }
}

impl Pagination for DecksPageQuery {

    fn first(&self) -> Self {
        DecksPageQuery(1, self.1.clone())
    }

    fn previous(&self) -> Option<Self> {
        let page_num = self.current_page();

        if page_num <= 1 {
            return None;
        }

        return Some(DecksPageQuery(page_num - 1, self.1.clone()));
    }

    fn next(&self, context: Rc<RefCell<Context>>, deck_id: DeckID) -> Option<Self> {

        let _guard = context::read_lock(context.clone());

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

        return Some(DecksPageQuery(next_page, self.1.clone()));
    }

    fn current_page(&self) -> Page {
        self.0
    }

    fn num_of_pages(&self, context: Rc<RefCell<Context>>, deck_id: DeckID) -> Page {

        let _guard = context::read_lock(context.clone());

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

    fn should_show_pagination(&self, context: Rc<RefCell<Context>>, deck_id: DeckID) -> bool {
        return self.num_of_pages(context, deck_id) > 1;
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
                let page_query = DecksPageQuery(page as Page, self.1.clone());
                trailing_left_side.push(page_query);
            }

            return Some(trailing_left_side);
        }



        // 1 to __PAGINATION_TRAIL_SIZE
        for page in 1..(__PAGINATION_TRAIL_SIZE + 1) {
            let page_query = DecksPageQuery(page as Page, self.1.clone());
            trailing_left_side.push(page_query);
        }

        // add extra button
        if start == 1 {

            let extra_page_num = 1 + __PAGINATION_TRAIL_SIZE;

            let page_query = DecksPageQuery(extra_page_num as Page, self.1.clone());
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
            let page_query = DecksPageQuery(page as Page, self.1.clone());
            left_side.push(page_query);
        }

        return left_side;
    }

    fn get_right_side(&self, context: Rc<RefCell<Context>>, deck_id: DeckID) -> Vec<Self> {

        let current_page = self.current_page() as i64;
        let num_of_pages = self.num_of_pages(context, deck_id) as i64;

        let beta = current_page + __PAGINATION__ALPHA;

        let mut right_side: Vec<Self> = Vec::new();

        let start = current_page + 1;
        let end = if beta >= num_of_pages {
            num_of_pages
        } else {
            beta
        };

        for page in start..(end + 1) {
            let page_query = DecksPageQuery(page as Page, self.1.clone());
            right_side.push(page_query);
        }

        return right_side;

    }

    fn has_trailing_right_side_delimeter(&self, context: Rc<RefCell<Context>>, deck_id: DeckID) -> bool {

        let current_page = self.current_page() as i64;
        let num_of_pages = self.num_of_pages(context, deck_id) as i64;

        let end = current_page + __PAGINATION__ALPHA + 1 + __PAGINATION_TRAIL_SIZE;

        return end < num_of_pages;
    }

    fn get_trailing_right_side(&self, context: Rc<RefCell<Context>>, deck_id: DeckID) -> Option<Vec<Self>> {

        let current_page = self.current_page() as i64;
        let num_of_pages = self.num_of_pages(context, deck_id) as i64;

        let mut trailing_right_side: Vec<Self> = Vec::new();

        let end = current_page + __PAGINATION__ALPHA + 1 + __PAGINATION_TRAIL_SIZE;

        // overlapping exists; if possible, populate with rest of buttons
        if end > num_of_pages {

            let start = current_page + __PAGINATION__ALPHA + 1;

            if start > num_of_pages {
                return None;
            }

            for page in start..(num_of_pages + 1) {
                let page_query = DecksPageQuery(page as Page, self.1.clone());
                trailing_right_side.push(page_query);
            }

            return Some(trailing_right_side);
        }

        // invariant: end <= num_of_pages

        // add extra button
        if end == num_of_pages {

            let extra_page_num = (num_of_pages - __PAGINATION_TRAIL_SIZE + 1) - 1;


            let page_query = DecksPageQuery(extra_page_num as Page, self.1.clone());
            trailing_right_side.push(page_query);

        } else {
            // insert delimeter
        }

        let start = num_of_pages - __PAGINATION_TRAIL_SIZE + 1;

        for page in start..(num_of_pages + 1) {
            let page_query = DecksPageQuery(page as Page, self.1.clone());
            trailing_right_side.push(page_query);
        }

        return Some(trailing_right_side);

    }
}

#[derive(Debug, Clone)]
pub struct CardsPageQuery(pub Page, pub CardsPageSort);

impl Default for CardsPageQuery {
    fn default() -> Self {
        CardsPageQuery(1, CardsPageSort::UpdatedAt(SortOrder::Descending))
    }
}

impl CardsPageQuery {

    pub fn parse(query_string: &QueryString, context: Rc<RefCell<Context>>, deck_id: DeckID) -> Self {

        let default_per_page = constants::CARDS_PER_PAGE;

        let page_num: Page = match query_string.get("page") {
            None => 1,
            Some(maybe_page_num_string) => {
                match *maybe_page_num_string {
                    None => 1,
                    Some(ref page_num_string) => {
                        match page_num_string.parse::<Page>() {
                            Err(_) => 1,
                            Ok(page_num) => {

                                let _guard = context::read_lock(context.clone());

                                let children_count = match cards::total_num_of_cards_in_deck(context, deck_id) {
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

        return CardsPageQuery(page_num, cards_page_sort);
    }

    // TODO: test
    // TODO: move to some trait
    pub fn get_offset(&self) -> Offset {
        let page = self.0;
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

        let &CardsPageQuery(page, ref page_sort) = self;

        let (order_by, sort_order) = match *page_sort {
            CardsPageSort::CardTitle(ref sort_order) => ("card_title", sort_order),
            CardsPageSort::CreatedAt(ref sort_order) => ("created_at", sort_order),
            CardsPageSort::UpdatedAt(ref sort_order) => ("updated_at", sort_order)
        };

        let sort_by = match *sort_order {
            SortOrder::Ascending => "asc",
            SortOrder::Descending => "desc"
        };

        format!("page={page}&order_by={order_by}&sort_by={sort_by}",
            page = page, order_by = order_by, sort_by = sort_by)
    }

    pub fn sort_by_string(&self) -> String {
        match self.1 {
            CardsPageSort::CardTitle(_) => "Card Title".to_owned(),
            CardsPageSort::CreatedAt(_) => "Created At".to_owned(),
            CardsPageSort::UpdatedAt(_) => "Updated At".to_owned()
        }
    }

    pub fn updated_at(&self) -> Self {
        return CardsPageQuery(self.0, CardsPageSort::UpdatedAt(self.1.order_by()))
    }

    pub fn created_at(&self) -> Self {
        return CardsPageQuery(self.0, CardsPageSort::CreatedAt(self.1.order_by()))
    }

    pub fn card_title(&self) -> Self {
        return CardsPageQuery(self.0, CardsPageSort::CardTitle(self.1.order_by()))
    }
}

impl SortOrderable for CardsPageQuery {

    fn order_by(&self) -> SortOrder {
        self.1.order_by()
    }

    fn ascending(&self) -> Self {
        CardsPageQuery(self.0, self.1.ascending())
    }

    fn descending(&self) -> Self {
        CardsPageQuery(self.0, self.1.descending())
    }

    fn sort_order_string(&self) -> String {
        self.1.sort_order_string()
    }
}

impl Pagination for CardsPageQuery {

    fn first(&self) -> Self {
        CardsPageQuery(1, self.1.clone())
    }

    fn previous(&self) -> Option<Self> {
        let page_num = self.current_page();

        if page_num <= 1 {
            return None;
        }

        return Some(CardsPageQuery(page_num - 1, self.1.clone()));
    }

    fn next(&self, context: Rc<RefCell<Context>>, deck_id: DeckID) -> Option<Self> {

        let _guard = context::read_lock(context.clone());

        let children_count = match cards::total_num_of_cards_in_deck(context, deck_id) {
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

        return Some(CardsPageQuery(next_page, self.1.clone()));
    }

    fn current_page(&self) -> Page {
        self.0
    }

    fn num_of_pages(&self, context: Rc<RefCell<Context>>, deck_id: DeckID) -> Page {

        let _guard = context::read_lock(context.clone());

        let children_count = match cards::total_num_of_cards_in_deck(context, deck_id) {
            Ok(count) => count,
            Err(_) => {
                // TODO: internal error logging
                panic!();
            }
        };

        let num_of_pages = get_num_pages(children_count, self.get_per_page());

        return num_of_pages;
    }

    fn should_show_pagination(&self, context: Rc<RefCell<Context>>, deck_id: DeckID) -> bool {
        return self.num_of_pages(context, deck_id) > 1;
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
                let page_query = CardsPageQuery(page as Page, self.1.clone());
                trailing_left_side.push(page_query);
            }

            return Some(trailing_left_side);
        }



        // 1 to __PAGINATION_TRAIL_SIZE
        for page in 1..(__PAGINATION_TRAIL_SIZE + 1) {
            let page_query = CardsPageQuery(page as Page, self.1.clone());
            trailing_left_side.push(page_query);
        }

        // add extra button
        if start == 1 {

            let extra_page_num = 1 + __PAGINATION_TRAIL_SIZE;

            let page_query = CardsPageQuery(extra_page_num as Page, self.1.clone());
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
            let page_query = CardsPageQuery(page as Page, self.1.clone());
            left_side.push(page_query);
        }

        return left_side;
    }

    fn get_right_side(&self, context: Rc<RefCell<Context>>, deck_id: DeckID) -> Vec<Self> {

        let current_page = self.current_page() as i64;
        let num_of_pages = self.num_of_pages(context, deck_id) as i64;

        let beta = current_page + __PAGINATION__ALPHA;

        let mut right_side: Vec<Self> = Vec::new();

        let start = current_page + 1;
        let end = if beta >= num_of_pages {
            num_of_pages
        } else {
            beta
        };

        for page in start..(end + 1) {
            let page_query = CardsPageQuery(page as Page, self.1.clone());
            right_side.push(page_query);
        }

        return right_side;

    }

    fn has_trailing_right_side_delimeter(&self, context: Rc<RefCell<Context>>, deck_id: DeckID) -> bool {

        let current_page = self.current_page() as i64;
        let num_of_pages = self.num_of_pages(context, deck_id) as i64;

        let end = current_page + __PAGINATION__ALPHA + 1 + __PAGINATION_TRAIL_SIZE;

        return end < num_of_pages;
    }

    fn get_trailing_right_side(&self, context: Rc<RefCell<Context>>, deck_id: DeckID) -> Option<Vec<Self>> {

        let current_page = self.current_page() as i64;
        let num_of_pages = self.num_of_pages(context, deck_id) as i64;

        let mut trailing_right_side: Vec<Self> = Vec::new();

        let end = current_page + __PAGINATION__ALPHA + 1 + __PAGINATION_TRAIL_SIZE;

        // overlapping exists; if possible, populate with rest of buttons
        if end > num_of_pages {

            let start = current_page + __PAGINATION__ALPHA + 1;

            if start > num_of_pages {
                return None;
            }

            for page in start..(num_of_pages + 1) {
                let page_query = CardsPageQuery(page as Page, self.1.clone());
                trailing_right_side.push(page_query);
            }

            return Some(trailing_right_side);
        }

        // invariant: end <= num_of_pages

        // add extra button
        if end == num_of_pages {

            let extra_page_num = (num_of_pages - __PAGINATION_TRAIL_SIZE + 1) - 1;


            let page_query = CardsPageQuery(extra_page_num as Page, self.1.clone());
            trailing_right_side.push(page_query);

        } else {
            // insert delimeter
        }

        let start = num_of_pages - __PAGINATION_TRAIL_SIZE + 1;

        for page in start..(num_of_pages + 1) {
            let page_query = CardsPageQuery(page as Page, self.1.clone());
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
    fn next(&self, context: Rc<RefCell<Context>>, deck_id: DeckID) -> Option<Self>;
    fn current_page(&self) -> Page;
    fn num_of_pages(&self, context: Rc<RefCell<Context>>, deck_id: DeckID) -> Page;

    fn should_show_pagination(&self, context: Rc<RefCell<Context>>, deck_id: DeckID) -> bool;

    fn get_trailing_left_side(&self) -> Option<Vec<Self>>;
    fn has_trailing_left_side_delimeter(&self) -> bool;
    fn get_left_side(&self) -> Vec<Self>;

    fn get_right_side(&self, context: Rc<RefCell<Context>>, deck_id: DeckID) -> Vec<Self>;
    fn has_trailing_right_side_delimeter(&self, context: Rc<RefCell<Context>>, deck_id: DeckID) -> bool;
    fn get_trailing_right_side(&self, context: Rc<RefCell<Context>>, deck_id: DeckID) -> Option<Vec<Self>>;
}
