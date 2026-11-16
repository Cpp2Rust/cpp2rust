extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static then_runs_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static else_runs_1: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static dtorCount_2: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct ConvertsToBool {
    #[offset(0)]
    pub value: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        let condition: Value<ConvertsToBool> = Rc::new(RefCell::new(ConvertsToBool { value: 0 }));
        let _dtor_condition =
            std::mem::DropGuard::new(condition.clone(), |__p| __p.as_pointer().destructor());
        if ({ ConvertsToBoolImpl::to_bool(&condition.as_pointer()) }) {
            (*then_runs_0.with(Value::clone).borrow_mut()).prefix_inc();
        } else {
            (*else_runs_1.with(Value::clone).borrow_mut()).prefix_inc();
            assert!(({ ({ (*condition.borrow()).value } == 0) }));
            assert!((dtorCount_2.with(|rc| *rc.borrow()) == 0));
        }
    }
    assert!((then_runs_0.with(|rc| *rc.borrow()) == 0));
    assert!((else_runs_1.with(|rc| *rc.borrow()) == 1));
    assert!((dtorCount_2.with(|rc| *rc.borrow()) == 1));
    {
        let condition: Value<ConvertsToBool> = Rc::new(RefCell::new(ConvertsToBool { value: 6 }));
        let _dtor_condition =
            std::mem::DropGuard::new(condition.clone(), |__p| __p.as_pointer().destructor());
        if ({ ConvertsToBoolImpl::to_bool(&condition.as_pointer()) }) {
            (*then_runs_0.with(Value::clone).borrow_mut()).prefix_inc();
            assert!(({ ({ (*condition.borrow()).value } == 6) }));
            assert!((dtorCount_2.with(|rc| *rc.borrow()) == 1));
        } else {
            (*else_runs_1.with(Value::clone).borrow_mut()).prefix_inc();
        }
    }
    assert!((then_runs_0.with(|rc| *rc.borrow()) == 1));
    assert!((else_runs_1.with(|rc| *rc.borrow()) == 1));
    assert!((dtorCount_2.with(|rc| *rc.borrow()) == 2));
    return 0;
}
pub trait ConvertsToBoolImpl {
    fn to_bool(&self) -> bool;
    fn destructor(&self);
}
impl ConvertsToBoolImpl for Ptr<ConvertsToBool> {
    fn to_bool(&self) -> bool {
        return ((*self).with(|__s| __s.value) != 0);
    }
    fn destructor(&self) {
        (*dtorCount_2.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = then_runs_0.with(|_| ());
    let _ = else_runs_1.with(|_| ());
    let _ = dtorCount_2.with(|_| ());
}
