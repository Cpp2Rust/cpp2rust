extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(8)]
    pub val: Option<Value<i32>>,
}
impl Holder {
    pub fn move_from(_a0: Ptr<Holder>) -> Self {
        Self {
            val: field!(_a0, val).with_mut(|__v: &mut Option<Value<i32>>| __v.take()),
        }
    }
}
pub fn read_val_0(mut h: Ptr<Holder>) -> i32 {
    return (*h.with(|__s| __s.val.clone()).as_ref().unwrap().borrow());
}
pub fn write_val_1(mut h: Ptr<Holder>, mut v: i32) {
    (*h.with(|__s| __s.val.clone()).as_ref().unwrap().borrow_mut()) = v;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let h: Value<Holder> = Rc::new(RefCell::new(<Holder>::default()));
    (field_ptr!(h.as_pointer(), val) as Ptr<Option<Value<i32>>>)
        .write(Some(Rc::new(RefCell::new(10))).take());
    ({ write_val_1((h.as_pointer()), 42) });
    assert!((({ read_val_0((h.as_pointer()),) }) == 42));
    return 0;
}
pub trait HolderImpl {
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder>;
}
impl HolderImpl for Ptr<Holder> {
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder> {
        (field_ptr!((*self), val) as Ptr<Option<Value<i32>>>)
            .write(field!(_a0, val).with_mut(|__v: &mut Option<Value<i32>>| __v.take()));
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
