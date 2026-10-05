extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn logf_impl_0(mut fmt: Ptr<i8>, ap: VaList) -> i32 {
    let ap: Value<VaList> = Rc::new(RefCell::new(ap));
    &(fmt);
    return ({ (*ap.borrow_mut()).arg::<i32>() } + { (*ap.borrow_mut()).arg::<i32>() });
}
pub fn logf_1(fmt: Ptr<i8>, __args: &[VaArg]) -> i32 {
    let fmt: Value<Ptr<i8>> = Rc::new(RefCell::new(fmt));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    (*ap.borrow_mut()) = VaList::new(__args);
    let mut result: i32 = ({ logf_impl_0((*fmt.borrow()).clone(), (*ap.borrow()).clone()) });
    return result;
}
pub fn lenf_2(fmt: Ptr<i8>, __args: &[VaArg]) -> i32 {
    let fmt: Value<Ptr<i8>> = Rc::new(RefCell::new(fmt));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    (*ap.borrow_mut()) = VaList::new(__args);
    let mut s: Ptr<i8> = (*ap.borrow_mut()).arg::<Ptr<i8>>();
    let mut result: i32 = (s.to_c_string_iterator().count() as i32);
    return result;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut dummy: Ptr<i8> = Ptr::<i8>::from_string_literal(b"dummy");
    assert!(
        (((({
            logf_1(
                Ptr::<i8>::from_string_literal(b"hello %d %d"),
                &[(10).into(), (dummy.to_c_string_iterator().count()).into()],
            )
        }) == 15) as i32)
            != 0)
    );
    assert!(
        (((({
            logf_1(
                Ptr::<i8>::from_string_literal(b"x %d %d"),
                &[(1).into(), (2).into()],
            )
        }) == 3) as i32)
            != 0)
    );
    assert!(
        (((({
            lenf_2(
                Ptr::<i8>::from_string_literal(b"%s"),
                &[((dummy).clone()).into()],
            )
        }) == 5) as i32)
            != 0)
    );
    assert!(
        (((({
            lenf_2(
                Ptr::<i8>::from_string_literal(b"%s"),
                &[(if (((elem!(dummy, 0).read()) as i32) != 0) {
                    (dummy).clone()
                } else {
                    Ptr::<i8>::from_string_literal(b"")
                })
                .into()],
            )
        }) == 5) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
