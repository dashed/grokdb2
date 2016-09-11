/* rust lib imports */

use std::sync::{Arc, Mutex, RwLock};

/* 3rd-party imports */

use rusqlite::Connection;

/* local imports */

use errors::RawAPIError;
use database::Database;

/// /////////////////////////////////////////////////////////////////////////////

const SETUP: [&'static str; 26] = [
    // configs
    CONFIGS,

    // configs/triggers
    CONFIG_ON_UPDATE_TRIGGER,

    // decks
    DECKS,
    DECKSCLOSURE,

    // decks/indices
    DECKSCLOSURE_DEPTH_INDEX,

    // decks/triggers
    DECK_ON_UPDATE_TRIGGER,
    DECKSCLOSURE_NEW_DECK_TRIGGER,

    // cards
    CARDS,

    // cards/indices
    CARD_ID_INDEX,

    // cards/triggers
    UPDATED_CARD_TRIGGER,

    // cards score
    CARDS_SCORE,

    // cards score/triggers
    CARDS_SCORE_ON_NEW_CARD_TRIGGER,

    // cards score/indices
    CARDS_SCORE_INDEX,

    // cards score history
    CARDS_SCORE_HISTORY,

    // cards score history/triggers
    SNAPSHOT_CARDS_SCORE_ON_UPDATED_TRIGGER,

    // cards score history/indices
    CARDS_SCORE_HISTORY_CARD_INDEX,
    CARDS_SCORE_HISTORY_OCCURRED_AT_INDEX,

    // stashes
    STASHES,
    STASHES_CARDS,

    // stashes/triggers
    STASHES_ON_UPDATE_TRIGGER,

    // review
    CACHED_DECK_REVIEW,
    CACHED_STASH_REVIEW,

    // FTS3/4 full-text searching sqlite module
    CARD_SEARCH_INDEX,
    CARD_SEARCH_FIRST_INDEX_TRIGGER,
    CARD_SEARCH_DELETE_INDEX_TRIGGER,
    CARD_SEARCH_UPDATE_INDEX_TRIGGER
];

/**
 * All SQL comply with syntax supported with SQLite v3.9.1
 */

/* configs */

// note: CHECK (setting <> '') ensures setting is non-empty string
const CONFIGS: &'static str = "
CREATE TABLE IF NOT EXISTS Configs (
    setting TEXT PRIMARY KEY NOT NULL,
    value TEXT,

    created_at INT NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INT NOT NULL DEFAULT (strftime('%s', 'now')),

    CHECK (setting <> '')
);
";

const CONFIG_ON_UPDATE_TRIGGER: &'static str = "
CREATE TRIGGER IF NOT EXISTS CONFIG_ON_UPDATE_TRIGGER
AFTER UPDATE OF
    setting, value
ON Configs
BEGIN
    UPDATE Configs SET updated_at = strftime('%s', 'now') WHERE setting = NEW.setting;
END;
";


/* decks */

// note: updated_at is when the deck was modified, not when it was reviewed.
// note: CHECK (name <> '') ensures name is non-empty string
const DECKS: &'static str = "
CREATE TABLE IF NOT EXISTS Decks (
    deck_id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',

    created_at INT NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INT NOT NULL DEFAULT (strftime('%s', 'now')),
    reviewed_at INT NOT NULL DEFAULT (strftime('%s', 'now')),

    CHECK (name <> '')
);
";

const DECK_ON_UPDATE_TRIGGER: &'static str = "
CREATE TRIGGER IF NOT EXISTS DECK_ON_UPDATE_TRIGGER
AFTER UPDATE OF
    name, description
ON Decks
BEGIN
    UPDATE Decks SET updated_at = strftime('%s', 'now') WHERE deck_id = NEW.deck_id;
END;
";

