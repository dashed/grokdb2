/* rust lib imports */

use std::cell::RefCell;
use std::rc::Rc;

/* 3rd-party imports */

use rusqlite::Connection;
use rusqlite::types::ToSql;
use rusqlite::Error as SqliteError;

/* local imports */

use context::Context;
use types::{UnixTimestamp, CardID, DeckID, DecksPageQuery, Search, ItemCount};
use errors::RawAPIError;
use constants;

/* ////////////////////////////////////////////////////////////////////////// */

#[derive(Debug, Clone)]
pub struct Card {
    pub id: CardID,
    pub title: String,

    pub question: String,
    pub answer: String,
    pub description: String,

    pub created_at: UnixTimestamp,
    pub updated_at: UnixTimestamp,

    pub deck_id: DeckID,

    pub is_active: bool
}
