/* rust lib imports */
use std::path::{Path};
use std::sync::{Arc, Mutex, RwLock};

/* 3rd-party imports */

use rusqlite::{Connection};

use regex::{Captures};

/* local imports */

use route::constants::AppRoute;

////////////////////////////////////////////////////////////////////////////////


macro_rules! db_read_lock(
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
        let _ = db_op_lock.lock().unwrap();


        // db_op_lock.lock().unwrap()



        // db_lock
        // match $e {
        //     Ok(v) => v,
        //     Err(e) => { println!("Error: {}", e); return; }
        // }
    )
);

macro_rules! db_write_lock(
    ($e:expr) => (

        {
            // hacky type checking
            let _: Arc<RwLock<Mutex<Connection>>> = $e;
        };

        let db_op_lock = $e.write().unwrap();
        let _ = db_op_lock.lock().unwrap();

    )
);
