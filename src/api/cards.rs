/* rust lib imports */

use std::cell::RefCell;
use std::rc::Rc;

/* 3rd-party imports */

use rusqlite::Connection;
use rusqlite::types::ToSql;
use rusqlite::Error as SqliteError;
use serde_json;
use chrono::naive::datetime::NaiveDateTime;

/* local imports */

use context::Context;
use types::{UnixTimestamp, CardID, DeckID, CardsPageQuery, Search, ItemCount, Offset};
use errors::RawAPIError;
use constants;
use api::review::{self, ActiveSelection, CachedReviewProcedure};
use api::user;
use timestamp;

/* ////////////////////////////////////////////////////////////////////////// */

#[derive(Debug, Clone, Serialize)]
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

// struct for requesting to create a card
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateCard {
    pub title: String, // required;
                       // if empty, it will be the first 140 characters (tweet length)
                       // of the question (must be given)
    pub question: String, // required, but may be empty
    pub answer: String, // required, but may be empty
    pub description: String, // required, but may be empty

    pub is_active: bool
}

impl CreateCard {
    pub fn validate(&self) -> Option<String> {

        if self.title.trim().len() <= 0 && self.question.trim().len() <= 0 {
            return Some("Either card title or card question must not be empty.".to_string());
        }

        return None;
    }
}

#[derive(Debug, Serialize)]
pub struct CardCreateResponse {
    pub profile_url: String
}

// TODO: need syncing mechanism (e.g. checksum) to be able to do field-level updates from client side
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateCard {
    pub title: String,
    pub question: String,
    pub answer: String,
    pub description: String,
    pub is_active: bool
}

impl UpdateCard {
    pub fn validate(&self) -> Option<String> {

        if self.title.trim().len() <= 0 && self.question.trim().len() <= 0 {
            return Some("Either card title or card question must not be empty.".to_string());
        }

        return None;
    }
}

#[derive(Debug, Serialize)]
pub struct DeleteCardResponse {
    pub redirect_to: String
}

#[inline]
pub fn get_card(context: Rc<RefCell<Context>>, card_id: CardID) -> Result<Card, RawAPIError> {

    {
        let context = context.borrow();
        if context.should_cache && context.cards.len() > 0 {
            match context.cards.get(&card_id) {
                None => {},
                Some(card) => {
                    return Ok(card.clone());
                }
            }
        }
    };

    assert!(context.borrow().is_read_locked());

    let query = format!("
        SELECT
            card_id,
            title,
            description,
            question,
            answer,
            created_at,
            updated_at,
            deck_id,
            is_active
        FROM Cards
        WHERE card_id = {card_id}
        LIMIT 1;
    ", card_id = card_id);

    let results = {
        let context = context.borrow();
        db_read_lock!(db_conn; context.database());
        let db_conn: &Connection = db_conn;

        let results = db_conn.query_row(&query, &[], |row| -> Card {
            return Card {
                id: row.get(0),
                title: row.get(1),
                description: row.get(2),
                question: row.get(3),
                answer: row.get(4),
                created_at: row.get(5),
                updated_at: row.get(6),
                deck_id: row.get(7),
                is_active: row.get(8)
            };
        });

        results
    };

    match results {
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        }
        Ok(card) => {

            let mut context = context.borrow_mut();

            if context.should_cache {
                context.cards.insert(card.id, card.clone());
            }

            return Ok(card);
        }
    };
}

#[inline]
pub fn card_exists(context: Rc<RefCell<Context>>, card_id: CardID) -> Result<bool, RawAPIError> {

    {
        let context = context.borrow();
        if context.should_cache && context.cards.len() > 0 {
            return Ok(context.cards.contains_key(&card_id));
        }
    };

    assert!(context.borrow().is_read_locked());

    let query = format!("
        SELECT
            COUNT(1)
        FROM Cards
        WHERE card_id = {card_id}
        LIMIT 1;
    ", card_id = card_id);

    let context = context.borrow();
    db_read_lock!(db_conn; context.database());
    let db_conn: &Connection = db_conn;

    let card_exists = db_conn.query_row(&query, &[], |row| -> bool {
        let count: i64 = row.get(0);
        return count >= 1;
    });

    match card_exists {
        Ok(card_exists) => return Ok(card_exists),
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        }
    }
}

