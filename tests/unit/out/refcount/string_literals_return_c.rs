extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn get_greeting_0() -> Ptr<u8> {
    return Ptr::<u8>::from_string_literal(b"hello");
}
pub fn get_empty_1() -> Ptr<u8> {
    return Ptr::<u8>::from_string_literal(b"");
}
pub fn get_branch_2(x: i32) -> Ptr<u8> {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    if ((((*x.borrow()) > 0) as i32) != 0) {
        return Ptr::<u8>::from_string_literal(b"positive");
    }
    return Ptr::<u8>::from_string_literal(b"non-positive");
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Ptr<u8>> = Rc::new(RefCell::new(({ get_greeting_0() })));
    assert!((((((elem!((*a.borrow()), 0).read()) as i32) == ('h' as i32)) as i32) != 0));
    assert!((((((elem!((*a.borrow()), 4).read()) as i32) == ('o' as i32)) as i32) != 0));
    assert!((((((elem!((*a.borrow()), 5).read()) as i32) == ('\0' as i32)) as i32) != 0));
    let b: Value<Ptr<u8>> = Rc::new(RefCell::new(({ get_empty_1() })));
    assert!((((((elem!((*b.borrow()), 0).read()) as i32) == ('\0' as i32)) as i32) != 0));
    let c: Value<Ptr<u8>> = Rc::new(RefCell::new(({ get_branch_2(1) })));
    assert!((((((elem!((*c.borrow()), 0).read()) as i32) == ('p' as i32)) as i32) != 0));
    assert!((((((elem!((*c.borrow()), 7).read()) as i32) == ('e' as i32)) as i32) != 0));
    assert!((((((elem!((*c.borrow()), 8).read()) as i32) == ('\0' as i32)) as i32) != 0));
    let d: Value<Ptr<u8>> = Rc::new(RefCell::new(({ get_branch_2(-1_i32) })));
    assert!((((((elem!((*d.borrow()), 0).read()) as i32) == ('n' as i32)) as i32) != 0));
    assert!((((((elem!((*d.borrow()), 11).read()) as i32) == ('e' as i32)) as i32) != 0));
    assert!((((((elem!((*d.borrow()), 12).read()) as i32) == ('\0' as i32)) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
