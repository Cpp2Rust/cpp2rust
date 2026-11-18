extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct p1_as_const_Counter {
    #[offset(0)]
    pub value: i32,
}
pub fn test_0() {
    let number: Value<i32> = Rc::new(RefCell::new(3));
    let ref_: Ptr<i32> = number.as_pointer();
    assert!(({ (ref_).clone() } == { (number.as_pointer()) }));
    (*number.borrow_mut()) = 4;
    assert!(((ref_.read()) == 4));
    assert!(({ ((*number.borrow()) == 4) }));
    let mut pointer: Ptr<i32> = (number.as_pointer());
    assert!(({ (pointer).clone() } == { (number.as_pointer()) }));
    let counter: Value<p1_as_const_Counter> =
        Rc::new(RefCell::new(p1_as_const_Counter { value: 5 }));
    assert!((({ p1_as_const_CounterImpl::get_2(&counter.as_pointer(),) }) == 5));
    assert!((({ p1_as_const_CounterImpl::get_1(&counter.as_pointer(),) }) == 6));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_0() });
    return 0;
}
pub trait p1_as_const_CounterImpl {
    fn get_1(&self) -> i32;
    fn get_2(&self) -> i32;
}
impl p1_as_const_CounterImpl for Ptr<p1_as_const_Counter> {
    fn get_1(&self) -> i32 {
        return ((*self).with(|__s| __s.value) + 1);
    }
    fn get_2(&self) -> i32 {
        return (*self).with(|__s| __s.value);
    }
}
pub fn __cpp2rust_init_globals() {}
