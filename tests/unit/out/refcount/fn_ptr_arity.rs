extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(
    a1: i32,
    a2: i32,
    a3: i32,
    a4: i32,
    a5: i32,
    a6: i32,
    a7: i32,
    a8: i32,
    a9: i32,
    a10: i32,
    a11: i32,
    a12: i32,
    a13: i32,
    a14: i32,
) -> i32 {
    let a1: Value<i32> = Rc::new(RefCell::new(a1));
    let a2: Value<i32> = Rc::new(RefCell::new(a2));
    let a3: Value<i32> = Rc::new(RefCell::new(a3));
    let a4: Value<i32> = Rc::new(RefCell::new(a4));
    let a5: Value<i32> = Rc::new(RefCell::new(a5));
    let a6: Value<i32> = Rc::new(RefCell::new(a6));
    let a7: Value<i32> = Rc::new(RefCell::new(a7));
    let a8: Value<i32> = Rc::new(RefCell::new(a8));
    let a9: Value<i32> = Rc::new(RefCell::new(a9));
    let a10: Value<i32> = Rc::new(RefCell::new(a10));
    let a11: Value<i32> = Rc::new(RefCell::new(a11));
    let a12: Value<i32> = Rc::new(RefCell::new(a12));
    let a13: Value<i32> = Rc::new(RefCell::new(a13));
    let a14: Value<i32> = Rc::new(RefCell::new(a14));
    return 22;
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let f: Value<
        FnPtr<fn(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) -> i32>,
    > =
        Rc::new(RefCell::new(
            (FnPtr::<
                fn(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) -> i32,
            >::new(foo_0)),
        ));
    assert!((({ (*(*f.borrow()))(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,) }) == 22));
    return 0;
}
