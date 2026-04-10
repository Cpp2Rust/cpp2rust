extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn conditional_log_0(verbose: i32, fmt: Ptr<u8>, args: &[VaArg]) -> i32 {
    let verbose: Value<i32> = Rc::new(RefCell::new(verbose));
    let fmt: Value<Ptr<u8>> = Rc::new(RefCell::new(fmt));
    if ((*verbose.borrow()) != 0) {
        let ap: Value<VaList> = Rc::new(RefCell::new(<VaList>::default()));
        (*ap.borrow_mut()) = VaList::new(args);
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
        (({
            let _verbose: i32 = 1;
            let _fmt: Ptr<u8> = Ptr::from_string_literal("%d");
            conditional_log_0(_verbose, _fmt, &[42.into()])
        }) == 42)
    );
    assert!(
        (({
            let _verbose: i32 = 0;
            let _fmt: Ptr<u8> = Ptr::from_string_literal("%d");
            conditional_log_0(_verbose, _fmt, &[99.into()])
        }) == -1_i32)
    );
    return 0;
}
