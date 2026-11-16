extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static shared_0: Value<i32> = Rc::new(RefCell::new(3));
);
thread_local!(
    pub static calls_1: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct HasStaticMembers {}
impl HasStaticMembers {
    pub fn run() {
        (*calls_1.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut object_pointer: Ptr<i32> = (shared_0.with(|v| v.as_pointer()));
    assert!(((object_pointer.read()) == 3));
    object_pointer.write(4);
    assert!((shared_0.with(|rc| *rc.borrow()) == 4));
    let mut function_pointer: FnPtr<fn()> = (FnPtr::<fn()>::new(HasStaticMembers::run));
    ({ function_pointer.call() });
    assert!((calls_1.with(|rc| *rc.borrow()) == 1));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = shared_0.with(|_| ());
    let _ = calls_1.with(|_| ());
}
