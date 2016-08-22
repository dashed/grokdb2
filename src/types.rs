/* rust lib imports */

use std::rc::Rc;
use std::cell::RefCell;

/* 3rd-party imports */

use url::percent_encoding::{QUERY_ENCODE_SET, utf8_percent_encode};

/* local imports */

use context::Context;
use route::QueryString;
use constants;
use api::decks;

/* Types */

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

#[derive(Debug)]
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
#[derive(Debug)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Debug)]
pub enum DecksPageSort {
    DeckTitle(SortOrder),
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

#[derive(Debug)]
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
                            "deck_title" => DecksPageSort::DeckTitle(sort_by),
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

    pub fn get_offset(&self) -> Offset {
        let page = self.0;
        let offset = (page - 1) * self.get_per_page();
        return offset;
    }

    pub fn get_per_page(&self) -> PerPage {
        return constants::DECKS_PER_PAGE;
    }

    pub fn generate_query_string(&self) -> String {

        let &DecksPageQuery(page, ref page_sort) = self;

        let (order_by, sort_order) = match *page_sort {
            DecksPageSort::DeckTitle(ref sort_order) => ("deck_title", sort_order),
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
}

// TODO: test for DecksPageQuery::parse
// TODO: test for DecksPageQuery::get_offset
// TODO: test for DecksPageQuery::get_per_page

// helper to get number of pages
#[inline]
fn get_num_pages(item_count: ItemCount, per_page: PerPage) -> Page {
    let item_count = item_count as f64;
    let per_page = per_page as f64;

    let num_of_pages = (item_count / per_page).ceil();

    return num_of_pages as Page;
}

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
