
/* fundamentals */

pub type UnixTimestamp = i64;

pub type Count = u64; // type for counting things

// pagination
// TODO: need compile-time check to ensure >= 1 constraint (rust doesn't support this yet)
pub type Page = u64;
pub type PerPage = u64;

#[derive(Debug, Clone)]
pub enum Search {
    NoQuery,
    Query(String)
}

#[derive(Debug, Clone)]
pub enum SortOrder {
    Ascending,
    Descending
}

#[derive(Debug, Clone)]
pub enum DecksPageSort {
    DeckTitle(SortOrder),
    CreatedAt(SortOrder),
    UpdatedAt(SortOrder),
    // last time user reviewed this deck;
    // not based on the cards the deck contains
    ReviewedAt(SortOrder)
}

#[derive(Debug, Clone)]
pub enum DecksPageQuery {
    NoQuery,
    Query(Page, DecksPageSort)
}

// TODO: migrate some types from route.rs to here
