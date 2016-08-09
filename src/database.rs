/* rust lib imports */

use std::sync::{Arc, Mutex, RwLock};

/* 3rd-party imports */

use rusqlite::Connection;

/* local imports */

use types::Database;
use tables;

/* ////////////////////////////////////////////////////////////////////////// */

pub fn get_database(file_path: String) -> Database {

    let db_connection = match Connection::open(file_path.as_str()) {
        Err(why) => {
            // TODO: fix
            panic!("{}", why);
        }
        Ok(db_conn) => Arc::new(RwLock::new(Mutex::new(db_conn))),
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
