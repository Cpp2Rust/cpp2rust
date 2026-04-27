extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum Kind {
    #[default]
    KIND_NONE = 0,
    KIND_DONE = 1,
}
#[derive(Default)]
pub union Event_anon_12_3 {
    pub obj: Value<AnyPtr>,
    pub code: Value<i32>,
}
impl Default for Event_anon_12_3 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl ByteRepr for Event_anon_12_3 {}
#[derive(Default)]
pub struct Event {
    pub kind: Value<Kind>,
    pub handle: Value<AnyPtr>,
    pub payload: Value<Event_anon_12_3>,
}
impl ByteRepr for Event {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let dummy: Value<i32> = Rc::new(RefCell::new(0));
    let m1: Value<Event> = <Value<Event>>::default();
    (*(*m1.borrow()).kind.borrow_mut()) = (Kind::KIND_DONE as Kind);
    (*(*m1.borrow()).handle.borrow_mut()) = ((dummy.as_pointer()) as Ptr<i32>).to_any();
    (*(*(*m1.borrow()).payload.borrow()).code.borrow_mut()) = 42;
    assert!((((*(*m1.borrow()).kind.borrow()) as u32) == (Kind::KIND_DONE as u32)));
    assert!(((*(*(*m1.borrow()).payload.borrow()).code.borrow()) == 42));
    let m2: Value<Event> = <Value<Event>>::default();
    (*(*m2.borrow()).kind.borrow_mut()) = (Kind::KIND_NONE as Kind);
    (*(*m2.borrow()).handle.borrow_mut()) = ((dummy.as_pointer()) as Ptr<i32>).to_any();
    (*(*(*m2.borrow()).payload.borrow()).obj.borrow_mut()) =
        ((dummy.as_pointer()) as Ptr<i32>).to_any();
    assert!({
        let _lhs = (*(*(*m2.borrow()).payload.borrow()).obj.borrow()).clone();
        _lhs == ((dummy.as_pointer()) as Ptr<i32>).to_any()
    });
    return 0;
}
