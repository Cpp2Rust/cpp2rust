extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct HasTwoInts {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
thread_local!(
    pub static count_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CountsPerMember_refHasTwoInts_first_ {}
thread_local!(
    pub static count_1: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CountsPerMember_refHasTwoInts_second_ {}
thread_local!(
    pub static count_2: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CountsPerMember_nullptr_ {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
    assert!((count_0.with(|rc| *rc.borrow()) == 1));
    assert!((count_1.with(|rc| *rc.borrow()) == 0));
    assert!((count_2.with(|rc| *rc.borrow()) == 0));
    count_1.with(|rc| *rc.borrow_mut() += 2);
    assert!((count_0.with(|rc| *rc.borrow()) == 1));
    assert!((count_1.with(|rc| *rc.borrow()) == 2));
    assert!((count_2.with(|rc| *rc.borrow()) == 0));
    count_2.with(|rc| *rc.borrow_mut() += 3);
    assert!((count_0.with(|rc| *rc.borrow()) == 1));
    assert!((count_1.with(|rc| *rc.borrow()) == 2));
    assert!((count_2.with(|rc| *rc.borrow()) == 3));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = count_0.with(|_| ());
    let _ = count_1.with(|_| ());
    let _ = count_2.with(|_| ());
}
