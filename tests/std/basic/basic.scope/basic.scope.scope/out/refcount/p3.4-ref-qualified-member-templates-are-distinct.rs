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
pub struct HasRefQualifiedTemplateOverloads {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let object: Value<HasRefQualifiedTemplateOverloads> =
        Rc::new(RefCell::new(<HasRefQualifiedTemplateOverloads>::default()));
    assert!(
        (({ HasRefQualifiedTemplateOverloadsImpl::get_1_int(&object.as_pointer(), 10,) }) == 13)
    );
    assert!((lvalue_calls_0.with(|rc| *rc.borrow()) == 1));
    assert!((rvalue_calls_1.with(|rc| *rc.borrow()) == 0));
    assert!(
        (({
            HasRefQualifiedTemplateOverloadsImpl::get_2_int(
                &Rc::new(RefCell::new(<HasRefQualifiedTemplateOverloads>::default())).as_pointer(),
                10,
            )
        }) == 14)
    );
    assert!((lvalue_calls_0.with(|rc| *rc.borrow()) == 1));
    assert!((rvalue_calls_1.with(|rc| *rc.borrow()) == 1));
    assert!(
        (({ HasRefQualifiedTemplateOverloadsImpl::get_2_int(&object.as_pointer(), 10,) }) == 14)
    );
    assert!((lvalue_calls_0.with(|rc| *rc.borrow()) == 1));
    assert!((rvalue_calls_1.with(|rc| *rc.borrow()) == 2));
    return 0;
}
pub trait HasRefQualifiedTemplateOverloadsImpl {
    fn get_1_int(&self, value: i32) -> i32;
    fn get_2_int(&self, value: i32) -> i32;
}
impl HasRefQualifiedTemplateOverloadsImpl for Ptr<HasRefQualifiedTemplateOverloads> {
    fn get_1_int(&self, mut value: i32) -> i32 {
        (*lvalue_calls_0.with(Value::clone).borrow_mut()).prefix_inc();
        return (value + 3);
    }
    fn get_2_int(&self, mut value: i32) -> i32 {
        (*rvalue_calls_1.with(Value::clone).borrow_mut()).prefix_inc();
        return (value + 4);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = lvalue_calls_0.with(|_| ());
    let _ = rvalue_calls_1.with(|_| ());
}
