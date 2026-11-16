extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static ctorCount_0: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static dtorCount_1: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Destructor, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct ConvertsToBool {
    #[offset(0)]
    pub value: i32,
}
impl ConvertsToBool {
    pub fn new(mut value: i32) -> Self {
        let __this: ConvertsToBool = Self { value: value };
        (*ctorCount_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut remaining: i32 = 3;
    let mut continued: i32 = 0;
    let mut completed: i32 = 0;
    'loop_: while ({ ConvertsToBoolImpl::to_bool(&condition.as_pointer()) }) {
        remaining.prefix_dec();
        if ({ (*condition.borrow()).value } == 3) {
            continued.prefix_inc();
            continue 'loop_;
        }
        assert!((ctorCount_0.with(|rc| *rc.borrow()) == 2));
        assert!((dtorCount_1.with(|rc| *rc.borrow()) == 1));
        completed.prefix_inc();
        break;
    }
    assert!((continued == 1));
    assert!((completed == 1));
    assert!((remaining == 1));
    assert!((ctorCount_0.with(|rc| *rc.borrow()) == 2));
    assert!((dtorCount_1.with(|rc| *rc.borrow()) == 2));
    return 0;
}
pub trait ConvertsToBoolImpl {
    fn destructor(&self);
    fn to_bool(&self) -> bool;
}
impl ConvertsToBoolImpl for Ptr<ConvertsToBool> {
    fn destructor(&self) {
        (*dtorCount_1.with(Value::clone).borrow_mut()).prefix_inc();
    }
    fn to_bool(&self) -> bool {
        return ((*self).with(|__s| __s.value) != 0);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = ctorCount_0.with(|_| ());
    let _ = dtorCount_1.with(|_| ());
}
