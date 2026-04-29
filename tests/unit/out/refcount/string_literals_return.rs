extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn get_greeting_0() -> Ptr<u8> {
    return Ptr::from_string_literal("hello");
}
pub fn get_empty_1() -> Ptr<u8> {
    return Ptr::from_string_literal("");
}
pub fn get_branch_2(x: i32) -> Ptr<u8> {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    if ((*x.borrow()) > 0) {
        return Ptr::from_string_literal("positive");
    }
    return Ptr::from_string_literal("non-positive");
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Ptr<u8>> = Rc::new(RefCell::new(({ get_greeting_0() })));
    assert!(((((*a.borrow()).offset((0) as isize).read()) as i32) == (('h' as u8) as i32)));
    assert!(((((*a.borrow()).offset((4) as isize).read()) as i32) == (('o' as u8) as i32)));
    assert!(((((*a.borrow()).offset((5) as isize).read()) as i32) == (('\0' as u8) as i32)));
    let b: Value<Ptr<u8>> = Rc::new(RefCell::new(({ get_empty_1() })));
    assert!(((((*b.borrow()).offset((0) as isize).read()) as i32) == (('\0' as u8) as i32)));
    let c: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ({
            let _x: i32 = 1;
            get_branch_2(_x)
        }),
    ));
    assert!(((((*c.borrow()).offset((0) as isize).read()) as i32) == (('p' as u8) as i32)));
    assert!(((((*c.borrow()).offset((7) as isize).read()) as i32) == (('e' as u8) as i32)));
    assert!(((((*c.borrow()).offset((8) as isize).read()) as i32) == (('\0' as u8) as i32)));
    let d: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ({
            let _x: i32 = -1_i32;
            get_branch_2(_x)
        }),
    ));
    assert!(((((*d.borrow()).offset((0) as isize).read()) as i32) == (('n' as u8) as i32)));
    assert!(((((*d.borrow()).offset((11) as isize).read()) as i32) == (('e' as u8) as i32)));
    assert!(((((*d.borrow()).offset((12) as isize).read()) as i32) == (('\0' as u8) as i32)));
    return 0;
}
