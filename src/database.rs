/* rust lib imports */

use std::sync::{Arc, Mutex};

/* 3rd-party imports */

use libc::{c_int, c_double};
use rusqlite::{Connection, Result as SqliteResult};
use rusqlite::functions::{Context};

/* local imports */

use tables;

/* ////////////////////////////////////////////////////////////////////////// */

// Arc := Shared resource between threads
// Mutex := Raw database operation
pub type Database = Arc<Mutex<Connection>>;

/* macro helpers */

macro_rules! db_read_lock(
    ($ident:ident; $e:expr) => (

        {
            use database::{Database};

            // hacky type checking
            let _: Database = $e;
        };

        let __db_read_lock = $e;
        let __db_conn_guard = __db_read_lock.lock().unwrap();
        let ref $ident = *__db_conn_guard;
    )
);

macro_rules! db_write_lock(
    ($ident:ident; $e:expr) => (

        {
            use database::{Database};

            // hacky type checking
            let _: Database = $e;
        };

        let __db_write_lock = $e;
        let __db_conn_guard = __db_write_lock.lock().unwrap();
        let ref $ident = *__db_conn_guard;
    )
);

// macro to generate efficient pagination sqlite query
// http://blog.ssokolow.com/archives/2009/12/23/sql-pagination-without-offset/
macro_rules! pagination(
    (
        select_sql = $select_sql:expr;
        not_in = $not_in:expr;
        inner_select_sql = $inner_select_sql:expr;
        where_sql = $where_sql:expr;
        order_by_sql = $order_by_sql:expr;
        offset = $offset:expr;
        per_page = $per_page:expr) => {{

        // Conventions:
        // select_sql := begins with SELECT
        // inner_select_sql := begins with SELECT
        // where_sql := does not begin wtih WHERE
        // order_by_sql := does not begin with ORDER BY
        //
        // offset >= 0
        // per_page >= 0


        use types;
        // TODO: compile-time type check other args
        let per_page: types::PerPage = $per_page;
        let offset: types::Offset = $offset;

        let where_sql: Option<&str> = $where_sql;
        let (where_sql, inner_where_sql)  = match where_sql {
            None => {
                ("".to_string(), "".to_string())
            },
            Some(where_sql) => {

                let __where_sql = format!("AND ({})", where_sql);
                let inner_where_sql = format!("WHERE {}", where_sql);

                (__where_sql, inner_where_sql)
            }
        };

        let order_by_sql: Option<&str> = $order_by_sql;
        let order_by_sql = match order_by_sql {
            None => "".to_string(),
            Some(order_by_sql) => format!("ORDER BY {}", order_by_sql)
        };

        format!("
            {select_sql}
            WHERE
                {not_in} NOT IN (
                    {inner_select_sql}
                    {inner_where_sql}
                    {order_by_sql}
                    LIMIT {offset}
                )
            {where_sql}
            {order_by_sql}
            LIMIT {per_page};
            ",

            select_sql = $select_sql,
            not_in = $not_in,
            inner_select_sql = $inner_select_sql,
            inner_where_sql = inner_where_sql,
            where_sql = where_sql,
            order_by_sql = order_by_sql,
            offset = offset,
            per_page = per_page
        )

    }}
);

// TODO: remove
// macro_rules! pagination(
//     ($pre_sql:expr; $inner_select_sql:expr; $post_sql:expr; $not_in:expr; $per_page:expr; $offset:expr) => {{

//         use types;
//         // TODO: compile-time type check other args
//         let per_page: types::PerPage = $per_page;
//         let offset: types::Offset = $offset;

//         // TODO: limitations: $post_sql  cannot start with ORDER BY

//         format!(indoc!("
//             {pre_sql}
//             WHERE
//                 {not_in} NOT IN (
//             {inner_select_sql}
//                     WHERE
//             {post_sql}
//                     LIMIT {offset}
//                 )
//             AND
//             {post_sql}
//             LIMIT {per_page};\
//         "),
//             pre_sql = $pre_sql,
//             not_in = $not_in,
//             inner_select_sql = $inner_select_sql,
//             post_sql = $post_sql, offset = offset, per_page = per_page)

//     }}
// );