// description of the closure table from:
// - https://pragprog.com/titles/bksqla/sql-antipatterns
// - http://dirtsimple.org/2010/11/simplest-way-to-do-tree-based-queries.html
//
// allows nested decks
const DECKSCLOSURE: &'static str = "
CREATE TABLE IF NOT EXISTS DecksClosure (
    ancestor INTEGER NOT NULL,
    descendent INTEGER NOT NULL,
    depth INTEGER NOT NULL,
    PRIMARY KEY(ancestor, descendent),
    FOREIGN KEY (ancestor) REFERENCES Decks(deck_id) ON DELETE CASCADE,
    FOREIGN KEY (descendent) REFERENCES Decks(deck_id) ON DELETE CASCADE
);
";

const DECKSCLOSURE_DEPTH_INDEX: &'static str = "
CREATE INDEX IF NOT EXISTS DECKSCLOSURE_DEPTH_INDEX
ON DecksClosure (depth DESC);
";

// any and all node Decks are an/a ancestor/descendent of itself.
const DECKSCLOSURE_NEW_DECK_TRIGGER: &'static str = "
CREATE TRIGGER IF NOT EXISTS DECKSCLOSURE_NEW_DECK_TRIGGER
AFTER INSERT
ON Decks
BEGIN
    INSERT OR IGNORE INTO DecksClosure(ancestor, descendent, depth) VALUES (NEW.deck_id, NEW.deck_id, 0);
END;
";

/* cards */

// note: updated_at is when the card was modified. not when it was seen.
// note: CHECK (title <> '') ensures title is non-empty string
const CARDS: &'static str = "
CREATE TABLE IF NOT EXISTS Cards (
    card_id INTEGER PRIMARY KEY NOT NULL,

    title TEXT NOT NULL,

    description TEXT NOT NULL DEFAULT '',
    question TEXT NOT NULL DEFAULT '',
    answer TEXT NOT NULL DEFAULT '',

    created_at INT NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INT NOT NULL DEFAULT (strftime('%s', 'now')),

    deck_id INTEGER NOT NULL,

    is_active BOOLEAN NOT NULL DEFAULT 1,

    CHECK (title <> ''),
    FOREIGN KEY (deck_id) REFERENCES Decks(deck_id) ON DELETE CASCADE
);
";

const CARD_ID_INDEX: &'static str = "
CREATE INDEX IF NOT EXISTS CARD_ID_INDEX
ON Cards (deck_id);
";

const UPDATED_CARD_TRIGGER: &'static str = "
CREATE TRIGGER IF NOT EXISTS UPDATED_CARD_TRIGGER
AFTER UPDATE OF
    title, description, question, answer, deck_id
ON Cards
BEGIN
    UPDATE Cards SET updated_at = strftime('%s', 'now') WHERE card_id = NEW.card_id;
END;
";

/* cards score */

// - changelog is internal for CardsScoreHistory to take snapshot of.
// - times_seen is number of times a card was put up for review.
// - times_reviewed is number of times a card was actually reviewed.
// - note that, skipping a card is not actually reviewing the card.
// - default value of 'review_after' is 3 hours == 10800 seconds
//
// - card will be eligible for review after seen_at + review_after >= strftime('%s', 'now')
// rational:
// - skipping a card will update seen_at, but not reviewed_at
// - reviewed_at_count is the Nth review that the user reviewed the card
const CARDS_SCORE: &'static str = "
CREATE TABLE IF NOT EXISTS CardsScore (

    changelog TEXT NOT NULL DEFAULT '',

    success INTEGER NOT NULL DEFAULT 0,
    fail INTEGER NOT NULL DEFAULT 0,

    times_reviewed INT NOT NULL DEFAULT 0,
    times_seen INT NOT NULL DEFAULT 0,

    seen_at INT NOT NULL DEFAULT (strftime('%s', 'now')),
    reviewed_at INT NOT NULL DEFAULT (strftime('%s', 'now')),

    reviewed_at_count INT NOT NULL DEFAULT 0,

    review_after INT NOT NULL DEFAULT 10800,
    cards_till_ready_for_review INT NOT NULL DEFAULT 0,

    card_id INTEGER NOT NULL,

    PRIMARY KEY(card_id),

    FOREIGN KEY (card_id) REFERENCES Cards(card_id) ON DELETE CASCADE
);
";

