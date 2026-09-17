extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static first_0: Value<i32> = <Value<i32>>::default();
);
thread_local!(
    pub static second_1: Value<i32> =
        Rc::new(RefCell::new(((*first_0.with(Value::clone).borrow()) + 1)));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(((*first_0.with(Value::clone).borrow()) == 0));
    assert!(
        ((*second_1.with(Value::clone).borrow()) == ((*first_0.with(Value::clone).borrow()) + 1))
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = first_0.with(|_| ());
    let _ = second_1.with(|_| ());
}
