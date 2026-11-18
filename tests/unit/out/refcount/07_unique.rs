extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fn_0(mut u: Option<Value<i32>>) -> Option<Value<i32>> {
    u.as_pointer().write(10);
    return u.take();
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let mut f: Option<Value<i32>> = Some(Rc::new(RefCell::new(8)));
    f.as_pointer().write(9);
    let mut f_ptr1: Ptr<i32> = f.as_pointer();
    f_ptr1.write(10);
    let mut f_ptr2: Ptr<i32> = (f.as_pointer());
    f_ptr2.write(11);
    {
        f = Some(Rc::new(RefCell::new(9))).take();
    };
    {
        f = ({ fn_0(f.take()) }).take();
    };
    assert!(((f.as_pointer().read()) == 10));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
