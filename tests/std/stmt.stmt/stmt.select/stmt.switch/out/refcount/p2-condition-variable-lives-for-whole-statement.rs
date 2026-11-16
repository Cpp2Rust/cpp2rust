extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static matching_case_runs_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static other_case_runs_1: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static dtorCount_2: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Destructor, Default)]
#[cpp_into(i32, to_i32)]
#[byte_size(4)]
pub struct ConvertsToInt {
    #[offset(0)]
    pub value: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        let selector: Value<ConvertsToInt> = Rc::new(RefCell::new(ConvertsToInt { value: 2 }));
        let _dtor_selector =
            std::mem::DropGuard::new(selector.clone(), |__p| __p.as_pointer().destructor());
        'switch: {
            match { ({ ConvertsToIntImpl::to_i32(&selector.as_pointer()) }) } {
                __v if __v == 2 => {
                    (*matching_case_runs_0.with(Value::clone).borrow_mut()).prefix_inc();
                    assert!(({ ({ (*selector.borrow()).value } == 2) }));
                    assert!((dtorCount_2.with(|rc| *rc.borrow()) == 0));
                    break 'switch;
                }
                _ => {
                    (*other_case_runs_1.with(Value::clone).borrow_mut()).prefix_inc();
                    break 'switch;
                }
            }
        }
    };
    assert!((matching_case_runs_0.with(|rc| *rc.borrow()) == 1));
    assert!((other_case_runs_1.with(|rc| *rc.borrow()) == 0));
    assert!((dtorCount_2.with(|rc| *rc.borrow()) == 1));
    return 0;
}
pub trait ConvertsToIntImpl {
    fn to_i32(&self) -> i32;
    fn destructor(&self);
}
impl ConvertsToIntImpl for Ptr<ConvertsToInt> {
    fn to_i32(&self) -> i32 {
        return (*self).with(|__s| __s.value);
    }
    fn destructor(&self) {
        (*dtorCount_2.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = matching_case_runs_0.with(|_| ());
    let _ = other_case_runs_1.with(|_| ());
    let _ = dtorCount_2.with(|_| ());
}
