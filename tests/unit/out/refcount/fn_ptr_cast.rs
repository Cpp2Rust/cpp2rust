extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn double_it_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ((*x.borrow()) * 2);
}
pub fn test_roundtrip_1() {
    let fn_: Value<Option<fn(i32) -> i32>> = Rc::new(RefCell::new(Some(double_it_0 as _)));
    assert!(
        (({
            let _arg0: i32 = 5;
            (*fn_.borrow()).unwrap()(_arg0)
        }) == 10)
    );
    let gfn: Value<Option<fn()>> = Rc::new(RefCell::new(
        ((*fn_.borrow()).to_strong().as_pointer() as Value<Option<fn()>>).clone(),
    ));
    assert!(!((*gfn.borrow()).is_none()));
    let fn2: Value<Option<fn(i32) -> i32>> = Rc::new(RefCell::new(
        ((*gfn.borrow()).to_strong().as_pointer() as Value<Option<fn(i32) -> i32>>).clone(),
    ));
    assert!(
        (({
            let _arg0: i32 = 5;
            (*fn2.borrow()).unwrap()(_arg0)
        }) == 10)
    );
    assert!({
        let _lhs = (*fn2.borrow()).clone();
        _lhs == (*fn_.borrow()).clone()
    });
}
pub fn test_double_cast_2() {
    let fn_: Value<Option<fn(i32) -> i32>> = Rc::new(RefCell::new(Some(double_it_0 as _)));
    let fn2: Value<Option<fn(i32) -> i32>> = Rc::new(RefCell::new(
        (((*fn_.borrow()).to_strong().as_pointer() as Value<Option<fn()>>)
            .to_strong()
            .as_pointer() as Value<Option<fn(i32) -> i32>>)
            .clone(),
    ));
    assert!(
        (({
            let _arg0: i32 = 5;
            (*fn2.borrow()).unwrap()(_arg0)
        }) == 10)
    );
    assert!({
        let _lhs = (*fn2.borrow()).clone();
        _lhs == (*fn_.borrow()).clone()
    });
}
#[derive(Default)]
pub struct Command {
    pub data: Value<AnyPtr>,
}
impl Clone for Command {
    fn clone(&self) -> Self {
        let mut this = Self {
            data: Rc::new(RefCell::new((*self.data.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for Command {}
pub fn test_void_ptr_to_fn_3() {
    let cmd: Value<Command> = Rc::new(RefCell::new(<Command>::default()));
    (*(*cmd.borrow()).data.borrow_mut()) =
        (Some(double_it_0 as _).to_strong().as_pointer() as AnyPtr);
    let fn_: Value<Option<fn(i32) -> i32>> = Rc::new(RefCell::new(
        ((*(*cmd.borrow()).data.borrow())
            .cast::<i32>()
            .expect("ub:wrong type"))
        .clone(),
    ));
    assert!(
        (({
            let _arg0: i32 = 5;
            (*fn_.borrow()).unwrap()(_arg0)
        }) == 10)
    );
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_roundtrip_1() });
    ({ test_double_cast_2() });
    ({ test_void_ptr_to_fn_3() });
    return 0;
}
