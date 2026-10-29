extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn first_0(u: Ptr<Option<Value<Vec<u8>>>>) -> Ptr<u8> {
    return (*u.upgrade().deref()).as_pointer().decay();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Option<Value<Vec<u8>>>> = Rc::new(RefCell::new(
        Ptr::alloc((0..(3_usize) as usize).map(|_| 0_u8).collect::<Vec<_>>()).to_owned_opt(),
    ));
    let mut p: Ptr<u8> = (*v.borrow()).as_pointer().decay();
    elem!(p, 2).write(1_u8);
    elem!(({ first_0(v.as_pointer(),) }), 0).write(5_u8);
    assert!((((elem!(p, 0).read()) as i32) == 5));
    assert!((((elem!(({ first_0(v.as_pointer(),) }), 2).read()) as i32) == 1));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
