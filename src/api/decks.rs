/* rust lib imports */

use std::cell::RefCell;
use std::rc::Rc;

/* 3rd-party imports */

use rusqlite::Connection;
use rusqlite::types::ToSql;
use rusqlite::Error as SqliteError;
use rand::{thread_rng, Rng};
use serde_json;

/* local imports */

use context::Context;
use types::{UnixTimestamp, DeckID, CardID, DecksPageQuery, Search, ItemCount, Offset};
use errors::RawAPIError;
use constants;
use api::review::{Reviewable, ActiveSelection, CachedReviewProcedure};
use api::cards;

/* ////////////////////////////////////////////////////////////////////////// */

#[derive(Debug, Clone)]
pub struct Deck {
    pub id: DeckID,
    pub name: String,
    pub description: String,
    pub created_at: UnixTimestamp, // unix timestamp
    pub updated_at: UnixTimestamp, // unix timestamp
    pub reviewed_at: UnixTimestamp, // unix timestamp
    pub has_reviewed: bool, // false if reviewed_at == created_at, otherwise true
}

// struct for requesting to create a deck
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateDeck {
    pub name: String, // required
    pub description: String, // required, but may be empty
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateDeckDescription {
    pub description: String // required, but may be empty
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateDeckName {
    pub name: String // required
}

#[derive(Debug, Serialize)]
pub struct DeckCreateResponse {
    pub profile_url: String
}

#[inline]
pub fn get_deck(context: Rc<RefCell<Context>>, deck_id: DeckID) -> Result<Deck, RawAPIError> {

    {
        let context = context.borrow();
        if context.should_cache && context.decks.len() > 0 {
            match context.decks.get(&deck_id) {
                None => {},
                Some(deck) => {
                    return Ok(deck.clone());
                }
            }
        }
    };

    assert!(context.borrow().is_read_locked());

    let query = format!("
        SELECT
            deck_id,
            name,
            description,
            created_at,
            updated_at,
            reviewed_at
        FROM Decks
        WHERE deck_id = {deck_id}
        LIMIT 1;
    ", deck_id = deck_id);

    let results = {
        let context = context.borrow();
        db_read_lock!(db_conn; context.database());
        let db_conn: &Connection = db_conn;

        let results = db_conn.query_row(&query, &[], |row| -> Deck {

            let created_at: UnixTimestamp = row.get(3);
            let reviewed_at: UnixTimestamp = row.get(5);

            return Deck {
                id: row.get(0),
                name: row.get(1),
                description: row.get(2),
                created_at: created_at,
                updated_at: row.get(4),
                reviewed_at: reviewed_at,
                has_reviewed: created_at != reviewed_at,
            };
        });

        results
    };

    match results {
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        }
        Ok(deck) => {

            let mut context = context.borrow_mut();

            if context.should_cache {
                context.decks.insert(deck.id, deck.clone());
            }

            return Ok(deck);
        }
    };
}

#[inline]
pub fn deck_exists(context: Rc<RefCell<Context>>, deck_id: DeckID) -> Result<bool, RawAPIError> {

    {
        let context = context.borrow();
        if context.should_cache && context.decks.len() > 0 {
            return Ok(context.decks.contains_key(&deck_id));
        }
    };

    assert!(context.borrow().is_read_locked());

    let query = format!("
        SELECT
            COUNT(1)
        FROM Decks
        WHERE deck_id = {deck_id}
        LIMIT 1;
    ", deck_id = deck_id);

    let context = context.borrow();
    db_read_lock!(db_conn; context.database());
    let db_conn: &Connection = db_conn;

    let deck_exists = db_conn.query_row(&query, &[], |row| -> bool {
        let count: i64 = row.get(0);
        return count >= 1;
    });

    match deck_exists {
        Ok(deck_exists) => return Ok(deck_exists),
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        }
    }
}

#[inline]
pub fn create_deck(context: Rc<RefCell<Context>>, create_deck_request: CreateDeck) -> Result<Deck, RawAPIError> {

    assert!(context.borrow().is_write_locked());

    let query = "INSERT INTO Decks(name, description) VALUES (:name, :description);";

    let params: &[(&str, &ToSql)] = &[(":name", &create_deck_request.name.clone()),
                                      (":description", &create_deck_request.description.clone())];

    let deck_id: DeckID = {

        let context = context.borrow();
        db_write_lock!(db_conn; context.database());
        let db_conn: &Connection = db_conn;

        match db_conn.execute_named(query, params) {
            Err(sqlite_error) => {
                return Err(RawAPIError::SQLError(sqlite_error, query.to_string()));
            }
            _ => {
                /* query sucessfully executed */
            }
        }

        let row_id = db_conn.last_insert_rowid();
        row_id
    };

    return get_deck(context, deck_id);
}

#[inline]
pub fn update_deck_description(
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    update_deck_description_request: UpdateDeckDescription) -> Result<(), RawAPIError> {

    assert!(context.borrow().is_write_locked());

    let query = format!("
        UPDATE Decks
        SET
        description = :description
        WHERE deck_id = {deck_id};
    ", deck_id = deck_id);

    let params: &[(&str, &ToSql)] = &[(":description", &update_deck_description_request.description.clone())];

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

#[inline]
pub fn update_deck_name(
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    update_deck_name_request: UpdateDeckName) -> Result<(), RawAPIError> {

    assert!(context.borrow().is_write_locked());

    let query = format!("
        UPDATE Decks
        SET
        name = :name
        WHERE deck_id = {deck_id};
    ", deck_id = deck_id);

    let params: &[(&str, &ToSql)] = &[(":name", &update_deck_name_request.name.clone())];

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

#[inline]
pub fn connect_decks(context: Rc<RefCell<Context>>, child: DeckID, parent: DeckID) -> Result<(), RawAPIError> {

    assert!(context.borrow().is_write_locked());

    // moving a child deck subtree consists of two procedures:
    // 1. delete any and all subtree connections between child (and its descendants)
    //    and the child's ancestors
    let query_delete = format!("
        DELETE FROM DecksClosure

        /* select all descendents of child */
        WHERE descendent IN (
            SELECT descendent
            FROM DecksClosure
            WHERE ancestor = {child}
        )
        AND

        /* select all ancestors of child but not child itself */
        ancestor IN (
            SELECT ancestor
            FROM DecksClosure
            WHERE descendent = {child}
            AND ancestor != descendent
        )
        AND descendent != ancestor;
    ", child = child);

    let context = context.borrow();
    db_write_lock!(db_conn; context.database());
    let db_conn: &Connection = db_conn;

    match db_conn.execute(&query_delete, &[]) {
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query_delete));
        },
        _ => {/* query sucessfully executed */},
    }

    // 2. make parent (and its ancestors) be ancestors of child deck (and its descendants)
    let query_insert = format!("
        INSERT OR IGNORE INTO DecksClosure(ancestor, descendent, depth)
        SELECT p.ancestor, c.descendent, p.depth+c.depth+1
            FROM DecksClosure AS p, DecksClosure AS c
        WHERE
            c.ancestor = {child}
            AND p.descendent = {parent};
    ", child = child, parent = parent);

    match db_conn.execute(&query_insert, &[]) {
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query_insert));
        },
        _ => {/* query sucessfully executed */},
    }

    return Ok(());
}

#[inline]
pub fn get_parent_id_of_deck(context: Rc<RefCell<Context>>, child: DeckID) -> Result<Option<DeckID>, RawAPIError> {

    assert!(context.borrow().is_read_locked());

    let query = format!("
        SELECT
            ancestor
        FROM DecksClosure
        WHERE
        descendent = {deck_id}
        AND depth = 1
        LIMIT 1;
    ", deck_id = child);

    let context = context.borrow();
    db_read_lock!(db_conn; context.database());
    let db_conn: &Connection = db_conn;

    let results = db_conn.query_row(&query, &[], |row| -> DeckID {
        return row.get(0);
    });

    match results {
        Err(sqlite_error) => {

            match sqlite_error {
                SqliteError::QueryReturnedNoRows => {
                    return Ok(None);
                }
                _ => {}
            };

            return Err(RawAPIError::SQLError(sqlite_error, query));
        }
        Ok(parent) => {
            return Ok(Some(parent));
        }
    };
}

#[inline]
pub fn get_path_of_deck(context: Rc<RefCell<Context>>, deck_id: DeckID) -> Result<Vec<DeckID>, RawAPIError> {

    assert!(context.borrow().is_read_locked());

    let query = format!("
        SELECT
            ancestor
        FROM DecksClosure
        WHERE
            descendent = {deck_id}
        AND
            depth >= 0
        ORDER BY
            depth DESC;
    ", deck_id = deck_id);

    let context = context.borrow();
    db_read_lock!(db_conn; context.database());
    let db_conn: &Connection = db_conn;

    let mut statement = match db_conn.prepare(&query) {
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        },
        Ok(statement) => statement
    };

    let maybe_iter = statement.query_map(&[], |row| -> DeckID {
        return row.get(0);
    });

    match maybe_iter {
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        },
        Ok(iter) => {

            let mut vec_of_deck_ids: Vec<DeckID> = Vec::new();

            for maybe_deck_id in iter {

                let item = match maybe_deck_id {
                    Err(sqlite_error) => {
                        return Err(RawAPIError::SQLError(sqlite_error, query));
                    },
                    Ok(item) => item
                };

                vec_of_deck_ids.push(item);

            }

            return Ok(vec_of_deck_ids);
        }
    };
}

#[inline]
pub fn get_deck_children_total_count(
    context: Rc<RefCell<Context>>,
    deck_id: DeckID) -> Result<ItemCount, RawAPIError> {

    // TODO: support search
    // TODO: cache does not support searching

    {
        let context = context.borrow();
        if context.should_cache && context.deck_children_count.len() > 0 {
            match context.deck_children_count.get(&deck_id) {
                None => {},
                Some(count) => {
                    return Ok(*count);
                }
            }
        }
    };

    assert!(context.borrow().is_read_locked());

    let query = format!(indoc!("
        SELECT
            COUNT(1)
        FROM
            DecksClosure
        INNER JOIN
            Decks
        ON DecksClosure.descendent = Decks.deck_id
        WHERE
            ancestor = {deck_id}
        AND
            depth = 1"), deck_id = deck_id);

    let result = {
        let context = context.borrow();
        db_read_lock!(db_conn; context.database());
        let db_conn: &Connection = db_conn;

        let result = db_conn.query_row(&query, &[], |row| -> i64 {
            return row.get(0);
        });
        result
    };

    match result {
        Ok(count) => {
            // TODO: dev mode
            assert!(count >= 0);

            let count = count as ItemCount;

            let mut context = context.borrow_mut();

            if context.should_cache {
                context.deck_children_count.insert(deck_id, count);
            }

            return Ok(count)
        },
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        }
    }
}

#[inline]
pub fn get_deck_children(
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    deck_page_query: &DecksPageQuery,
    search: &Search) -> Result<Vec<DeckID>, RawAPIError> {

    assert!(context.borrow().is_read_locked());

    let offset = deck_page_query.get_offset();
    let per_page = deck_page_query.get_per_page();

    // TODO: cache support?

    // TODO: search
    // TODO: sort

    let select_sql = indoc!("
        SELECT
            DecksClosure.descendent
        FROM
            DecksClosure
        INNER JOIN
            Decks
        ON DecksClosure.descendent = Decks.deck_id");

    let where_order_sql = format!(indoc!("
            ancestor = {deck_id}
        AND
            depth = 1
        ORDER BY
            Decks.name
        COLLATE NOCASE ASC"), deck_id = deck_id);

    let query = pagination!(select_sql; select_sql; where_order_sql; "DecksClosure.descendent"; per_page; offset);

    let context = context.borrow();
    db_read_lock!(db_conn; context.database());
    let db_conn: &Connection = db_conn;

    let mut statement = match db_conn.prepare(&query) {
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        },
        Ok(statement) => statement
    };

    let maybe_iter = statement.query_map(&[], |row| -> DeckID {
        return row.get(0);
    });

    match maybe_iter {
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        },
        Ok(iter) => {

            let mut vec_of_deck_ids: Vec<DeckID> = Vec::new();

            for maybe_deck_id in iter {

                let item = match maybe_deck_id {
                    Err(sqlite_error) => {
                        return Err(RawAPIError::SQLError(sqlite_error, query));
                    },
                    Ok(item) => item
                };

                vec_of_deck_ids.push(item);

            }

            return Ok(vec_of_deck_ids);
        }
    }
}

// TODO: test
impl Reviewable for Deck {

    /* caching */

    fn get_cached_card(&self, context: Rc<RefCell<Context>>)
        -> Result<Option<(CardID, Option<CachedReviewProcedure>)>, RawAPIError> {

        match cards::cached_review_card_for_deck(context.clone(), self.id) {
            Err(why) => {
                return Err(why);
            },
            Ok(None) => {
                return Ok(None);
            },
            Ok(Some((card_id, cached_review_procedure))) => {

                // check if the card is still within the deck

                match cards::is_card_in_deck(context.clone(), card_id, self.id) {
                    Err(why) => {
                        return Err(why);
                    },
                    Ok(true) => {
                        return Ok(Some((card_id, cached_review_procedure)));
                    },
                    Ok(false) => {

                        match self.remove_cache(context) {
                            Err(why) => {
                                return Err(why);
                            },
                            Ok(_) => {
                                return Ok(None);
                            }
                        }
                    }
                }
            }
        }
    }

    // remove card review entry by deck
    fn remove_cache(&self, context: Rc<RefCell<Context>>) -> Result<(), RawAPIError> {

        assert!(context.borrow().is_write_locked());

        let query = format!(indoc!("
            DELETE FROM
                CachedDeckReview
            WHERE deck_id = {deck_id};
        "), deck_id = self.id);

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

    fn set_cache_card(&self,
        context: Rc<RefCell<Context>>,
        card_id: CardID,
        cached_review_procedure: CachedReviewProcedure)
        -> Result<(), RawAPIError> {

        assert!(context.borrow().is_write_locked());

        let cached_review_procedure: String = match serde_json::to_string(&cached_review_procedure) {
            Ok(cached_review_procedure) => cached_review_procedure.to_string(),
            Err(_) => {

                // TODO: internal error reporting

                "".to_string()
            }
        };

        let query = format!("
            INSERT OR REPLACE INTO CachedDeckReview(deck_id, card_id, cached_review_procedure)
            VALUES (:deck_id, :card_id, :cached_review_procedure);
        ");

        let params: &[(&str, &ToSql)] = &[
            (":deck_id", &(self.id)),
            (":card_id", &card_id),
            (":cached_review_procedure", &cached_review_procedure)
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

    /* new cards */

    fn have_new_cards(&self,
        context: Rc<RefCell<Context>>,
        active_selection: &ActiveSelection) -> Result<bool, RawAPIError> {
        return cards::deck_have_new_cards_for_review(context, self.id, active_selection);
    }

    fn num_of_new_cards_for_review(&self,
        context: Rc<RefCell<Context>>,
        active_selection: &ActiveSelection) -> Result<ItemCount, RawAPIError> {
        return cards::deck_num_of_new_cards_for_review(context, self.id, active_selection);
    }

    fn get_new_card_for_review(&self,
        context: Rc<RefCell<Context>>,
        active_selection: &ActiveSelection,
        card_idx: Offset) -> Result<CardID, RawAPIError> {
        return cards::deck_get_new_card_for_review(context, self.id, active_selection, card_idx);
    }

    /* cards ready for review */

    fn have_cards_ready_for_review(&self,
        context: Rc<RefCell<Context>>,
        active_selection: &ActiveSelection) -> Result<bool, RawAPIError> {
        return cards::deck_have_cards_ready_for_review(context, self.id, active_selection);
    }

    fn num_of_cards_ready_for_review(&self,
        context: Rc<RefCell<Context>>,
        active_selection: &ActiveSelection) -> Result<ItemCount, RawAPIError> {
        return cards::deck_num_of_cards_ready_for_review(context, self.id, active_selection);
    }

    fn get_card_ready_for_review(&self,
        context: Rc<RefCell<Context>>,
        active_selection: &ActiveSelection,
        card_idx: Offset) -> Result<CardID, RawAPIError> {
        return cards::deck_get_card_ready_for_review(context, self.id, active_selection, card_idx);
    }

    /* least recently reviewed */

    fn have_cards_for_review(&self, context: Rc<RefCell<Context>>,
        active_selection: &ActiveSelection) -> Result<bool, RawAPIError> {
        return cards::deck_have_cards_for_review(context, self.id, active_selection);
    }

    fn deck_num_of_cards_for_review(&self,
        context: Rc<RefCell<Context>>,
        active_selection: &ActiveSelection) -> Result<ItemCount, RawAPIError> {
        return cards::deck_num_of_cards_for_review(context.clone(), self.id, active_selection);
    }

    fn get_least_recently_reviewed_card(&self,
        context: Rc<RefCell<Context>>,
        active_selection: &ActiveSelection,
        card_idx: Offset) -> Result<CardID, RawAPIError> {
        return cards::deck_get_least_recently_reviewed_card(context, self.id, active_selection, card_idx);
    }

}

#[test]
fn decks_test() {

    /* imports */

    use std::sync::{Arc, RwLock};
    use std::fs;
    use database;
    use api::decks;
    use context;

    /* setup */

    let file_path = "test/assets/decks_test.db".to_string();
    fs::remove_file(file_path.clone());

    let db_connection = database::get_database(file_path.clone());
    let global_lock = Arc::new(RwLock::new(db_connection));

    // deck doesn't exist

    {
        let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
        let _guard = context::read_lock(context.clone());
        match decks::get_deck(context, 1) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    };

    // create deck

    {
        // case: don't allow empty deck name

        let request = CreateDeck {
            name: format!(""),
            description: format!(""),
        };

        let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
        let _guard = context::write_lock(context.clone());
        match decks::create_deck(context, request) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    };

    {
        // case: add new deck

        {
            let request = CreateDeck {
                name: format!("Foo"),
                description: format!(""),
            };

            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::write_lock(context.clone());
            match decks::create_deck(context, request) {
                Ok(actual) => {
                    assert_eq!(actual.id, 1);
                    assert_eq!(actual.name, format!("Foo"));
                    assert_eq!(actual.description, format!(""));
                    assert_eq!(actual.created_at, actual.updated_at);
                    assert_eq!(actual.created_at, actual.reviewed_at);
                    assert_eq!(actual.has_reviewed, false);
                },
                Err(_) => assert!(false),
            }
        };

        {
            let request = CreateDeck {
                name: format!("Bar"),
                description: format!("Amazing description of this deck."),
            };

            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::write_lock(context.clone());
            match decks::create_deck(context, request) {
                Ok(actual) => {
                    assert_eq!(actual.id, 2); // ensure increment
                    assert_eq!(actual.name, format!("Bar"));
                    assert_eq!(actual.description, format!("Amazing description of this deck."));
                    assert_eq!(actual.created_at, actual.updated_at);
                    assert_eq!(actual.created_at, actual.reviewed_at);
                    assert_eq!(actual.has_reviewed, false);
                },
                Err(_) => assert!(false),
            }
        };
    };

    {
        // case: retrieve created decks

        {
            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::read_lock(context.clone());
            match decks::get_deck(context.clone(), 2) {
                Ok(actual) => {
                    assert_eq!(actual.id, 2);
                    assert_eq!(actual.name, format!("Bar"));
                    assert_eq!(actual.description, format!("Amazing description of this deck."));
                    assert_eq!(actual.created_at, actual.updated_at);
                    assert_eq!(actual.created_at, actual.reviewed_at);
                    assert_eq!(actual.has_reviewed, false);
                },
                Err(_) => assert!(false),
            }

            // ensure nothing was cached
            assert_eq!(context.borrow().should_cache, false);
            assert_eq!(context.borrow().decks.len(), 0);
        };

        {
            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::read_lock(context.clone());
            match decks::get_deck(context.clone(), 1) {
                Ok(actual) => {
                    assert_eq!(actual.id, 1);
                    assert_eq!(actual.name, format!("Foo"));
                    assert_eq!(actual.description, format!(""));
                    assert_eq!(actual.created_at, actual.updated_at);
                    assert_eq!(actual.created_at, actual.reviewed_at);
                    assert_eq!(actual.has_reviewed, false);
                },
                Err(_) => assert!(false),
            }

            // ensure nothing was cached
            assert_eq!(context.borrow().should_cache, false);
            assert_eq!(context.borrow().decks.len(), 0);
        };

        {
            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            (*context.borrow_mut()).should_cache = true;
            let _guard = context::read_lock(context.clone());
            match decks::get_deck(context.clone(), 1) {
                Ok(actual) => {
                    assert_eq!(actual.id, 1);
                    assert_eq!(actual.name, format!("Foo"));
                    assert_eq!(actual.description, format!(""));
                    assert_eq!(actual.created_at, actual.updated_at);
                    assert_eq!(actual.created_at, actual.reviewed_at);
                    assert_eq!(actual.has_reviewed, false);
                },
                Err(_) => assert!(false),
            }

            // ensure deck was cached
            assert_eq!(context.borrow().should_cache, true);
            assert_eq!(context.borrow().decks.len(), 1);

            // cache check

            let context = context.borrow();
            let actual = context.decks.get(&1).unwrap();
            assert_eq!(actual.id, 1);
            assert_eq!(actual.name, format!("Foo"));
            assert_eq!(actual.description, format!(""));
            assert_eq!(actual.created_at, actual.updated_at);
            assert_eq!(actual.created_at, actual.reviewed_at);
            assert_eq!(actual.has_reviewed, false);
        };
    };

    {

        // case: update deck description

        {
            let request = UpdateDeckDescription {
                description: "this is a description".to_string()
            };

            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::write_lock(context.clone());
            match decks::update_deck_description(context, 1, request) {
                Ok(_) => assert!(true),
                Err(_) => assert!(false),
            };
        };

        {
            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::read_lock(context.clone());
            match decks::get_deck(context.clone(), 1) {
                Ok(actual) => {
                    assert_eq!(actual.id, 1);
                    assert_eq!(actual.name, format!("Foo"));
                    assert_eq!(actual.description, format!("this is a description"));
                    assert_eq!(actual.created_at, actual.updated_at);
                    assert_eq!(actual.created_at, actual.reviewed_at);
                    assert_eq!(actual.has_reviewed, false);
                },
                Err(_) => assert!(false),
            }

            // ensure nothing was cached
            assert_eq!(context.borrow().should_cache, false);
            assert_eq!(context.borrow().decks.len(), 0);
        };

    };

    {

        // case: update deck name

        {
            let request = UpdateDeckName {
                name: "FooBar".to_string()
            };

            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::write_lock(context.clone());
            match decks::update_deck_name(context, 1, request) {
                Ok(_) => assert!(true),
                Err(_) => assert!(false),
            };
        };

        {
            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::read_lock(context.clone());
            match decks::get_deck(context.clone(), 1) {
                Ok(actual) => {
                    assert_eq!(actual.id, 1);
                    assert_eq!(actual.name, format!("FooBar"));
                    assert_eq!(actual.description, format!("this is a description"));
                    assert_eq!(actual.created_at, actual.updated_at);
                    assert_eq!(actual.created_at, actual.reviewed_at);
                    assert_eq!(actual.has_reviewed, false);
                },
                Err(_) => assert!(false),
            }

            // ensure nothing was cached
            assert_eq!(context.borrow().should_cache, false);
            assert_eq!(context.borrow().decks.len(), 0);
        };

    };

    // deck exists

    {
        // case: doesn't exist

        {
            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::read_lock(context.clone());
            match decks::deck_exists(context, 3) {
                Ok(actual) => {
                    assert_eq!(actual, false);
                }
                Err(_) => assert!(false),
            }
        };

        // case: exists

        {
            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::read_lock(context.clone());
            match decks::deck_exists(context, 1) {
                Ok(actual) => {
                    assert_eq!(actual, true);
                }
                Err(_) => assert!(false),
            }
        };

        {
            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::read_lock(context.clone());
            match decks::deck_exists(context, 2) {
                Ok(actual) => {
                    assert_eq!(actual, true);
                }
                Err(_) => assert!(false),
            }
        };

    };

    // connect decks and fetching deck parents

    {

        {
            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::read_lock(context.clone());
            match decks::get_parent_id_of_deck(context, 1) {
                Ok(actual) => assert_eq!(actual, None),
                Err(_) => assert!(false)
            }
        };

        {
            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::read_lock(context.clone());
            match decks::get_parent_id_of_deck(context, 2) {
                Ok(actual) => assert_eq!(actual, None),
                Err(_) => assert!(false)
            }
        };

        // set 2 to be child of 1
        {
            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::write_lock(context.clone());
            match decks::connect_decks(context, 2, 1) {
                Ok(_) => assert!(true),
                Err(_) => assert!(false)
            }
        };

        // verify

        {
            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::read_lock(context.clone());
            match decks::get_parent_id_of_deck(context, 1) {
                Ok(actual) => assert_eq!(actual, None),
                Err(_) => assert!(false)
            }
        };

        {
            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::read_lock(context.clone());
            match decks::get_parent_id_of_deck(context, 2) {
                Ok(actual) => assert_eq!(actual, Some(1)),
                Err(_) => assert!(false)
            }
        };

        // parent id of non-existent deck doesn't exist

        {
            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::read_lock(context.clone());
            match decks::get_parent_id_of_deck(context, 42) {
                Ok(actual) => assert_eq!(actual, None),
                Err(_) => assert!(false)
            }
        };

    };

    // deck paths

    {

        {
            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::read_lock(context.clone());
            match decks::get_path_of_deck(context, 1) {
                Ok(actual) => assert_eq!(actual, vec![1]),
                Err(_) => assert!(false)
            }
        };

        {
            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::read_lock(context.clone());
            match decks::get_path_of_deck(context, 2) {
                Ok(actual) => assert_eq!(actual, vec![1, 2]),
                Err(_) => assert!(false)
            }
        };

        {
            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::read_lock(context.clone());
            match decks::get_path_of_deck(context, 42) {
                Ok(actual) => assert_eq!(actual, vec![]),
                Err(_) => assert!(false)
            }
        };

    };

    // deck children

    {

        // deck with a child

        {
            let deck_page_query = DecksPageQuery(1, Default::default());
            let search = Search::NoQuery;

            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::read_lock(context.clone());
            match decks::get_deck_children(context, 1, &deck_page_query, &search) {
                Ok(actual) => assert_eq!(actual, vec![2]),
                Err(_) => assert!(false)
            }
        };

        // deck with no children

        {
            let deck_page_query = DecksPageQuery(1, Default::default());
            let search = Search::NoQuery;

            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::read_lock(context.clone());
            match decks::get_deck_children(context, 2, &deck_page_query, &search) {
                Ok(actual) => assert_eq!(actual, vec![]),
                Err(_) => assert!(false)
            }
        };

        // non-existent deck

        {
            let deck_page_query = DecksPageQuery(1, Default::default());
            let search = Search::NoQuery;

            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::read_lock(context.clone());
            match decks::get_deck_children(context, 42, &deck_page_query, &search) {
                Ok(actual) => assert_eq!(actual, vec![]),
                Err(_) => assert!(false)
            }
        };

        // TODO: pagination

        // TODO: pagination + search

        // TODO: search
    };

    // deck children count

    {

        // deck with a child

        {
            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            (*context.borrow_mut()).should_cache = true;
            let _guard = context::read_lock(context.clone());
            match decks::get_deck_children_total_count(context.clone(), 1) {
                Ok(actual) => assert_eq!(actual, 1),
                Err(_) => assert!(false)
            }

            // ensure deck was cached
            assert_eq!(context.borrow().should_cache, true);
            assert_eq!(context.borrow().deck_children_count.len(), 1);

            // cache check

            let context = context.borrow();
            let actual = context.deck_children_count.get(&1).unwrap();
            assert_eq!(*actual, 1);

        };

        // deck with no children

        {
            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::read_lock(context.clone());
            match decks::get_deck_children_total_count(context, 2) {
                Ok(actual) => assert_eq!(actual, 0),
                Err(_) => assert!(false)
            }
        };

        // non-existent deck

        {
            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::read_lock(context.clone());
            match decks::get_deck_children_total_count(context, 42) {
                Ok(actual) => assert_eq!(actual, 0),
                Err(_) => assert!(false)
            }
        };

    };

    /* teardown */

    fs::remove_file(file_path.clone()).unwrap();
}