const CARDS_SCORE_ON_NEW_CARD_TRIGGER: &'static str = "
CREATE TRIGGER IF NOT EXISTS CARDS_SCORE_ON_NEW_CARD_TRIGGER
AFTER INSERT
ON Cards
BEGIN
    INSERT OR IGNORE INTO CardsScore(card_id) VALUES (NEW.card_id);
END;
";

// enforce 1-1 relationship
const CARDS_SCORE_INDEX: &'static str = "
CREATE UNIQUE INDEX IF NOT EXISTS CARDS_SCORE_INDEX ON CardsScore (card_id);
";

/* cards score history */

// changelog is internal for CardsScoreHistory to take snapshot of
const CARDS_SCORE_HISTORY: &'static str = "
CREATE TABLE IF NOT EXISTS CardsScoreHistory (

    occurred_at INT NOT NULL DEFAULT (strftime('%s', 'now')),

    is_review_event INT NOT NULL DEFAULT 0,

    success INTEGER NOT NULL DEFAULT 0,
    fail INTEGER NOT NULL DEFAULT 0,

    total_success INTEGER NOT NULL DEFAULT 0,
    total_fail INTEGER NOT NULL DEFAULT 0,

    changelog TEXT NOT NULL DEFAULT '',

    card_id INTEGER NOT NULL,

    FOREIGN KEY (card_id) REFERENCES Cards(card_id) ON DELETE CASCADE
);
";

const SNAPSHOT_CARDS_SCORE_ON_UPDATED_TRIGGER: &'static str = "
CREATE TRIGGER IF NOT EXISTS SNAPSHOT_CARDS_SCORE_ON_UPDATED_TRIGGER
AFTER UPDATE
OF success, fail, changelog
ON CardsScore
BEGIN
    INSERT INTO CardsScoreHistory(
        is_review_event,
        occurred_at,
        success,
        fail,
        total_success,
        total_fail,
        changelog,
        card_id
    )
    VALUES (
        NEW.reviewed_at <> OLD.reviewed_at,
        strftime('%s', 'now'),
        max(NEW.success - OLD.success, 0),
        max(NEW.fail - OLD.fail, 0),
        NEW.success,
        NEW.fail,
        NEW.changelog,
        NEW.card_id
    );
END;
";

const CARDS_SCORE_HISTORY_CARD_INDEX: &'static str = "
CREATE INDEX IF NOT EXISTS CARDS_SCORE_HISTORY_CARD_INDEX
ON CardsScoreHistory (card_id);
";

const CARDS_SCORE_HISTORY_OCCURRED_AT_INDEX: &'static str = "
CREATE INDEX IF NOT EXISTS CARDS_SCORE_HISTORY_OCCURRED_AT_INDEX
ON CardsScoreHistory (occurred_at DESC);
";

/* stashes */

// note: updated_at is when the stash was modified, not when it was reviewed.
// note: CHECK (name <> '') ensures name is non-empty string
const STASHES: &'static str = "
CREATE TABLE IF NOT EXISTS Stashes (
    stash_id INTEGER PRIMARY KEY NOT NULL,

    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',

    created_at INT NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INT NOT NULL DEFAULT (strftime('%s', 'now')),
    reviewed_at INT NOT NULL DEFAULT (strftime('%s', 'now')),

    CHECK (name <> '')
);
";

// cards that belong to a stash
const STASHES_CARDS: &'static str = "
CREATE TABLE IF NOT EXISTS StashCards (

    stash_id INTEGER NOT NULL,
    card_id INTEGER NOT NULL,

    added_at INT NOT NULL DEFAULT (strftime('%s', 'now')),

    PRIMARY KEY(stash_id, card_id),

    FOREIGN KEY (stash_id) REFERENCES Stashes(stash_id) ON DELETE CASCADE,
    FOREIGN KEY (card_id) REFERENCES Cards(card_id) ON DELETE CASCADE
);
";

