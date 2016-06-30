/* rust lib imports */
use std::path::{Path};
use std::sync::{Arc, Mutex, RwLock};
use std::collections::{HashMap, HashSet, BTreeMap};

/* 3rd-party imports */

use rusqlite::{Connection};

use regex::{Captures};

/* local imports */

use route::constants::{DeckID, AppRoute};
use decks::{Deck};

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

    pub api: APIContext<'global>,

    pub uri: &'regexset str,
    pub captures: Option<Captures<'regexset>>,

    pub view_route: AppRoute,

    // request: Request<'request, 'network_stream>,
    // response: Response<'response>

}

pub struct Cache {
    pub decks: HashMap<DeckID, Deck>
}

impl Cache {
    fn new() -> Self {
        return Cache {
            decks: HashMap::new()
        };
    }
}

pub struct APIContext<'global> {
    pub global_context: &'global GlobalContext<'global>,
    pub should_cache: bool,

    pub cache: Cache
}

impl<'global> APIContext<'global> {

    pub fn new(global_context: &'global GlobalContext<'global>) -> Self {
        return APIContext {
            global_context: global_context,
            should_cache: false,
            cache: Cache::new()
        };
    }

    pub fn should_cache(&mut self, should_cache: bool) {
        self.should_cache = should_cache;
    }
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
