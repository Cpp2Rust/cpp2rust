extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static first: Value<i32> = <Value<i32>>::default();
);
thread_local!(
    pub static second: Value<i32> =
        Rc::new(RefCell::new(((*first.with(Value::clone).borrow()) + 1)));
);
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(((*first.with(Value::clone).borrow()) == 0));
    assert!(((*second.with(Value::clone).borrow()) == ((*first.with(Value::clone).borrow()) + 1)));
    return 0;
}
