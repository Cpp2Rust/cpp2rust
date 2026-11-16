extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    let mut total: i32 = 4;
    let bump: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let total: Value<i32> = Rc::new(RefCell::new((total + 1)));
        },
        || -> i32 {
            return (*total.borrow_mut()).prefix_inc();
        }
    )));
    assert!((total == 4));
    assert!(({ (({ (*bump.borrow()).call() }) == 6) }));
    assert!(({ (({ (*bump.borrow()).call() }) == 7) }));
    assert!((total == 4));
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