// TODO: fix this
// #[test]
// fn pagination_macro_test() {

//     let deck_id = 42;
//     let per_page = 25;
//     let offset = 0;

//     let query = format!(indoc!("
//         SELECT
//             DecksClosure.descendent
//         FROM
//             DecksClosure
//         INNER JOIN
//             Decks
//         ON DecksClosure.descendent = Decks.deck_id
//         WHERE
//             DecksClosure.descendent NOT IN (
//         SELECT
//             DecksClosure.descendent
//         FROM
//             DecksClosure
//         INNER JOIN
//             Decks
//         ON DecksClosure.descendent = Decks.deck_id
//                 WHERE
//             ancestor = {deck_id}
//         AND
//             depth = 1
//         ORDER BY
//             Decks.name
//         COLLATE NOCASE ASC
//                 LIMIT {offset}
//             )
//         AND
//             ancestor = {deck_id}
//         AND
//             depth = 1
//         ORDER BY
//             Decks.name
//         COLLATE NOCASE ASC
//         LIMIT {per_page};"), deck_id = deck_id, per_page = per_page, offset = offset);

//     let pre_sql = indoc!("
//         SELECT
//             DecksClosure.descendent
//         FROM
//             DecksClosure
//         INNER JOIN
//             Decks
//         ON DecksClosure.descendent = Decks.deck_id");

//     let post_sql = format!(indoc!("
//             ancestor = {deck_id}
//         AND
//             depth = 1
//         ORDER BY
//             Decks.name
//         COLLATE NOCASE ASC"), deck_id = deck_id);

//     let actual = pagination!(pre_sql; pre_sql; post_sql; "DecksClosure.descendent"; per_page; offset);

//     // println!("'{}'", query);
//     // println!("'{}'", actual);
//     assert_eq!(query, actual);
// }

/* API */

pub fn get_database(file_path: String) -> Database {

    let db_connection = match Connection::open(file_path.as_str()) {
        Err(why) => {
            // TODO: fix
            panic!("{}", why);
        }
        Ok(db_conn) => Arc::new(Mutex::new(db_conn)),
    };

    {

        // NOTE: by default sqlite v3.x does not enforce foreign_keys by default
        //       see: http://stackoverflow.com/a/9937992/412627

        db_write_lock!(db_conn; db_connection.clone());
        let db_conn: &Connection = db_conn;

        match db_conn.execute_batch("PRAGMA foreign_keys=ON;") {
            Err(why) => {
                // TODO: fix
                panic!("{}", why);
            },
            _ => {/* query sucessfully executed */},
        }

    };


    /* table setup */

    match tables::setup_database(db_connection.clone()) {
        Ok(_) => {}
        Err(why) => {
            handle_raw_api_error!(why);
            panic!("unable to set up database: {}", file_path);
        }
    }

    /* custom function */
    {

        db_write_lock!(db_conn; db_connection.clone());
        let db_conn: &Connection = db_conn;

        // TODO: move this somewhere
        match db_conn.create_scalar_function("raw_score", 2, true, raw_score) {
            Err(why) => {
                // TODO: fix
                panic!("{}", why);
            },
            _ => {

                // ensure custom scalar function was loaded

                let query = format!("
                    SELECT raw_score(0, 0);
                ");

                let maybe_result = db_conn.query_row(&query, &[], |row| -> f64 {
                    return row.get(0);
                });

                match maybe_result {
                    Err(why) => {
                        // TODO: fix
                        panic!("{}", why);
                    },
                    Ok(_/*result*/) => {
                        // TODO: assert result is 0.5, otherwise panic
                        // println!("result: {}", result);
                    }
                };
            }
        }

    };

    return db_connection;
}

// the higher the score, the more important it is to be reviewed
fn raw_score(ctx: &Context) -> SqliteResult<c_double> {

    // raw_score(success: int, fail: int) -> f64
    assert!(ctx.len() == 2, "called with unexpected number of arguments");

    let success: c_double = try!(ctx.get::<c_double>(0));
    let fail: c_double = try!(ctx.get::<c_double>(1));

    let total: c_double = success + fail;

    let lidstone: c_double = (fail + 0.5f64) / (total + 1.0f64);

    return Ok(lidstone);
}
