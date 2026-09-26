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
    let i1: Value<i32> = Rc::new(RefCell::new(42));
    let ptr1: Value<Ptr<i32>> = Rc::new(RefCell::new((i1.as_pointer())));
    let ptr2: Value<Ptr<u8>> = Rc::new(RefCell::new((i1.as_pointer()).reinterpret_cast::<u8>()));
    let vptr1: Value<AnyPtr> = Rc::new(RefCell::new(
        ((*ptr1.borrow()).clone() as Ptr<i32>).to_any(),
    ));
    let vptr2: Value<AnyPtr> =
        Rc::new(RefCell::new(((*ptr2.borrow()).clone() as Ptr<u8>).to_any()));
    assert!({
        let _lhs = (*vptr1.borrow()).clone();
        _lhs == (*vptr2.borrow()).clone()
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