#[inline]
pub fn create_card(
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    create_card_request: CreateCard) -> Result<Card, RawAPIError> {

    assert!(context.borrow().is_write_locked());

    let query = format!("
        INSERT INTO Cards(title, description, question, answer, deck_id, is_active)
        VALUES (:title, :description, :question, :answer, {deck_id}, :is_active);
    ", deck_id = deck_id);

    let params: &[(&str, &ToSql)] = &[(":title", &create_card_request.title.clone()),
                                      (":description", &create_card_request.description.clone()),
                                      (":question", &create_card_request.question.clone()),
                                      (":answer", &create_card_request.answer.clone()),
                                      (":is_active", &create_card_request.is_active.clone())];

    let card_id: CardID = {

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

        let row_id = db_conn.last_insert_rowid();
        row_id
    };

    return get_card(context, card_id);
}

// TODO: test
#[inline]
pub fn delete_card(
    context: Rc<RefCell<Context>>,
    card_id: CardID
    ) -> Result<(), RawAPIError> {

    assert!(context.borrow().is_write_locked());

    let query = format!(indoc!("
        DELETE FROM
            Cards
        WHERE card_id = {card_id};
    "), card_id = card_id);

    let context = context.borrow();
    db_write_lock!(db_conn; context.database());
    let db_conn: &Connection = db_conn;

    match db_conn.execute(&query, &[]) {
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        }
        _ => {
            /* query sucessfully executed */
        }
    }

    return Ok(());
}

// TODO: needs test
#[inline]
pub fn update_card(
    context: Rc<RefCell<Context>>,
    card_id: CardID,
    update_card_request: UpdateCard
    ) -> Result<Card, RawAPIError>  {

    assert!(context.borrow().is_write_locked());

    let query = format!("
        UPDATE Cards
        SET
            title = :title,
            description = :description,
            question = :question,
            answer = :answer,
            is_active = :is_active
        WHERE card_id = {card_id};
    ", card_id = card_id);

    let params: &[(&str, &ToSql)] = &[
        (":title", &update_card_request.title.clone()),
        (":description", &update_card_request.description.clone()),
        (":question", &update_card_request.question.clone()),
        (":answer", &update_card_request.answer.clone()),
        (":is_active", &update_card_request.is_active.clone())
    ];

    {
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
    };

    return get_card(context, card_id);
}

#[inline]
pub fn total_num_of_cards_in_deck(
    context: Rc<RefCell<Context>>,
    deck_id: DeckID
    ) -> Result<ItemCount, RawAPIError> {

    assert!(context.borrow().is_read_locked());

    // TODO: complete
    let search_inner_join = "";
    // let search_inner_join = match maybe_search_query {
    //     None => "",
    //     Some(_) => {
    //         "
    //         INNER JOIN CardsFTS
    //         ON CardsFTS.docid = c.card_id
    //         "
    //     }
    // };

    // TODO: complete
    let search_where_cond = "";
    // let search_where_cond = match maybe_search_query {
    //     None => "",
    //     Some(_) => {
    //         "AND CardsFTS MATCH :search_query"
    //     }
    // };

    let query = format!("
        SELECT
            COUNT(1)
        FROM DecksClosure AS dc

        INNER JOIN Cards AS c
        ON c.deck_id = dc.descendent

        {search_inner_join}

        WHERE
        dc.ancestor = {deck_id}

        {search_where_cond}
        ;
    ",
    deck_id = deck_id,
    search_inner_join = search_inner_join,
    search_where_cond = search_where_cond);

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

            return Ok(count)
        },
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        }
    }
}

