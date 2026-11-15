extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static x1_0: Value<Ptr<X>> = Rc::new(RefCell::new(Ptr::<X>::null()));
);
thread_local!(
    pub static x2_1: Value<Ptr<X>> = Rc::new(RefCell::new(Ptr::<X>::null()));
);
pub fn identity_2(mut p: Ptr<X>) -> Ptr<X> {
    return (p).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(({ (*x1_0.with(Value::clone).borrow()).is_null() }));
    assert!(
        ({
            ({ ({ identity_2((*x2_1.with(Value::clone).borrow()).clone()) }) } == {
                (*x1_0.with(Value::clone).borrow()).clone()
            })
        })
    );
    let mut pp: Ptr<Ptr<X>> = (x1_0.with(|v| v.as_pointer()));
    assert!((pp.read()).is_null());
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct X;
pub fn __cpp2rust_init_globals() {
    let _ = x1_0.with(|_| ());
    let _ = x2_1.with(|_| ());
}
