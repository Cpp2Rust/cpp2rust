extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn conditional_log_0(verbose: i32, fmt: Ptr<core::ffi::c_char>, __args: &[VaArg]) -> i32 {
    let verbose: Value<i32> = Rc::new(RefCell::new(verbose));
    let fmt: Value<Ptr<core::ffi::c_char>> = Rc::new(RefCell::new(fmt));
    if ((*verbose.borrow()) != 0) {
        let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
        (*ap.borrow_mut()) = VaList::new(__args);
        let result: Value<i32> = Rc::new(RefCell::new(((*ap.borrow_mut()).arg::<i32>()).clone()));
        return (*result.borrow());
    }
    return -1_i32;
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (((({ conditional_log_0(1, Ptr::from_string_literal(b"%d"), &[(42).into(),]) }) == 42)
            as i32)
            != 0)
    );
    assert!(
        (((({ conditional_log_0(0, Ptr::from_string_literal(b"%d"), &[(99).into(),]) }) == -1_i32)
            as i32)
            != 0)
    );
    return 0;
}
