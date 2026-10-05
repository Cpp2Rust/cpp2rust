extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn conditional_log_0(mut verbose: i32, fmt: Ptr<i8>, __args: &[VaArg]) -> i32 {
    let fmt: Value<Ptr<i8>> = Rc::new(RefCell::new(fmt));
    if (verbose != 0) {
        let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
        (*ap.borrow_mut()) = VaList::new(__args);
        let mut result: i32 = (*ap.borrow_mut()).arg::<i32>();
        return result;
    }
    return -1_i32;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (((({ conditional_log_0(1, Ptr::<i8>::from_string_literal(b"%d"), &[(42).into(),]) }) == 42)
            as i32)
            != 0)
    );
    assert!(
        (((({ conditional_log_0(0, Ptr::<i8>::from_string_literal(b"%d"), &[(99).into(),]) })
            == -1_i32) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
