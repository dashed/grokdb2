/* rust lib imports */
use std::fmt;
use std::error;
use std::path::{Path};
use std::sync::{Arc, Mutex, RwLock};

/* 3rd-party imports */

use rusqlite::{Connection, Error as SqliteError};

use regex::{Captures};

/* local imports */

use route::constants::AppRoute;

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct QueryError {
    pub sqlite_error: SqliteError,
    pub query: String,
}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} \nFor query:\n{}", self.sqlite_error, self.query)
    }
}

impl error::Error for QueryError {
    fn description(&self) -> &str {
        return self.sqlite_error.description();
    }
}

macro_rules! ____db_read_lock(
    ($e:expr) => (

        // match $e.global_context.db_connection.read() {
        //     Ok(db_op_lock) => {
        //         db_op_lock
        //         // match db_op_lock.lock() {
        //         //     Ok(db_lock) => db_lock,
        //         //     Err(why) => {
        //         //         panic!("db_read_lock/RwLock/Mutex {:?}", why);
        //         //     }
        //         // }
        //     },
        //     Err(why) => {
        //         panic!("db_read_lock/RwLock {:?}", why);
        //     }
        // }.lock().unwrap()


        // let db_op_lock = $e.global_context.db_connection.read().unwrap();
        // let db_lock = db_op_lock.lock().unwrap();

        {
            use std::sync::{Arc, Mutex, RwLock};
            use rusqlite::{Connection};

            // hacky type checking
            let _: Arc<RwLock<Mutex<Connection>>> = $e;
        };

        let db_op_lock = $e.read().unwrap();
        let db_conn_guard = db_op_lock.lock().unwrap();
        let ref $ident = *db_conn_guard;

        // db_op_lock.lock().unwrap()



        // db_lock
        // match $e {
        //     Ok(v) => v,
        //     Err(e) => { println!("Error: {}", e); return; }
        // }
    )
);

// lock critical section for reads only at API level
macro_rules! api_read_lock(
    ($ident:ident; $e:expr) => (

        {
            use std::sync::{Arc, Mutex, RwLock};
            use rusqlite::{Connection};

            // hacky type checking
            let _: Arc<RwLock<Mutex<Connection>>> = $e;
        };

        let $ident = $e.read().unwrap();
    )
);

macro_rules! db_read_lock(
    ($ident:ident; $e:expr) => (

        {
            use std::sync::{Arc, Mutex, RwLock};
            use rusqlite::{Connection};

            // hacky type checking
            let _: Arc<RwLock<Mutex<Connection>>> = $e;
        };

        let db_op_lock = $e.read().unwrap();
        let db_conn_guard = db_op_lock.lock().unwrap();
        let ref $ident = *db_conn_guard;
    )
);


macro_rules! db_write_lock(
    ($ident:ident; $e:expr) => (

        {
            use std::sync::{Arc, Mutex, RwLock};
            use rusqlite::{Connection};

            // hacky type checking
            let _: Arc<RwLock<Mutex<Connection>>> = $e;
        };

        let db_op_lock = $e.write().unwrap();
        let db_conn_guard = db_op_lock.lock().unwrap();
        let ref $ident = *db_conn_guard;
    )
);
