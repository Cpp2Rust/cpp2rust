extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new({
        let a: Value<i32> = Rc::new(RefCell::new(1));
        let b: Value<i32> = Rc::new(RefCell::new(2));
        let __result = ((*a.borrow()) + (*b.borrow()));
        __result
    }));
    assert!(((*x.borrow()) == 3));
    let counter: Value<i32> = Rc::new(RefCell::new(0));
    let y: Value<i32> = Rc::new(RefCell::new({
        (*counter.borrow_mut()).postfix_inc();
        let __result = ((*counter.borrow()) * 10);
        __result
    }));
    assert!(((*y.borrow()) == 10));
    assert!(((*counter.borrow()) == 1));
    let z: Value<i32> = Rc::new(RefCell::new({
        let v: Value<i32> = Rc::new(RefCell::new(5));
        if ((*v.borrow()) > 0) {
            let __rhs = ((*v.borrow()) * 2);
            (*v.borrow_mut()) = __rhs;
        }
        let __result = (*v.borrow());
        __result
    }));
    assert!(((*z.borrow()) == 10));
    assert!(
        ({
            let inner: Value<i32> = Rc::new(RefCell::new({
                let a: Value<i32> = Rc::new(RefCell::new(100));
                let __result = (*a.borrow());
                __result
            }));
            let __result = (*inner.borrow());
            __result
        } == 100)
    );
    return 0;
}
