/* rust lib imports */

use std::sync::{Arc, Mutex, LockResult, MutexGuard, RwLock};

/* 3rd-party imports */

use rusqlite::Connection;
use hyper::header::{Headers, ContentType, TransferEncoding};

/// /////////////////////////////////////////////////////////////////////////////

/* Types */

pub type UnixTimestamp = i64;

// TODO: change to u64? it's i64 b/c sqlite requires it.
pub type DeckID = i64;
pub type CardID = i64;
pub type StashID = i64;

// pagination
// TODO: need compile-time check to ensure >= 1 constraint (rust doesn't support this yet)
pub type Page = u64;
pub type PerPage = u64;

#[derive(Debug)]
pub enum Search {
    NoQuery,
    Query(String),
}

#[derive(Debug)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Debug)]
pub enum DecksPageQuery {
    NoQuery,
    Query(Page, DecksPageSort),
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
