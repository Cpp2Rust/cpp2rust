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
    let mut x: i32 = 1;
    let mut y: i32 = {
        x = 2;
        (x + 1)
    };
    assert!((x == 2));
    assert!((y == 3));
    let mut z: i32 = {
        {
            1;
            2
        };
        3
    };
    assert!((z == 3));
    let counter: Value<i32> = Rc::new(RefCell::new(0));
    let mut w: i32 = {
        {
            (*counter.borrow_mut()).postfix_inc();
            (*counter.borrow_mut()).postfix_inc()
        };
        (*counter.borrow())
    };
    assert!(((*counter.borrow()) == 2));
    assert!((w == 2));
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    if {
        {
            a = 1;
            b = 2
        };
        ((a + b) > 0)
    } {
        assert!((a == 1));
        assert!((b == 2));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
