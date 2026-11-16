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
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let scoped: Value<LifetimeTracked> = Rc::new(RefCell::new(LifetimeTracked::new()));
    let _dtor_scoped =
        std::mem::DropGuard::new(scoped.clone(), |__p| __p.as_pointer().destructor());
    assert!((ctorCount_0.with(|rc| *rc.borrow()) == 1));
    {
        let mut index: i32 = 0;
        'loop_: while (index < 3) {
            let scoped: Value<LifetimeTracked> = Rc::new(RefCell::new(LifetimeTracked::new()));
            let _dtor_scoped =
                std::mem::DropGuard::new(scoped.clone(), |__p| __p.as_pointer().destructor());
            index.prefix_inc();
        }
    }
    assert!((ctorCount_0.with(|rc| *rc.borrow()) == 4));
    assert!((dtorCount_2.with(|rc| *rc.borrow()) == 3));
    if (ctorCount_0.with(|rc| *rc.borrow()) == 4) {
        let scoped: Value<LifetimeTracked> = Rc::new(RefCell::new(LifetimeTracked::new()));
        let _dtor_scoped =
            std::mem::DropGuard::new(scoped.clone(), |__p| __p.as_pointer().destructor());
    }
    assert!((ctorCount_0.with(|rc| *rc.borrow()) == 5));
    assert!((dtorCount_2.with(|rc| *rc.borrow()) == 4));
    if (ctorCount_0.with(|rc| *rc.borrow()) == 0) {
    } else {
        let scoped: Value<LifetimeTracked> = Rc::new(RefCell::new(LifetimeTracked::new()));
        let _dtor_scoped =
            std::mem::DropGuard::new(scoped.clone(), |__p| __p.as_pointer().destructor());
    }
    assert!((ctorCount_0.with(|rc| *rc.borrow()) == 6));
    assert!((dtorCount_2.with(|rc| *rc.borrow()) == 5));
    ({ LifetimeTrackedImpl::use_(&scoped.as_pointer()) });
    assert!(({ ({ (*scoped.borrow()).uses } == 1) }));
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
        field!((*self), uses).with_mut(|__v| __v.prefix_inc());
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
}
