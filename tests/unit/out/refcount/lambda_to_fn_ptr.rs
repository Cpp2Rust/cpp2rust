extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn apply_0(x: i32, fn_: FnPtr<fn(i32) -> i32>) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let fn_: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(fn_));
    return ({ (*fn_.borrow()).call((*x.borrow())) });
}
pub fn call_nttp_1(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ({
        (FnPtr::<fn(i32) -> i32>::new(int_operator___int__const::__invoke))((*x.borrow()))
    });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let fresh: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(FnPtr::new(
        (|x: i32| {
            let x: Value<i32> = Rc::new(RefCell::new(x));
            return -(*x.borrow());
        }),
    )));
    assert!((({ (*fresh.borrow()).call(5,) }) == -5_i32));
    let twice: Value<_> = Rc::new(RefCell::new(
        (|x: i32| {
            let x: Value<i32> = Rc::new(RefCell::new(x));
            return ((*x.borrow()) * 2);
        }),
    ));
    let named: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(({ (*twice.borrow())() })));
    assert!((({ (*named.borrow()).call(5,) }) == 10));
    assert!((({ apply_0(5, ({ (*twice.borrow())() }),) }) == 10));
    (*named.borrow_mut()) = (*fresh.borrow()).clone();
    assert!((({ (*named.borrow()).call(3,) }) == -3_i32));
    let p: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(
        ({
            (|x: i32| {
                let x: Value<i32> = Rc::new(RefCell::new(x));
                return ((*x.borrow()) * 3);
            })()
        }),
    ));
    assert!((({ (*p.borrow()).call(2,) }) == 6));
    assert!((({ call_nttp_1(5,) }) == 4));
    let sum: Value<_> = Rc::new(RefCell::new(
        (|n: i32| {
            let n: Value<i32> = Rc::new(RefCell::new(n));
            let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
            (*ap.borrow_mut()) = VaList::new(__args);
            let s: Value<i32> = Rc::new(RefCell::new(0));
            let i: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ((*i.borrow()) < (*n.borrow())) {
                (*s.borrow_mut()) += (*ap.borrow_mut()).arg::<i32>();
                (*i.borrow_mut()).postfix_inc();
            }
            return (*s.borrow());
        }),
    ));
    assert!((({ (*sum.borrow_mut())(2, &[(5).into(), (5).into(),]) }) == 10));
    assert!((({ (*sum.borrow_mut())(3, &[(1).into(), (2).into(), (3).into(),]) }) == 6));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
