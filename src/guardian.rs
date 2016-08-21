extern crate guardian;

use std::cell::RefCell;
use std::sync::{Arc, Mutex, RwLock};
use std::rc::Rc;

use guardian::{ArcRwLockReadGuardian, ArcRwLockWriteGuardian};

// stub
struct Connection;

type Database = Arc<Mutex<Connection>>;

enum LockState {
    Read(ArcRwLockReadGuardian<Database>),
    Write(ArcRwLockWriteGuardian<Database>)
}

struct Context {
    global_lock: Arc<RwLock<Database>>,
    lock_state: Option<LockState>,
    lock_state_ref_count: i64
}

impl Context {
    fn new(global_lock: Arc<RwLock<Database>>) -> Self {
        Context {
            global_lock: global_lock,
            lock_state: None,
            // 0 := unlocked
            // 1 or greater := locked for reads
            // -1 or less := locked for writes
            lock_state_ref_count: 0
        }
    }

    fn database(&self) -> Database {

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

        assert!(self.lock_state_ref_count == 0);

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
        self.lock_state_ref_count = 1;
    }

    fn is_read_locked(&self) -> bool {
        match self.lock_state {
            None => {
                false
            },
            Some(ref lock_state) => {

                match *lock_state {
                    LockState::Read(_) => {
                        true
                    },
                    LockState::Write(_) => {
                        false
                    }
                }
            }
        }
    }

    fn __write_lock(&mut self) {

        assert!(self.lock_state_ref_count == 0);
        assert!(self.lock_state.is_none());

        let w_lock = ArcRwLockWriteGuardian::take(self.global_lock.clone()).unwrap();

        self.lock_state = Some(LockState::Write(w_lock));
        self.lock_state_ref_count = -1;
    }

    fn is_write_locked(&self) -> bool {
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

struct ContextLockGuard(ContextLockGuardState, Rc<RefCell<Context>>);

impl Drop for ContextLockGuard {
    fn drop(&mut self) {


        match self.0 {
            ContextLockGuardState::Read => {

                assert!(self.1.borrow().lock_state_ref_count > 0);

                self.1.borrow_mut().lock_state_ref_count -= 1;

                assert!(!self.1.borrow().global_lock.is_poisoned());

                let ref_count = self.1.borrow().lock_state_ref_count;

                if ref_count <= 0 {
                    self.1.borrow_mut().lock_state = None;
                }

            },
            ContextLockGuardState::Write => {

                assert!(self.1.borrow().lock_state_ref_count < 0);

                self.1.borrow_mut().lock_state_ref_count += 1;

                assert!(!self.1.borrow().global_lock.is_poisoned());

                let ref_count = self.1.borrow().lock_state_ref_count;

                if ref_count >= 0 {
                    self.1.borrow_mut().lock_state = None;
                }
            }
        }

    }
}

fn read_lock(context: Rc<RefCell<Context>>) -> ContextLockGuard {

    let should_read_lock = match context.borrow().lock_state {
        None => {
            true
        },
        Some(ref lock_state) => {
            match *lock_state {
                LockState::Read(_) => {
                    assert!(!context.borrow().global_lock.is_poisoned());
                    false
                },
                LockState::Write(_) => {
                    panic!();
                }
            }
        }
    };

    assert!(context.borrow().lock_state_ref_count >= 0);

    if should_read_lock {

        assert!(context.borrow().lock_state_ref_count == 0);

        context.borrow_mut().__read_lock();

        assert!(context.borrow().is_read_locked());

    } else {

        assert!(context.borrow().lock_state_ref_count > 0);

        context.borrow_mut().lock_state_ref_count += 1;
    }

    return ContextLockGuard(ContextLockGuardState::Read, context.clone());
}

fn write_lock(context: Rc<RefCell<Context>>) -> ContextLockGuard {

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
                    assert!(!context.borrow().global_lock.is_poisoned());
                    false
                }
            }
        }
    };

    assert!(context.borrow().lock_state_ref_count <= 0);

    if should_write_lock {

        assert!(context.borrow().lock_state_ref_count == 0);

        context.borrow_mut().__write_lock();

        assert!(context.borrow().is_write_locked());

    } else {

        assert!(context.borrow().lock_state_ref_count < 0);

        context.borrow_mut().lock_state_ref_count -= 1;
    }

    return ContextLockGuard(ContextLockGuardState::Write, context.clone());
}

fn main() {

    let database = Arc::new(Mutex::new(Connection));
    let global_lock = Arc::new(RwLock::new(database.clone()));

    drop(database);

    {
        let global_lock = global_lock.clone();

        let context = Context::new(global_lock.clone());
        let context = Rc::new(RefCell::new(context));

        {
            let _guard = read_lock(context.clone());

            let context = context.clone();
            let _db = context.borrow().database();

            {
                let _guard = read_lock(context.clone());

                let context = context.clone();
                let _db = context.borrow().database();

            };

            let global_lock = global_lock.clone();
            assert!(global_lock.try_write().is_err());
        };

        {
            let global_lock = global_lock.clone();
            assert!(global_lock.try_write().is_ok());
        };

        {
            let _guard = write_lock(context.clone());

            let context = context.clone();
            let _db = context.borrow().database();

            {
                let _guard = write_lock(context.clone());

                let context = context.clone();
                let _db = context.borrow().database();

                println!("{:?}", context.borrow().lock_state_ref_count);
            };

            let global_lock = global_lock.clone();
            assert!(global_lock.try_read().is_err());
        };

        {
            let global_lock = global_lock.clone();
            assert!(global_lock.try_read().is_ok());
        };

        println!("{:?}", context.borrow().lock_state_ref_count);


    };

}
