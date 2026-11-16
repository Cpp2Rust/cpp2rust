extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static initial_0: Value<i32> = Rc::new(RefCell::new(100));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct HasStaticTotal {
    #[offset(0)]
    pub held: i32,
}
impl Default for HasStaticTotal {
    fn default() -> Self {
        { HasStaticTotal { held: 1 } }
    }
}
thread_local!(
    pub static initial_1: Value<i32> = Rc::new(RefCell::new(6));
);
thread_local!(
    pub static total_2: Value<i32> = Rc::new(RefCell::new((initial_1.with(|rc| *rc.borrow()) + 1)));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((total_2.with(|rc| *rc.borrow()) == 7));
    let object: Value<HasStaticTotal> = Rc::new(RefCell::new(<HasStaticTotal>::default()));
    assert!((({ HasStaticTotalImpl::add(&object.as_pointer(), 4,) }) == 5));
    assert!((total_2.with(|rc| *rc.borrow()) == 13));
    assert!((initial_1.with(|rc| *rc.borrow()) == 6));
    assert!((initial_0.with(|rc| *rc.borrow()) == 100));
    return 0;
}
pub trait HasStaticTotalImpl {
    fn add(&self, amount: i32) -> i32;
}
impl HasStaticTotalImpl for Ptr<HasStaticTotal> {
    fn add(&self, mut amount: i32) -> i32 {
        {
            let __rhs = amount;
            field!((*self), held).with_mut(|__v| *__v = *__v + __rhs)
        };
        total_2.with(|rc| *rc.borrow_mut() += initial_1.with(|rc| *rc.borrow()));
        return (*self).with(|__s| __s.held);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = initial_0.with(|_| ());
    let _ = initial_1.with(|_| ());
    let _ = total_2.with(|_| ());
}
