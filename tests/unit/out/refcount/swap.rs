extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn identity_0(mut x: i32) -> i32 {
    return x;
}
pub fn swap_by_ptr_1(mut a: Ptr<i32>, mut b: Ptr<i32>) {
    let mut tmp: i32 = (a.read());
    a.write({ (b.read()) });
    b.write({ tmp });
}
pub fn swap_by_ref_2(a: Ptr<i32>, b: Ptr<i32>) {
    let mut tmp: i32 = (a.read());
    a.write({ (b.read()) });
    b.write({ tmp });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut local: i32 = 0;
    let a: Value<i32> = Rc::new(RefCell::new(1));
    let b: Value<i32> = Rc::new(RefCell::new(2));
    let c: Value<i32> = Rc::new(RefCell::new(({ identity_0(local) })));
    let mut p: Ptr<i32> = (a.as_pointer());
    p = (b.as_pointer());
    p = (a.as_pointer());
    ({ swap_by_ptr_1((p).clone(), (b.as_pointer())) });
    ({ swap_by_ref_2(a.as_pointer(), c.as_pointer()) });
    assert!(((*c.borrow()) == 2));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
