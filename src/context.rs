/* rust lib imports */

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::rc::Rc;
use std::cell::RefCell;

/* 3rd-party imports */

use guardian::{ArcRwLockReadGuardian, ArcRwLockWriteGuardian};

/* local imports */

use types::{DeckID, ItemCount};
use api::decks::Deck;
use database::Database;

/* ////////////////////////////////////////////////////////////////////////// */

/* Request Context */

pub enum LockState {
    Read(ArcRwLockReadGuardian<Database>),
    Write(ArcRwLockWriteGuardian<Database>)
}

pub struct Context {

    // TODO: need granular private access of fields

    pub global_lock: Arc<RwLock<Database>>,
    pub lock_state: Option<LockState>,
    pub lock_state_ref_read_count: u64,
    pub lock_state_ref_write_count: u64,

    pub root_deck_id: DeckID,
    pub request_uri: String,

    /* cache */

    pub should_cache: bool,
    pub decks: HashMap<DeckID, Deck>,
    pub deck_children_count: HashMap<DeckID, ItemCount>,
    // TODO: cache more resources
}

impl Context {

    pub fn new(global_lock: Arc<RwLock<Database>>) -> Self {
        Context {

            global_lock: global_lock,
            lock_state: None,
            lock_state_ref_read_count: 0,
            lock_state_ref_write_count: 0,

            root_deck_id: 1,
            request_uri: "".to_string(),

            // cache
            should_cache: false,
            decks: HashMap::new(),
            deck_children_count: HashMap::new()
        }
    }

    pub fn database(&self) -> Database {

        assert!(self.lock_state_ref_read_count > 0 || self.lock_state_ref_write_count > 0);
        assert!(self.lock_state.is_some());

        match self.lock_state {
            Some(ref lock_state) => {
                match *lock_state {
                    LockState::Read(ref db) => {
                        (*db).clone()
                    },
                    LockState::Write(ref db) => {
                        (*db).clone()
                    }
                }
            },
            None => panic!(),
        }
    }

    fn __read_lock(&mut self) {

        assert!(self.lock_state_ref_read_count == 0);

        match self.lock_state {
            None => {},
            Some(ref lock_state) => {

                match *lock_state {
                    LockState::Read(_) => {
                        return;
                    },
                    LockState::Write(_) => {
                        panic!();
                    }
                }
            }
        }

        let r_lock = ArcRwLockReadGuardian::take(self.global_lock.clone()).unwrap();

        self.lock_state = Some(LockState::Read(r_lock));
        self.lock_state_ref_read_count = 1;
    }

    pub fn is_read_locked(&self) -> bool {
        match self.lock_state {
            None => {
                false
            },
            Some(_) => {
                // NOTE: may be able to do API reads on a write lock
                true
            }
        }
    }

    fn __write_lock(&mut self) {

        assert!(self.lock_state_ref_read_count == 0);
        assert!(self.lock_state_ref_write_count == 0);
        assert!(self.lock_state.is_none());

        let w_lock = ArcRwLockWriteGuardian::take(self.global_lock.clone()).unwrap();

        self.lock_state = Some(LockState::Write(w_lock));
        self.lock_state_ref_write_count = 1;
    }

    pub fn is_write_locked(&self) -> bool {
        match self.lock_state {
            None => {
                false
            },
            Some(ref lock_state) => {

                match *lock_state {
                    LockState::Read(_) => {
                        false
                    },
                    LockState::Write(_) => {
                        true
                    }
                }
            }
        }
    }

}

enum ContextLockGuardState {
    Read,
    Write
}

pub struct ContextLockGuard(ContextLockGuardState, Rc<RefCell<Context>>);

impl Drop for ContextLockGuard {
    fn drop(&mut self) {


        match self.0 {
            ContextLockGuardState::Read => {

                assert!(self.1.borrow().lock_state_ref_read_count > 0);

                self.1.borrow_mut().lock_state_ref_read_count -= 1;
            },
            ContextLockGuardState::Write => {

                assert!(self.1.borrow().lock_state_ref_write_count > 0);

                self.1.borrow_mut().lock_state_ref_write_count -= 1;
            }
        }

        assert!(!self.1.borrow().global_lock.is_poisoned());

        let ref_read_count = self.1.borrow().lock_state_ref_read_count;
        let ref_write_count = self.1.borrow().lock_state_ref_write_count;

        if ref_read_count <= 0 && ref_write_count <= 0 {
            self.1.borrow_mut().lock_state = None;
        }

    }
}

pub fn read_lock(context: Rc<RefCell<Context>>) -> ContextLockGuard {

    let should_read_lock = match context.borrow().lock_state {
        None => {
            true
        },
        Some(_) => {
            false
        }
    };

    if should_read_lock {

        assert!(context.borrow().lock_state_ref_read_count == 0);

        context.borrow_mut().__read_lock();

        assert!(context.borrow().lock_state_ref_read_count == 1);
        assert!(context.borrow().is_read_locked());

    } else {

        assert!(context.borrow().lock_state_ref_read_count > 0 || context.borrow().lock_state_ref_write_count > 0);

        context.borrow_mut().lock_state_ref_read_count += 1;
    }

    return ContextLockGuard(ContextLockGuardState::Read, context.clone());
}

pub fn write_lock(context: Rc<RefCell<Context>>) -> ContextLockGuard {

    let should_write_lock = match context.borrow().lock_state {
        None => {
            true
        },
        Some(ref lock_state) => {
            match *lock_state {
                LockState::Read(_) => {
                    panic!();
                },
                LockState::Write(_) => {
                    assert!(context.borrow().lock_state_ref_write_count > 0);
                    assert!(!context.borrow().global_lock.is_poisoned());
                    false
                }
            }
        }
    };

    if should_write_lock {

        assert!(context.borrow().lock_state_ref_write_count == 0);

        context.borrow_mut().__write_lock();

        assert!(context.borrow().lock_state_ref_write_count == 1);
        assert!(context.borrow().is_write_locked());

    } else {

        assert!(context.borrow().lock_state_ref_write_count > 0);

        context.borrow_mut().lock_state_ref_write_count += 1;
    }

    return ContextLockGuard(ContextLockGuardState::Write, context.clone());
}
