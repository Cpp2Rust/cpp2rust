extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn logf_impl_0(fmt: Ptr<u8>, ap: VaList) -> i32 {
    let fmt: Value<Ptr<u8>> = Rc::new(RefCell::new(fmt));
    let ap: Value<VaList> = Rc::new(RefCell::new(ap));
    return {
        let _lhs = ((*ap.borrow_mut()).arg::<i32>()).clone();
        _lhs + ((*ap.borrow_mut()).arg::<i32>()).clone()
    };
}
pub fn logf_1(fmt: Ptr<u8>, args: &[VaArg]) -> i32 {
    let fmt: Value<Ptr<u8>> = Rc::new(RefCell::new(fmt));
    let ap: Value<VaList> = Rc::new(RefCell::new(<VaList>::default()));
    (*ap.borrow_mut()) = VaList::new(args);
    let result: Value<i32> = Rc::new(RefCell::new(
        ({
            let _fmt: Ptr<u8> = (*fmt.borrow()).clone();
            let _ap: VaList = (*ap.borrow()).clone();
            logf_impl_0(_fmt, _ap)
        }),
    ));
    return (*result.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (({
            let _fmt: Ptr<u8> = Ptr::from_string_literal("hello %d %d");
            logf_1(_fmt, &[10.into(), 32.into()])
        }) == 42)
    );
    assert!(
        (({
            let _fmt: Ptr<u8> = Ptr::from_string_literal("x %d %d");
            logf_1(_fmt, &[1.into(), 2.into()])
        }) == 3)
    );
    return 0;
}
