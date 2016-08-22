/* rust lib imports */

use std::sync::{Arc, Mutex, RwLock};

/* 3rd-party imports */

use rusqlite::Connection;

/* local imports */

use tables;

/* ////////////////////////////////////////////////////////////////////////// */

// Arc := Shared resource between threads
// Mutex := Raw database operation
pub type Database = Arc<Mutex<Connection>>;

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

/* API */

pub fn get_database(file_path: String) -> Database {

    let db_connection = match Connection::open(file_path.as_str()) {
        Err(why) => {
            // TODO: fix
            panic!("{}", why);
        }
        Ok(db_conn) => Arc::new(Mutex::new(db_conn)),
    };

    /* table setup */

    match tables::setup_database(db_connection.clone()) {
        Ok(_) => {}
        Err(why) => {
            handle_raw_api_error!(why);
            panic!("unable to set up database: {}", file_path);
        }
    }

    return db_connection;
}
