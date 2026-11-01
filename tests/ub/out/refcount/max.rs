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
    let a: Ptr<i32> = ({
        let __tmp_0: Value<i32> = Rc::new(RefCell::new(2));
        let __tmp_1: Value<i32> = Rc::new(RefCell::new(1));
        if CppLess::less(&__tmp_0.as_pointer(), &__tmp_1.as_pointer()) {
            __tmp_0.as_pointer()
        } else {
            __tmp_1.as_pointer()
        }
    });
    let b: Ptr<i32> = ({
        let __tmp_2: Value<i32> = Rc::new(RefCell::new(1));
        let __tmp_3: Value<i32> = Rc::new(RefCell::new(2));
        if CppLess::less(&__tmp_2.as_pointer(), &__tmp_3.as_pointer()) {
            __tmp_3.as_pointer()
        } else {
            __tmp_2.as_pointer()
        }
    });
    return (({ (a.read()) } == { (b.read()) }) as i32);
}
pub fn __cpp2rust_init_globals() {}
