extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let zero: Value<_> = Rc::new(RefCell::new(
        (|| {
            return 42;
        }),
    ));
    assert!((({ (*zero.borrow_mut())() }) == 42));
    let one: Value<_> = Rc::new(RefCell::new(
        (|x: i32| {
            let x: Value<i32> = Rc::new(RefCell::new(x));
            return ((*x.borrow()) + 1);
        }),
    ));
    assert!((({ (*one.borrow_mut())(1,) }) == 2));
    let three: Value<_> = Rc::new(RefCell::new(
        (|x: i32, y: i32, z: i32| {
            let x: Value<i32> = Rc::new(RefCell::new(x));
            let y: Value<i32> = Rc::new(RefCell::new(y));
            let z: Value<i32> = Rc::new(RefCell::new(z));
            return ((((*x.borrow()) * 100) + ((*y.borrow()) * 10)) + (*z.borrow()));
        }),
    ));
    assert!((({ (*three.borrow_mut())(1, 2, 3,) }) == 123));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
