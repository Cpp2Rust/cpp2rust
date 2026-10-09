extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(vec![1, 2]));
    let f: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let v: Value<Vec<i32>> = Rc::new(RefCell::new((*v.borrow()).clone()));
        },
        || -> i32 {
            return ((*v.borrow()).len() as i32);
        },
    )));
    let g: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new((*f.borrow()).copy_from()));
    let h: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new((*f.borrow()).move_from()));
    assert!((({ (*g.borrow()).call() }) == 2));
    assert!((({ (*h.borrow()).call() }) == 2));
    assert!((({ (*f.borrow()).call() }) == 0));
    let p: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let u: Value<Option<Value<i32>>> =
                Rc::new(RefCell::new(Some(Rc::new(RefCell::new(5)))));
        },
        || -> i32 {
            return if !((*u.borrow()).as_pointer()).is_null() {
                (*(*u.borrow()).as_ref().unwrap().borrow())
            } else {
                0
            };
        },
    )));
    let q: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new((*p.borrow()).move_from()));
    assert!((({ (*q.borrow()).call() }) == 5));
    assert!((({ (*p.borrow()).call() }) == 0));
    let n: Value<i32> = Rc::new(RefCell::new(0));
    let inner: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let n: Value<i32> = Rc::new(RefCell::new((*n.borrow())));
        },
        || -> i32 {
            return (*n.borrow_mut()).prefix_inc();
        },
    )));
    let outer: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let inner: Value<FnPtr<fn() -> i32>> =
                Rc::new(RefCell::new((*inner.borrow()).copy_from()));
        },
        || -> i32 {
            return ({ (*inner.borrow()).call() }).clone();
        },
    )));
    assert!((({ (*outer.borrow()).call() }) == 1));
    let outer2: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new((*outer.borrow()).copy_from()));
    assert!((({ (*outer2.borrow()).call() }) == 2));
    assert!((({ (*outer.borrow()).call() }) == 2));
    let outer3: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new((*outer.borrow()).move_from()));
    assert!((({ (*outer3.borrow()).call() }) == 3));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
