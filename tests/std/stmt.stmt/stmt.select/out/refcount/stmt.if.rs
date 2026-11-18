extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static ctorCount_0: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static copyCount_1: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static dtorCount_2: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static useCount_3: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(4)]
pub struct LifetimeTracked {
    #[offset(0)]
    pub uses: i32,
}
impl LifetimeTracked {
    pub fn reset() {
        ctorCount_0.with(|rc| {
            *rc.borrow_mut() = {
                copyCount_1.with(|rc| {
                    *rc.borrow_mut() = {
                        dtorCount_2.with(|rc| {
                            *rc.borrow_mut() = {
                                useCount_3.with(|rc| *rc.borrow_mut() = 0);
                                useCount_3.with(|rc| *rc.borrow())
                            }
                        });
                        dtorCount_2.with(|rc| *rc.borrow())
                    }
                });
                copyCount_1.with(|rc| *rc.borrow())
            }
        });
    }
    pub fn new() -> Self {
        let __this: LifetimeTracked = Self { uses: 0 };
        (*ctorCount_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(_a0: Ptr<LifetimeTracked>) -> Self {
        let __this: LifetimeTracked = Self { uses: 0 };
        (*copyCount_1.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for LifetimeTracked {
    fn clone(&self) -> Self {
        let __src: Value<LifetimeTracked> = Rc::new(RefCell::new(LifetimeTracked {
            uses: self.uses.clone(),
        }));
        LifetimeTracked::copy_from(__src.as_pointer())
    }
}
impl Default for LifetimeTracked {
    fn default() -> Self {
        { LifetimeTracked::new() }
    }
}
thread_local!(
    pub static then_runs_4: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static else_runs_5: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn test_6() {
    ({ LifetimeTracked::reset() });
    let mut choice: i32 = 9;
    {
        let guard: Value<LifetimeTracked> = Rc::new(RefCell::new(LifetimeTracked::new()));
        let _dtor_guard =
            std::mem::DropGuard::new(guard.clone(), |__p| __p.as_pointer().destructor());
        let mut choice: i32 = 0;
        if (choice != 0) {
            (*then_runs_4.with(Value::clone).borrow_mut()).prefix_inc();
        } else {
            (*else_runs_5.with(Value::clone).borrow_mut()).prefix_inc();
            ({ LifetimeTrackedImpl::use_(&guard.as_pointer()) });
            assert!(({ ({ (*guard.borrow()).uses } == 1) }));
            assert!((choice == 0));
            assert!((dtorCount_2.with(|rc| *rc.borrow()) == 0));
        }
    }
    assert!((then_runs_4.with(|rc| *rc.borrow()) == 0));
    assert!((else_runs_5.with(|rc| *rc.borrow()) == 1));
    assert!((dtorCount_2.with(|rc| *rc.borrow()) == 1));
    {
        let guard: Value<LifetimeTracked> = Rc::new(RefCell::new(LifetimeTracked::new()));
        let _dtor_guard =
            std::mem::DropGuard::new(guard.clone(), |__p| __p.as_pointer().destructor());
        let mut choice: i32 = 4;
        if (choice != 0) {
            (*then_runs_4.with(Value::clone).borrow_mut()).prefix_inc();
            ({ LifetimeTrackedImpl::use_(&guard.as_pointer()) });
            assert!(({ ({ (*guard.borrow()).uses } == 1) }));
            assert!((choice == 4));
            assert!((dtorCount_2.with(|rc| *rc.borrow()) == 1));
        }
    }
    assert!((then_runs_4.with(|rc| *rc.borrow()) == 1));
    assert!((ctorCount_0.with(|rc| *rc.borrow()) == 2));
    assert!((dtorCount_2.with(|rc| *rc.borrow()) == 2));
    assert!((choice == 9));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_6() });
    return 0;
}
pub trait LifetimeTrackedImpl {
    fn destructor(&self);
    fn use_(&self);
    fn copy_assign(&self, _a0: Ptr<LifetimeTracked>) -> Ptr<LifetimeTracked>;
}
impl LifetimeTrackedImpl for Ptr<LifetimeTracked> {
    fn destructor(&self) {
        (*dtorCount_2.with(Value::clone).borrow_mut()).prefix_inc();
    }
    fn use_(&self) {
        field!((*self), uses).with_mut(|__v| {
            __v.prefix_inc();
        });
        (*useCount_3.with(Value::clone).borrow_mut()).prefix_inc();
    }
    fn copy_assign(&self, _a0: Ptr<LifetimeTracked>) -> Ptr<LifetimeTracked> {
        field!((*self), uses).write({ _a0.with(|__s| __s.uses) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = ctorCount_0.with(|_| ());
    let _ = copyCount_1.with(|_| ());
    let _ = dtorCount_2.with(|_| ());
    let _ = useCount_3.with(|_| ());
    let _ = then_runs_4.with(|_| ());
    let _ = else_runs_5.with(|_| ());
}
