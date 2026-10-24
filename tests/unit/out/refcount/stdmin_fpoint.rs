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
    let a0: Value<f64> = Rc::new(RefCell::new(3.14_f64));
    let a1: Value<f64> = Rc::new(RefCell::new(2.71_f64));
    {
        let __a = a0.as_pointer();
        let __b = a1.as_pointer();
        if CppLess::less(&__b, &__a) { __b } else { __a }
    };
    return 0;
}
pub fn __cpp2rust_init_globals() {}
