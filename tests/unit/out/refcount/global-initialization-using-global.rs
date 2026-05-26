extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static s_first: Value<i32> = <Value<i32>>::default();
);
thread_local!(
    pub static s_second: Value<i32> =
        Rc::new(RefCell::new(((*s_first.with(Value::clone).borrow()) + 1)));
);
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(((*s_first.with(Value::clone).borrow()) == 0));
    assert!(
        ((*s_second.with(Value::clone).borrow()) == ((*s_first.with(Value::clone).borrow()) + 1))
    );
    return 0;
}
