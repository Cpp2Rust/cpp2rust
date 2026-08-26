extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(v: Ptr<i32>) -> i32 {
    return (v.read());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let __tmp_0: Value<i32> = Rc::new(RefCell::new(5));
    let i2: Ptr<i32> = __tmp_0.as_pointer();
    assert!(((i2.read()) == 5));
    let i3: Value<i32> = Rc::new(RefCell::new(({ foo_0(i2) })));
    assert!(((*i3.borrow()) == 5));
    assert!(
        (({
            let _v: Value<i32> = Rc::new(RefCell::new(5));
            foo_0(_v.as_pointer())
        }) == 5)
    );
    return 0;
}
