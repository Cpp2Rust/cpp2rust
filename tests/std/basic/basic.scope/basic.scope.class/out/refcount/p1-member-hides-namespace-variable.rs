extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static balance_0: Value<i32> = Rc::new(RefCell::new(50));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct HasBalance {
    #[offset(0)]
    pub balance: i32,
}
impl Default for HasBalance {
    fn default() -> Self {
        { HasBalance { balance: 7 } }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let object: Value<HasBalance> = Rc::new(RefCell::new(<HasBalance>::default()));
    assert!((({ HasBalanceImpl::deposit(&object.as_pointer(), 3,) }) == 10));
    assert!(({ ({ (*object.borrow()).balance } == 10) }));
    assert!((balance_0.with(|rc| *rc.borrow()) == 50));
    assert!((({ HasBalanceImpl::deposit_globally(&object.as_pointer(), 4,) }) == 54));
    assert!(({ ({ (*object.borrow()).balance } == 10) }));
    assert!((balance_0.with(|rc| *rc.borrow()) == 54));
    return 0;
}
pub trait HasBalanceImpl {
    fn deposit(&self, amount: i32) -> i32;
    fn deposit_globally(&self, amount: i32) -> i32;
}
impl HasBalanceImpl for Ptr<HasBalance> {
    fn deposit(&self, mut amount: i32) -> i32 {
        {
            let __rhs = amount;
            field!((*self), balance).with_mut(|__v| *__v = *__v + __rhs)
        };
        return (*self).with(|__s| __s.balance);
    }
    fn deposit_globally(&self, mut amount: i32) -> i32 {
        balance_0.with(|rc| *rc.borrow_mut() += amount);
        return balance_0.with(|rc| *rc.borrow());
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = balance_0.with(|_| ());
}
