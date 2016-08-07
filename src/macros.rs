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