#[inline]
pub fn is_card_in_deck(
    context: Rc<RefCell<Context>>,
    card_id: CardID,
    deck_id: DeckID) -> Result<bool, RawAPIError> {

    assert!(context.borrow().is_read_locked());

    let query = format!(indoc!("
        SELECT
            COUNT(1)
        FROM DecksClosure AS dc
            INNER JOIN Cards AS c
            ON c.deck_id = dc.descendent
        WHERE
            dc.ancestor = {deck_id}
        AND
            c.card_id = {card_id}
        LIMIT 1;"), card_id = card_id, deck_id = deck_id);

    let context = context.borrow();
    db_read_lock!(db_conn; context.database());
    let db_conn: &Connection = db_conn;

    let card_exists = db_conn.query_row(&query, &[], |row| -> bool {
        let count: i64 = row.get(0);
        return count >= 1;
    });

    match card_exists {
        Ok(card_exists) => return Ok(card_exists),
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        }
    }
}

#[inline]
pub fn cards_in_deck(
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    cards_page_query: &CardsPageQuery,
    search: &Search) -> Result<Vec<Card>, RawAPIError> {

    assert!(context.borrow().is_read_locked());

    // TODO: complete
    let search_inner_join = "";
    let search_where_cond = "";

    let select_sql = format!(indoc!("
        SELECT
            c.card_id,
            c.title,
            c.description,
            c.question,
            c.answer,
            c.created_at,
            c.updated_at,
            c.deck_id,
            c.is_active
        FROM DecksClosure AS dc

        INNER JOIN Cards AS c
        ON c.deck_id = dc.descendent

        {search_inner_join}
        "), search_inner_join = search_inner_join);

    let inner_select_sql = format!(indoc!("
        SELECT
            c.card_id
        FROM DecksClosure AS dc

        INNER JOIN Cards AS c
        ON c.deck_id = dc.descendent

        {search_inner_join}
        "), search_inner_join = search_inner_join);

    let where_sql = format!(indoc!("
        dc.ancestor = {deck_id}

        {search_where_cond}
    "),
    deck_id = deck_id,
    search_where_cond = search_where_cond);

    let order_by_sql = format!(indoc!("
        c.created_at
        DESC
    "));

    let offset = cards_page_query.get_offset();
    let per_page = cards_page_query.get_per_page();

    let query = pagination!(
        select_sql = select_sql;
        not_in = "c.oid";
        inner_select_sql = inner_select_sql;
        where_sql = Some(&where_sql);
        order_by_sql = Some(&order_by_sql);
        offset = offset;
        per_page = per_page);

    let mut context = context.borrow_mut();
    db_read_lock!(db_conn; context.database());
    let db_conn: &Connection = db_conn;

    let mut statement = match db_conn.prepare(&query) {
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        },
        Ok(statement) => statement
    };

    let maybe_iter = statement.query_map(&[], |row| -> Card {
        return Card {
            id: row.get(0),
            title: row.get(1),
            description: row.get(2),
            question: row.get(3),
            answer: row.get(4),
            created_at: row.get(5),
            updated_at: row.get(6),
            deck_id: row.get(7),
            is_active: row.get(8)
        };
    });

    match maybe_iter {
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        },
        Ok(iter) => {

            let mut vec_of_card: Vec<Card> = Vec::new();

            for card in iter {

                let item = match card {
                    Err(sqlite_error) => {
                        return Err(RawAPIError::SQLError(sqlite_error, query));
                    },
                    Ok(item) => {

                        // let mut context = context.borrow_mut();
                        if context.should_cache {
                            context.cards.insert(item.id, item.clone());
                        }

                        item
                    }
                };

                vec_of_card.push(item);

            }

            return Ok(vec_of_card);
        }
    }
}

#[inline]
pub fn deck_have_cards(context: Rc<RefCell<Context>>, deck_id: DeckID) -> Result<bool, RawAPIError> {

    assert!(context.borrow().is_read_locked());

    let query = format!("
        SELECT
            COUNT(1)
        FROM DecksClosure AS dc

        INNER JOIN Cards AS c
        ON c.deck_id = dc.descendent

        WHERE
        dc.ancestor = {deck_id}
        LIMIT 1;
    ",
    deck_id = deck_id);

    let context = context.borrow();
    db_read_lock!(db_conn; context.database());
    let db_conn: &Connection = db_conn;

    let have_cards = db_conn.query_row(&query, &[], |row| -> bool {
        let count: i64 = row.get(0);
        return count >= 1;
    });

    match have_cards {
        Ok(have_cards) => return Ok(have_cards),
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        }
    }
}

// TODO: needs test
#[inline]
pub fn deck_have_cards_for_review(
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    active_selection: &review::ActiveSelection) -> Result<bool, RawAPIError> {

    assert!(context.borrow().is_read_locked());

    let mut is_active = true;
    let mut params: Vec<(&str, &ToSql)> = vec![];

    let active_query = match *active_selection {
        ActiveSelection::Active => {

            let active_query = "AND c.is_active = :is_active";
            is_active = true;
            params.push((":is_active", &is_active));

            active_query
        },
        ActiveSelection::Inactive => {

            let active_query = "AND c.is_active = :is_active";
            is_active = false;
            params.push((":is_active", &is_active));

            active_query
        },
        ActiveSelection::All => {

            let active_query = "";

            active_query
        }
    };

    let query = format!(indoc!("
        SELECT
            COUNT(1)
        FROM DecksClosure AS dc

        INNER JOIN Cards AS c
        ON c.deck_id = dc.descendent

        WHERE
            dc.ancestor = {deck_id}
        {active_query}
        LIMIT 1;
    "),
    deck_id = deck_id,
    active_query = active_query);

    let context = context.borrow();
    db_read_lock!(db_conn; context.database());
    let db_conn: &Connection = db_conn;

    let have_cards = db_conn.query_row_named(&query, params.as_slice(), |row| -> bool {
        let count: i64 = row.get(0);
        return count >= 1;
    });

    match have_cards {
        Ok(have_cards) => return Ok(have_cards),
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        }
    }
}

// TODO: needs test
#[inline]
pub fn deck_num_of_cards_for_review(
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    active_selection: &review::ActiveSelection) -> Result<ItemCount, RawAPIError> {

    assert!(context.borrow().is_read_locked());

    let mut is_active = true;
    let mut params: Vec<(&str, &ToSql)> = vec![];

    let active_query = match *active_selection {
        ActiveSelection::Active => {

            let active_query = "AND c.is_active = :is_active";
            is_active = true;
            params.push((":is_active", &is_active));

            active_query
        },
        ActiveSelection::Inactive => {

            let active_query = "AND c.is_active = :is_active";
            is_active = false;
            params.push((":is_active", &is_active));

            active_query
        },
        ActiveSelection::All => {

            let active_query = "";

            active_query
        }
    };

    let query = format!(indoc!("
        SELECT
            COUNT(1)
        FROM DecksClosure AS dc

        INNER JOIN Cards AS c
        ON c.deck_id = dc.descendent

        INNER JOIN CardsScore AS cs
        ON c.card_id = cs.card_id

        WHERE
            dc.ancestor = {deck_id}
        {active_query};
    "),
    deck_id = deck_id,
    active_query = active_query);

    let context = context.borrow();
    db_read_lock!(db_conn; context.database());
    let db_conn: &Connection = db_conn;

    let result = db_conn.query_row_named(&query, params.as_slice(), |row| -> ItemCount {
        let count: i64 = row.get(0);
        return count as ItemCount;
    });

    match result {
        Ok(num_cards) => return Ok(num_cards),
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        }
    }
}

// TODO: needs test
#[inline]
pub fn deck_have_new_cards_for_review(
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    active_selection: &review::ActiveSelection) -> Result<bool, RawAPIError> {

    assert!(context.borrow().is_read_locked());

    let mut is_active = true;
    let mut params: Vec<(&str, &ToSql)> = vec![];

    let active_query = match *active_selection {
        ActiveSelection::Active => {

            let active_query = "AND c.is_active = :is_active";
            is_active = true;
            params.push((":is_active", &is_active));

            active_query
        },
        ActiveSelection::Inactive => {

            let active_query = "AND c.is_active = :is_active";
            is_active = false;
            params.push((":is_active", &is_active));

            active_query
        },
        ActiveSelection::All => {

            let active_query = "";

            active_query
        }
    };

    let query = format!(indoc!("
        SELECT
            COUNT(1)
        FROM DecksClosure AS dc

        INNER JOIN Cards AS c
        ON c.deck_id = dc.descendent

        INNER JOIN CardsScore AS cs
        ON c.card_id = cs.card_id

        WHERE
            dc.ancestor = {deck_id}
            AND
                (c.created_at - cs.seen_at) = 0
        {active_query}
        LIMIT 1;
    "),
    deck_id = deck_id,
    active_query = active_query);

    let context = context.borrow();
    db_read_lock!(db_conn; context.database());
    let db_conn: &Connection = db_conn;

    let have_cards = db_conn.query_row_named(&query, params.as_slice(), |row| -> bool {
        let count: i64 = row.get(0);
        return count >= 1;
    });

    match have_cards {
        Ok(have_cards) => return Ok(have_cards),
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        }
    }
}

// TODO: need test
#[inline]
pub fn deck_num_of_new_cards_for_review(
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    active_selection: &review::ActiveSelection) -> Result<ItemCount, RawAPIError> {

    assert!(context.borrow().is_read_locked());

    let mut is_active = true;
    let mut params: Vec<(&str, &ToSql)> = vec![];

    let active_query = match *active_selection {
        ActiveSelection::Active => {

            let active_query = "AND c.is_active = :is_active";
            is_active = true;
            params.push((":is_active", &is_active));

            active_query
        },
        ActiveSelection::Inactive => {

            let active_query = "AND c.is_active = :is_active";
            is_active = false;
            params.push((":is_active", &is_active));

            active_query
        },
        ActiveSelection::All => {

            let active_query = "";

            active_query
        }
    };

    let query = format!(indoc!("
        SELECT
            COUNT(1)
        FROM DecksClosure AS dc

        INNER JOIN Cards AS c
        ON c.deck_id = dc.descendent

        INNER JOIN CardsScore AS cs
        ON c.card_id = cs.card_id

        WHERE
            dc.ancestor = {deck_id}
            AND
                (c.created_at - cs.seen_at) = 0
        {active_query};
    "),
    deck_id = deck_id,
    active_query = active_query);

    let context = context.borrow();
    db_read_lock!(db_conn; context.database());
    let db_conn: &Connection = db_conn;

    let result = db_conn.query_row_named(&query, params.as_slice(), |row| -> ItemCount {
        let count: i64 = row.get(0);
        return count as ItemCount;
    });

    match result {
        Ok(num_cards) => return Ok(num_cards),
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        }
    }
}

// TODO: need test
#[inline]
pub fn deck_get_new_card_for_review(
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    active_selection: &review::ActiveSelection,
    index: Offset) -> Result<CardID, RawAPIError> {

    assert!(context.borrow().is_read_locked());

    let mut is_active = true;
    let mut params: Vec<(&str, &ToSql)> = vec![];

    let active_query = match *active_selection {
        ActiveSelection::Active => {

            let active_query = "AND c.is_active = :is_active";
            is_active = true;
            params.push((":is_active", &is_active));

            active_query
        },
        ActiveSelection::Inactive => {

            let active_query = "AND c.is_active = :is_active";
            is_active = false;
            params.push((":is_active", &is_active));

            active_query
        },
        ActiveSelection::All => {

            let active_query = "";

            active_query
        }
    };

    let select_sql = indoc!("
        SELECT
            c.card_id
        FROM DecksClosure AS dc

        INNER JOIN Cards AS c
        ON c.deck_id = dc.descendent

        INNER JOIN CardsScore AS cs
        ON c.card_id = cs.card_id
        ");

    let where_sql = format!(indoc!("
        dc.ancestor = {deck_id}
        AND
            (c.created_at - cs.seen_at) = 0
        {active_query}
        "),
        deck_id = deck_id,
        active_query = active_query);

    // least recently created to most recently created
    let order_by_sql = format!(indoc!("
        (strftime('%s','now') - c.created_at)
        DESC
    "));

    let per_page = 1;
    let offset = index;

    let query = pagination!(
        select_sql = select_sql;
        not_in = "c.oid";
        inner_select_sql = select_sql;
        where_sql = Some(&where_sql);
        order_by_sql = Some(&order_by_sql);
        offset = offset;
        per_page = per_page);

    let context = context.borrow();
    db_read_lock!(db_conn; context.database());
    let db_conn: &Connection = db_conn;

    let result = db_conn.query_row_named(&query, params.as_slice(), |row| -> CardID {
        return row.get(0);
    });

    match result {
        Ok(card_id) => return Ok(card_id),
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        }
    }
}

// TODO: needs test
#[inline]
pub fn deck_have_cards_ready_for_review(
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    active_selection: &ActiveSelection) -> Result<bool, RawAPIError> {

    assert!(context.borrow().is_read_locked());

    let mut is_active = true;
    let mut params: Vec<(&str, &ToSql)> = vec![];

    let active_query = match *active_selection {
        ActiveSelection::Active => {

            let active_query = "AND c.is_active = :is_active";
            is_active = true;
            params.push((":is_active", &is_active));

            active_query
        },
        ActiveSelection::Inactive => {

            let active_query = "AND c.is_active = :is_active";
            is_active = false;
            params.push((":is_active", &is_active));

            active_query
        },
        ActiveSelection::All => {

            let active_query = "";

            active_query
        }
    };

    let user_review_count = match try!(user::get_review_count(context.clone())) {
        None => 0,
        Some(review_count) => review_count
    };

    let query = format!(indoc!("
        SELECT
            COUNT(1)
        FROM DecksClosure AS dc

        INNER JOIN Cards AS c
        ON c.deck_id = dc.descendent

        INNER JOIN CardsScore AS cs
        ON c.card_id = cs.card_id

        WHERE
            dc.ancestor = {deck_id}
        AND
            (cs.seen_at + cs.review_after) >= strftime('%s', 'now')
        AND
            (cs.reviewed_at_count + cs.cards_till_ready_for_review) <= {user_review_count}
        {active_query}
        LIMIT 1;
    "),
    deck_id = deck_id,
    active_query = active_query,
    user_review_count = user_review_count);

    let context = context.borrow();
    db_read_lock!(db_conn; context.database());
    let db_conn: &Connection = db_conn;

    let have_cards = db_conn.query_row_named(&query, params.as_slice(), |row| -> bool {
        let count: i64 = row.get(0);
        return count >= 1;
    });

    match have_cards {
        Ok(have_cards) => return Ok(have_cards),
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        }
    }
}

// TODO: needs test
#[inline]
pub fn deck_num_of_cards_ready_for_review(
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    active_selection: &review::ActiveSelection)
    -> Result<ItemCount, RawAPIError> {

    assert!(context.borrow().is_read_locked());

    let mut is_active = true;
    let mut params: Vec<(&str, &ToSql)> = vec![];

    let active_query = match *active_selection {
        ActiveSelection::Active => {

            let active_query = "AND c.is_active = :is_active";
            is_active = true;
            params.push((":is_active", &is_active));

            active_query
        },
        ActiveSelection::Inactive => {

            let active_query = "AND c.is_active = :is_active";
            is_active = false;
            params.push((":is_active", &is_active));

            active_query
        },
        ActiveSelection::All => {

            let active_query = "";

            active_query
        }
    };

    let user_review_count = match try!(user::get_review_count(context.clone())) {
        None => 0,
        Some(review_count) => review_count
    };

    let query = format!(indoc!("
        SELECT
            COUNT(1)
        FROM DecksClosure AS dc

        INNER JOIN Cards AS c
        ON c.deck_id = dc.descendent

        INNER JOIN CardsScore AS cs
        ON c.card_id = cs.card_id

        WHERE
            dc.ancestor = {deck_id}
        AND
                (cs.seen_at + cs.review_after) >= strftime('%s', 'now')
        AND
            (cs.reviewed_at_count + cs.cards_till_ready_for_review) <= {user_review_count}
        {active_query};
    "),
    deck_id = deck_id,
    active_query = active_query,
    user_review_count = user_review_count);

    let context = context.borrow();
    db_read_lock!(db_conn; context.database());
    let db_conn: &Connection = db_conn;

    let result = db_conn.query_row_named(&query, params.as_slice(), |row| -> ItemCount {
        let count: i64 = row.get(0);
        return count as ItemCount;
    });

    match result {
        Ok(num_cards) => return Ok(num_cards),
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        }
    }

}

// TODO: need test
#[inline]
pub fn deck_get_card_ready_for_review(
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    active_selection: &review::ActiveSelection,
    index: Offset) -> Result<CardID, RawAPIError> {

    assert!(context.borrow().is_read_locked());

    let user_review_count = match try!(user::get_review_count(context.clone())) {
        None => 0,
        Some(review_count) => review_count
    };

    let mut is_active = true;
    let mut params: Vec<(&str, &ToSql)> = vec![];

    let active_query = match *active_selection {
        ActiveSelection::Active => {

            let active_query = "AND c.is_active = :is_active";
            is_active = true;
            params.push((":is_active", &is_active));

            active_query
        },
        ActiveSelection::Inactive => {

            let active_query = "AND c.is_active = :is_active";
            is_active = false;
            params.push((":is_active", &is_active));

            active_query
        },
        ActiveSelection::All => {

            let active_query = "";

            active_query
        }
    };

    let select_sql = indoc!("
        SELECT
            c.card_id
        FROM DecksClosure AS dc

        INNER JOIN Cards AS c
        ON c.deck_id = dc.descendent

        INNER JOIN CardsScore AS cs
        ON c.card_id = cs.card_id
        ");

    let where_sql = format!(indoc!("
            dc.ancestor = {deck_id}
        AND
            (cs.seen_at + cs.review_after) >= strftime('%s', 'now')
        AND
            (cs.reviewed_at_count + cs.cards_till_ready_for_review) <= {user_review_count}
        {active_query}
        "),
        deck_id = deck_id,
        active_query = active_query,
        user_review_count = user_review_count);

    let order_by_sql = format!(indoc!("
        raw_score(cs.success, cs.fail)
        DESC
    "));

    let per_page = 1;
    let offset = index;

    let query = pagination!(
        select_sql = select_sql;
        not_in = "c.oid";
        inner_select_sql = select_sql;
        where_sql = Some(&where_sql);
        order_by_sql = Some(&order_by_sql);
        offset = offset;
        per_page = per_page);

    let context = context.borrow();
    db_read_lock!(db_conn; context.database());
    let db_conn: &Connection = db_conn;

    let result = db_conn.query_row_named(&query, params.as_slice(), |row| -> CardID {
        return row.get(0);
    });

    match result {
        Ok(card_id) => return Ok(card_id),
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        }
    }
}

// TODO: need test
#[inline]
pub fn deck_get_least_recently_reviewed_card(
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    active_selection: &review::ActiveSelection,
    top_n: ItemCount,
    index: Offset
) -> Result<CardID, RawAPIError> {

    // TODO: dev mode
    assert!(0 <= index);
    assert!(index < top_n);

    assert!(context.borrow().is_read_locked());

    // parse active

    let mut is_active = true;
    let mut params: Vec<(&str, &ToSql)> = vec![];

    let active_query = match *active_selection {
        ActiveSelection::Active => {

            let active_query = "AND c.is_active = :is_active";
            is_active = true;
            params.push((":is_active", &is_active));

            active_query
        },
        ActiveSelection::Inactive => {

            let active_query = "AND c.is_active = :is_active";
            is_active = false;
            params.push((":is_active", &is_active));

            active_query
        },
        ActiveSelection::All => {

            let active_query = "";

            active_query
        }
    };

    // top N selection query

    let top_n_query = format!(indoc!("
        SELECT

            c.card_id,
            cs.success,
            cs.fail

        FROM DecksClosure AS dc

        INNER JOIN Cards AS c
        ON c.deck_id = dc.descendent

        INNER JOIN CardsScore AS cs
        ON c.card_id = cs.card_id

        WHERE

            dc.ancestor = {deck_id}

            {active_query}

        ORDER BY

            (strftime('%s','now') - cs.seen_at) DESC

        LIMIT {top_n}
        "),
        deck_id = deck_id,
        active_query = active_query,
        top_n = top_n);

    // sort top n query based on score

    let select_sql = format!(indoc!("
        SELECT

            tnq.card_id

        FROM

            ({top_n_query}) AS tnq
        "),
        top_n_query = top_n_query);

    let order_by_sql = format!(indoc!("
            raw_score(tnq.success, tnq.fail) DESC
        "));

    let per_page = 1;
    let offset = index;

    let query = pagination!(
        select_sql = select_sql;
        not_in = "tnq.card_id";
        inner_select_sql = select_sql;
        where_sql = None;
        order_by_sql = Some(&order_by_sql);
        offset = offset;
        per_page = per_page);

    let context = context.borrow();
    db_read_lock!(db_conn; context.database());
    let db_conn: &Connection = db_conn;

    let result = db_conn.query_row_named(&query, params.as_slice(), |row| -> CardID {
        return row.get(0);
    });

    match result {
        Ok(card_id) => return Ok(card_id),
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        }
    }
}

// TODO: remove this
#[inline]
pub fn ___deck_get_least_recently_reviewed_card(
    context: Rc<RefCell<Context>>,
    deck_id: DeckID,
    active_selection: &review::ActiveSelection,
    index: Offset) -> Result<CardID, RawAPIError> {

    assert!(context.borrow().is_read_locked());

    let mut is_active = true;
    let mut params: Vec<(&str, &ToSql)> = vec![];

    let active_query = match *active_selection {
        ActiveSelection::Active => {

            let active_query = "AND c.is_active = :is_active";
            is_active = true;
            params.push((":is_active", &is_active));

            active_query
        },
        ActiveSelection::Inactive => {

            let active_query = "AND c.is_active = :is_active";
            is_active = false;
            params.push((":is_active", &is_active));

            active_query
        },
        ActiveSelection::All => {

            let active_query = "";

            active_query
        }
    };

    let select_sql = indoc!("
        SELECT
            c.card_id
        FROM DecksClosure AS dc

        INNER JOIN Cards AS c
        ON c.deck_id = dc.descendent

        INNER JOIN CardsScore AS cs
        ON c.card_id = cs.card_id
        ");

    let where_sql = format!(indoc!("
            dc.ancestor = {deck_id}
        {active_query}
        "),
        deck_id = deck_id,
        active_query = active_query);

    let order_by_sql = format!(indoc!("
        (strftime('%s','now') - cs.seen_at)
        DESC
    "));

    let per_page = 1;
    let offset = index;

    let query = pagination!(
        select_sql = select_sql;
        not_in = "c.oid";
        inner_select_sql = select_sql;
        where_sql = Some(&where_sql);
        order_by_sql = Some(&order_by_sql);
        offset = offset;
        per_page = per_page);

    let context = context.borrow();
    db_read_lock!(db_conn; context.database());
    let db_conn: &Connection = db_conn;

    let result = db_conn.query_row_named(&query, params.as_slice(), |row| -> CardID {
        return row.get(0);
    });

    match result {
        Ok(card_id) => return Ok(card_id),
        Err(sqlite_error) => {
            return Err(RawAPIError::SQLError(sqlite_error, query));
        }
    }
}

// TODO: needs test
#[inline]
pub fn cached_review_card_for_deck(
    context: Rc<RefCell<Context>>,
    deck_id: DeckID
    ) -> Result<Option<(CardID, Option<CachedReviewProcedure>)>, RawAPIError> {

    assert!(context.borrow().is_read_locked());

    let query = format!(indoc!("
        SELECT
            card_id,
            cached_review_procedure
        FROM CachedDeckReview
        WHERE
            deck_id = {deck_id}
        LIMIT 1;
    "), deck_id = deck_id);

    let context = context.borrow();
    db_read_lock!(db_conn; context.database());
    let db_conn: &Connection = db_conn;

    let results = db_conn.query_row(&query, &[], |row| -> (CardID, Option<CachedReviewProcedure>) {

        let card_id = row.get(0);
        let cached_review_procedure: String = row.get(1);
        let cached_review_procedure = match serde_json::from_str(&cached_review_procedure) {
            Ok(cached_review_procedure) => Some(cached_review_procedure),
            Err(_) => None
        };

        return (card_id, cached_review_procedure);
    });

    match results {
        Err(sqlite_error) => {

            match sqlite_error {
                SqliteError::QueryReturnedNoRows => {
                    return Ok(None);
                }
                _ => {}
            };

            return Err(RawAPIError::SQLError(sqlite_error, query.to_string()));
        }
        Ok((card_id, cached_review_procedure)) => {
            return Ok(Some((card_id, cached_review_procedure)));
        }
    };
}

#[test]
fn cards_test() {

    /* imports */

    use std::sync::{Arc, RwLock};
    use std::fs;
    use database;
    use api::{decks, cards};
    use context;

    /* setup */

    let file_path = "test/assets/cards_test.db".to_string();
    fs::remove_file(file_path.clone());

    let db_connection = database::get_database(file_path.clone());
    let global_lock = Arc::new(RwLock::new(db_connection));

    // card doesn't exist

    {
        let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
        let _guard = context::read_lock(context.clone());
        match cards::get_card(context, 1) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    };

    // create deck (to put a card in)

    {
        let request = decks::CreateDeck {
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

    // create card

    {

        // case: non existent deck

        {

            let request = CreateCard {
                title: format!("Card Title"),
                description: format!("Card Description"),
                question: format!("Card Question"),
                answer: format!("Card Answer"),
                is_active: false
            };

            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::write_lock(context.clone());
            match cards::create_card(context, 42, request) {
                Ok(_) => assert!(false),
                Err(_) => assert!(true),
            }
        };

        // case: deck that exists

        {

            let request = CreateCard {
                title: format!("Card Title"),
                description: format!("Card Description"),
                question: format!("Card Question"),
                answer: format!("Card Answer"),
                is_active: false
            };

            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::write_lock(context.clone());
            match cards::create_card(context, 1, request) {
                Ok(actual) => {
                    assert_eq!(actual.id, 1);
                    assert_eq!(actual.title, format!("Card Title"));
                    assert_eq!(actual.description, format!("Card Description"));
                    assert_eq!(actual.answer, format!("Card Answer"));
                    assert_eq!(actual.question, format!("Card Question"));
                    assert_eq!(actual.created_at, actual.updated_at);
                    assert_eq!(actual.deck_id, 1);
                    assert_eq!(actual.is_active, false);
                },
                Err(_) => assert!(false),
            }
        };

        // case: is active

        {

            let request = CreateCard {
                title: format!("Card Title 2"),
                description: format!("Card Description 2"),
                question: format!("Card Question 2"),
                answer: format!("Card Answer 2"),
                is_active: true
            };

            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::write_lock(context.clone());
            match cards::create_card(context, 1, request) {
                Ok(actual) => {
                    assert_eq!(actual.id, 2);
                    assert_eq!(actual.title, format!("Card Title 2"));
                    assert_eq!(actual.description, format!("Card Description 2"));
                    assert_eq!(actual.answer, format!("Card Answer 2"));
                    assert_eq!(actual.question, format!("Card Question 2"));
                    assert_eq!(actual.created_at, actual.updated_at);
                    assert_eq!(actual.deck_id, 1);
                    assert_eq!(actual.is_active, true);
                },
                Err(_) => assert!(false),
            }
        };

        // case: empty card title

        {

            let request = CreateCard {
                title: format!(""),
                description: format!("Card Description 2"),
                question: format!("Card Question 2"),
                answer: format!("Card Answer 2"),
                is_active: true
            };

            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::write_lock(context.clone());
            match cards::create_card(context, 1, request) {
                Ok(_) => assert!(false),
                Err(_) => assert!(true),
            }
        };

    };

    {
        // case: retrieve created cards

        {
            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::read_lock(context.clone());
            match cards::get_card(context.clone(), 1) {
                Ok(actual) => {
                    assert_eq!(actual.id, 1);
                    assert_eq!(actual.title, format!("Card Title"));
                    assert_eq!(actual.description, format!("Card Description"));
                    assert_eq!(actual.answer, format!("Card Answer"));
                    assert_eq!(actual.question, format!("Card Question"));
                    assert_eq!(actual.created_at, actual.updated_at);
                    assert_eq!(actual.deck_id, 1);
                    assert_eq!(actual.is_active, false);
                },
                Err(_) => assert!(false),
            }

            // ensure nothing was cached
            assert_eq!(context.borrow().should_cache, false);
            assert_eq!(context.borrow().cards.len(), 0);
        };

        {
            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::read_lock(context.clone());
            match cards::get_card(context.clone(), 2) {
                Ok(actual) => {
                    assert_eq!(actual.id, 2);
                    assert_eq!(actual.title, format!("Card Title 2"));
                    assert_eq!(actual.description, format!("Card Description 2"));
                    assert_eq!(actual.answer, format!("Card Answer 2"));
                    assert_eq!(actual.question, format!("Card Question 2"));
                    assert_eq!(actual.created_at, actual.updated_at);
                    assert_eq!(actual.deck_id, 1);
                    assert_eq!(actual.is_active, true);
                },
                Err(_) => assert!(false),
            }

            // ensure nothing was cached
            assert_eq!(context.borrow().should_cache, false);
            assert_eq!(context.borrow().cards.len(), 0);
        };

        {
            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            (*context.borrow_mut()).should_cache = true;
            let _guard = context::read_lock(context.clone());
            match cards::get_card(context.clone(), 1) {
                Ok(actual) => {
                    assert_eq!(actual.id, 1);
                    assert_eq!(actual.title, format!("Card Title"));
                    assert_eq!(actual.description, format!("Card Description"));
                    assert_eq!(actual.answer, format!("Card Answer"));
                    assert_eq!(actual.question, format!("Card Question"));
                    assert_eq!(actual.created_at, actual.updated_at);
                    assert_eq!(actual.deck_id, 1);
                    assert_eq!(actual.is_active, false);
                },
                Err(_) => assert!(false),
            }

            // ensure deck was cached
            assert_eq!(context.borrow().should_cache, true);
            assert_eq!(context.borrow().cards.len(), 1);

            // cache check

            let context = context.borrow();
            let actual = context.cards.get(&1).unwrap();
            assert_eq!(actual.id, 1);
            assert_eq!(actual.title, format!("Card Title"));
            assert_eq!(actual.description, format!("Card Description"));
            assert_eq!(actual.answer, format!("Card Answer"));
            assert_eq!(actual.question, format!("Card Question"));
            assert_eq!(actual.created_at, actual.updated_at);
            assert_eq!(actual.deck_id, 1);
            assert_eq!(actual.is_active, false);
        };
    };

    // cards exists

    {
        // case: doesn't exist

        {
            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::read_lock(context.clone());
            match cards::card_exists(context, 3) {
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
            match cards::card_exists(context, 1) {
                Ok(actual) => {
                    assert_eq!(actual, true);
                }
                Err(_) => assert!(false),
            }
        };

        {
            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::read_lock(context.clone());
            match cards::card_exists(context, 2) {
                Ok(actual) => {
                    assert_eq!(actual, true);
                }
                Err(_) => assert!(false),
            }
        };

    };

    // cards by deck

    {
        let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
        let _guard = context::read_lock(context.clone());
        let children = match cards::cards_in_deck(context.clone(), 1, &Default::default(), &Default::default()) {
            Ok(children) => {
                assert!(children.len() > 0);
            },
            Err(err) => {
                assert!(false);
            }
        };
    };

    // is card in deck

    {
        let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
        let _guard = context::read_lock(context.clone());
        match cards::is_card_in_deck(context.clone(), 1, 1) {
            Ok(actual) => assert!(actual),
            Err(_) => assert!(false)
        }
    };

    {
        let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
        let _guard = context::read_lock(context.clone());
        match cards::is_card_in_deck(context.clone(), 2, 1) {
            Ok(actual) => assert!(actual),
            Err(_) => assert!(false)
        }
    };

    {
        // case: nonexistent deck

        let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
        let _guard = context::read_lock(context.clone());
        match cards::is_card_in_deck(context.clone(), 1, 42) {
            Ok(actual) => assert_eq!(actual, false),
            Err(_) => assert!(false)
        }
    };

    {
        // case: nonexistent card

        let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
        let _guard = context::read_lock(context.clone());
        match cards::is_card_in_deck(context.clone(), 42, 1) {
            Ok(actual) => assert_eq!(actual, false),
            Err(_) => assert!(false)
        }
    };

    {
        // case: nonexistent deck and nonexistent card

        let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
        let _guard = context::read_lock(context.clone());
        match cards::is_card_in_deck(context.clone(), 42, 42) {
            Ok(actual) => assert_eq!(actual, false),
            Err(_) => assert!(false)
        }
    };

    {
        {
            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::read_lock(context.clone());

            match cards::deck_have_cards(context.clone(), 1) {
                Ok(actual) => assert!(actual),
                Err(_) => assert!(false)
            }
        };

        {
            let context = Rc::new(RefCell::new(Context::new(global_lock.clone())));
            let _guard = context::read_lock(context.clone());

            match cards::deck_have_cards(context.clone(), 42) {
                Ok(actual) => assert_eq!(actual, false),
                Err(_) => assert!(false)
            }
        };
    }

    /* teardown */

    fs::remove_file(file_path.clone()).unwrap();
}