const STASHES_ON_UPDATE_TRIGGER: &'static str = "
CREATE TRIGGER IF NOT EXISTS STASHES_ON_UPDATE_TRIGGER
AFTER UPDATE OF
    name, description
ON Stashes
BEGIN
    UPDATE Stashes SET updated_at = strftime('%s', 'now') WHERE stash_id = NEW.stash_id;
END;
";

/* review */

const CACHED_DECK_REVIEW: &'static str = "
CREATE TABLE IF NOT EXISTS CachedDeckReview (
    deck_id INTEGER NOT NULL,
    card_id INTEGER NOT NULL,
    cached_review_procedure TEXT NOT NULL DEFAULT '',

    created_at INT NOT NULL DEFAULT (strftime('%s', 'now')),

    PRIMARY KEY(deck_id),

    FOREIGN KEY (deck_id) REFERENCES Decks(deck_id) ON DELETE CASCADE,
    FOREIGN KEY (card_id) REFERENCES Cards(card_id) ON DELETE CASCADE
);
";

const CACHED_STASH_REVIEW: &'static str = "
CREATE TABLE IF NOT EXISTS CachedStashReview (
    stash_id INTEGER NOT NULL,
    card_id INTEGER NOT NULL,
    created_at INT NOT NULL DEFAULT (strftime('%s', 'now')),

    PRIMARY KEY(stash_id),

    FOREIGN KEY (stash_id) REFERENCES Stashes(stash_id) ON DELETE CASCADE,
    FOREIGN KEY (card_id) REFERENCES Cards(card_id) ON DELETE CASCADE
);
";

const CARD_SEARCH_INDEX: &'static str = "
CREATE VIRTUAL TABLE IF NOT EXISTS
    CardsFTS
USING fts4(
    title TEXT,
    description TEXT,
    question TEXT,
    answer TEXT
);
";

const CARD_SEARCH_FIRST_INDEX_TRIGGER: &'static str = "
CREATE TRIGGER IF NOT EXISTS CARD_SEARCH_FIRST_INDEX_TRIGGER
AFTER INSERT
ON Cards
BEGIN
    INSERT OR REPLACE INTO CardsFTS(docid, title, description, question, answer)
    VALUES (NEW.card_id, NEW.title, NEW.description, NEW.question, NEW.answer);
END;
";

const CARD_SEARCH_DELETE_INDEX_TRIGGER: &'static str = "
CREATE TRIGGER IF NOT EXISTS CARD_SEARCH_DELETE_INDEX_TRIGGER
AFTER DELETE
ON Cards
BEGIN
    DELETE FROM CardsFTS WHERE docid = OLD.card_id;
END;
";

const CARD_SEARCH_UPDATE_INDEX_TRIGGER: &'static str = "
CREATE TRIGGER IF NOT EXISTS CARD_SEARCH_UPDATE_INDEX_TRIGGER
AFTER UPDATE OF
title, description, question, answer, deck
ON Cards
BEGIN
    INSERT OR REPLACE INTO CardsFTS(docid, title, description, question, answer)
    VALUES (NEW.card_id, NEW.title, NEW.description, NEW.question, NEW.answer);
END;
";

pub fn setup_database(db_connection: Database) -> Result<(), RawAPIError> {

    db_write_lock!(db_conn; db_connection);
    let db_conn: &Connection = db_conn;

    // execute every table setup query
    for query in SETUP.into_iter() {

        match db_conn.execute_batch(query) {
            Err(sqlite_error) => {
                return Err(RawAPIError::SQLError(sqlite_error, query.to_string()));
            }
            _ => {
                /* query sucessfully executed */
            }
        }
    }

    return Ok(());
}
