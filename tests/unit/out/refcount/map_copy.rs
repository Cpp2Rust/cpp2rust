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
    let a: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    Map::subscript_move(
        a.as_pointer(),
        Rc::new(RefCell::new(1)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(10);
    let b: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new((*a.borrow()).clone()));
    Map::subscript_move(
        b.as_pointer(),
        Rc::new(RefCell::new(1)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(20);
    assert!(
        ((Map::subscript_move(
            a.as_pointer(),
            Rc::new(RefCell::new(1)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 10)
    );
    let x: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    Map::subscript_move(
        x.as_pointer(),
        Rc::new(RefCell::new(2)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(3);
    let y: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new((*x.borrow()).clone()));
    Map::subscript_move(
        y.as_pointer(),
        Rc::new(RefCell::new(2)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(4);
    assert!(
        ((Map::subscript_move(
            x.as_pointer(),
            Rc::new(RefCell::new(2)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .read())
            == 3)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
