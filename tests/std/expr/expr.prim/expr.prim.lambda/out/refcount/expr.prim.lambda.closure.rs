extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    let twice: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |mut x: i32| -> i32 {
            return (x * 2);
        })));
    let mut n: i32 = 4;
    assert!((n == 4));
    assert!(({ (({ (*twice.borrow()).spec::<fn(i32) -> i32>(0).call(3,) }) == 6) }));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
