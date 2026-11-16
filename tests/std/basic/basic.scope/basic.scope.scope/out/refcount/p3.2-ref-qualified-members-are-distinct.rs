extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static lvalue_calls_0: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static rvalue_calls_1: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct HasRefQualifiedOverloads {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let object: Value<HasRefQualifiedOverloads> =
        Rc::new(RefCell::new(<HasRefQualifiedOverloads>::default()));
    assert!((({ HasRefQualifiedOverloadsImpl::get_1(&object.as_pointer(),) }) == 3));
    assert!((lvalue_calls_0.with(|rc| *rc.borrow()) == 1));
    assert!((rvalue_calls_1.with(|rc| *rc.borrow()) == 0));
    assert!(
        (({
            HasRefQualifiedOverloadsImpl::get_2(
                &Rc::new(RefCell::new(<HasRefQualifiedOverloads>::default())).as_pointer(),
            )
        }) == 4)
    );
    assert!((lvalue_calls_0.with(|rc| *rc.borrow()) == 1));
    assert!((rvalue_calls_1.with(|rc| *rc.borrow()) == 1));
    assert!((({ HasRefQualifiedOverloadsImpl::get_2(&object.as_pointer(),) }) == 4));
    assert!((lvalue_calls_0.with(|rc| *rc.borrow()) == 1));
    assert!((rvalue_calls_1.with(|rc| *rc.borrow()) == 2));
    return 0;
}
pub trait HasRefQualifiedOverloadsImpl {
    fn get_1(&self) -> i32;
    fn get_2(&self) -> i32;
}
impl HasRefQualifiedOverloadsImpl for Ptr<HasRefQualifiedOverloads> {
    fn get_1(&self) -> i32 {
        (*lvalue_calls_0.with(Value::clone).borrow_mut()).prefix_inc();
        return 3;
    }
    fn get_2(&self) -> i32 {
        (*rvalue_calls_1.with(Value::clone).borrow_mut()).prefix_inc();
        return 4;
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = lvalue_calls_0.with(|_| ());
    let _ = rvalue_calls_1.with(|_| ());
}
