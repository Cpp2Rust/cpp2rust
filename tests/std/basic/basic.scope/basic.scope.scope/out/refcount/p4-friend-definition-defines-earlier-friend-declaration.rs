extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static friend_calls_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct DeclaresFriend {}
pub fn shared_friend_1() -> i32 {
    (*friend_calls_0.with(Value::clone).borrow_mut()).prefix_inc();
    return 7;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct DefinesFriend {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ shared_friend_1() }) == 7));
    assert!((friend_calls_0.with(|rc| *rc.borrow()) == 1));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = friend_calls_0.with(|_| ());
}
