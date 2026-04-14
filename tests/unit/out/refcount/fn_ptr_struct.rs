extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub struct Handler {
    pub tag: Value<i32>,
    pub cb: Value<FnPtr<fn(i32) -> i32>>,
}
impl Clone for Handler {
    fn clone(&self) -> Self {
        let mut this = Self {
            tag: Rc::new(RefCell::new((*self.tag.borrow()))),
            cb: Rc::new(RefCell::new((*self.cb.borrow()).clone())),
        };
        this
    }
}
impl Default for Handler {
    fn default() -> Self {
        Handler {
            tag: <Value<i32>>::default(),
            cb: Rc::new(RefCell::new(FnPtr::null())),
        }
    }
}
impl ByteRepr for Handler {}
pub fn double_it_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ((*x.borrow()) * 2);
}
pub fn negate_1(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return -(*x.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let h1: Value<Handler> = Rc::new(RefCell::new(Handler {
        tag: Rc::new(RefCell::new(1)),
        cb: Rc::new(RefCell::new(fn_ptr!(double_it_0, fn(i32) -> i32))),
    }));
    let h2: Value<Handler> = Rc::new(RefCell::new(Handler {
        tag: Rc::new(RefCell::new(2)),
        cb: Rc::new(RefCell::new(fn_ptr!(negate_1, fn(i32) -> i32))),
    }));
    assert!(!((*(*h1.borrow()).cb.borrow()).is_null()));
    assert!(
        (({
            let _arg0: i32 = 5;
            (*(*h1.borrow()).cb.borrow()).call()(_arg0)
        }) == 10)
    );
    assert!(
        (({
            let _arg0: i32 = 7;
            (*(*h2.borrow()).cb.borrow()).call()(_arg0)
        }) == -7_i32)
    );
    (*(*h1.borrow()).cb.borrow_mut()) = fn_ptr!(negate_1, fn(i32) -> i32);
    assert!(
        (({
            let _arg0: i32 = 3;
            (*(*h1.borrow()).cb.borrow()).call()(_arg0)
        }) == -3_i32)
    );
    assert!({
        let _lhs = (*(*h1.borrow()).cb.borrow()).clone();
        _lhs == (*(*h2.borrow()).cb.borrow()).clone()
    });
    return 0;
}
