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
    let a: Value<i32> = Rc::new(RefCell::new(1));
    let b: Value<i32> = Rc::new(RefCell::new(2));
    let c: Value<i32> = Rc::new(RefCell::new(3));
    let by_value: Value<_> = Rc::new(RefCell::new(
        (|x: i32| {
            let x: Value<i32> = Rc::new(RefCell::new(x));
            return ((((*a.borrow()) + (*b.borrow())) + (*c.borrow())) + (*x.borrow()));
        }),
    ));
    assert!((({ (*by_value.borrow_mut())(10,) }) == 16));
    (*a.borrow_mut()) = 100;
    assert!((({ (*by_value.borrow_mut())(10,) }) == 16));
    let by_ref: Value<_> = Rc::new(RefCell::new(
        (|x: i32| {
            let x: Value<i32> = Rc::new(RefCell::new(x));
            return ((((*a.borrow()) + (*b.borrow())) + (*c.borrow())) + (*x.borrow()));
        }),
    ));
    assert!((({ (*by_ref.borrow_mut())(10,) }) == 115));
    (*b.borrow_mut()) = 200;
    assert!((({ (*by_ref.borrow_mut())(10,) }) == 313));
    let mixed: Value<_> = Rc::new(RefCell::new(
        (|x: i32| {
            let x: Value<i32> = Rc::new(RefCell::new(x));
            (*c.borrow_mut()) += (*x.borrow());
            return (((*a.borrow()) + (*b.borrow())) + (*c.borrow()));
        }),
    ));
    assert!((({ (*mixed.borrow_mut())(1,) }) == ((100 + 200) + 4)));
    assert!(((*c.borrow()) == 4));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
