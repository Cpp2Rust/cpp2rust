extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static stock_0: Value<i32> = Rc::new(RefCell::new(100));
);
pub fn minimum_1() -> i32 {
    return 50;
}
pub type anon_2 = u32;
pub const anon_2_reserve: anon_2 = 30;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct HasLaterMembers {
    #[offset(0)]
    pub stock: i32,
}
impl Default for HasLaterMembers {
    fn default() -> Self {
        { HasLaterMembers { stock: 9 } }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let object: Value<HasLaterMembers> = Rc::new(RefCell::new(<HasLaterMembers>::default()));
    assert!((({ HasLaterMembersImpl::take(&object.as_pointer(), 3,) }) == 38));
    assert!(({ ({ (*object.borrow()).stock } == 6) }));
    assert!((stock_0.with(|rc| *rc.borrow()) == 100));
    return 0;
}
pub trait HasLaterMembersImpl {
    fn take(&self, amount: i32) -> i32;
    fn minimum(&self) -> i32;
}
impl HasLaterMembersImpl for Ptr<HasLaterMembers> {
    fn take(&self, mut amount: i32) -> i32 {
        {
            let __rhs = amount;
            field!((*self), stock).with_mut(|__v| *__v = *__v - __rhs)
        };
        return (((*self).with(|__s| __s.stock) + ({ HasLaterMembersImpl::minimum(self) }))
            + (anon_2_reserve as i32));
    }
    fn minimum(&self) -> i32 {
        return 2;
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = stock_0.with(|_| ());
}
