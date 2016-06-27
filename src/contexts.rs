/* rust lib imports */
use std::path::{Path};
use std::sync::{Arc, Mutex, RwLock};

/* 3rd-party imports */

use rusqlite::{Connection};

use regex::{Captures};

/* local imports */

use route::constants::AppRoute;

////////////////////////////////////////////////////////////////////////////////

pub struct GlobalContext<'a> {
    pub assets_root_path: &'a Path,

    pub root_deck_id: i64,

    // RwLock => ORM operations read/write lock
    // Mutex => database low-level lock
    pub db_connection: Arc<RwLock<Mutex<Connection>>>
    // db_ops_lock: Arc<RwLock<bool>>
}

// TODO: remove
// impl GlobalContext<'a> {
// }

pub struct Context<

    // global context
    'global,

    // router
    'regexset,

    // hyper lifetimes
    // 'request, 'network_stream: 'request,
    // 'response
    > {

    pub global_context: &'global GlobalContext<'global>,

    pub uri: &'regexset str,
    pub captures: Option<Captures<'regexset>>,

    pub view_route: AppRoute,

    // request: Request<'request, 'network_stream>,
    // response: Response<'response>


}

// impl<'a, 'b> Context<'a, 'b> {

//     /* deck API */

//     fn deck_read(&self, deck_id: u64) {

//         // lock database for read operation
//         db_read_lock!(self.global_context.db_connection);
//         // let db_op_lock = self.global_context.db_connection.read().unwrap();
//         // let db_lock = db_op_lock.lock().unwrap();
//     }
// }
