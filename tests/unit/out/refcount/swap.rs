extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn identity_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return (*x.borrow());
}
pub fn swap_by_ptr_1(a: Ptr<i32>, b: Ptr<i32>) {
    let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Ptr<i32>> = Rc::new(RefCell::new(b));
    let tmp: Value<i32> = Rc::new(RefCell::new(((*a.borrow()).read())));
    let __rhs = ((*b.borrow()).read());
    (*a.borrow()).write(__rhs);
    let __rhs = (*tmp.borrow());
    (*b.borrow()).write(__rhs);
}
pub fn swap_by_ref_2(a: Ptr<i32>, b: Ptr<i32>) {
    let tmp: Value<i32> = Rc::new(RefCell::new((a.read())));
    let __rhs = (b.read());
    a.write(__rhs);
    let __rhs = (*tmp.borrow());
    b.write(__rhs);
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let local: Value<i32> = Rc::new(RefCell::new(0));
    let a: Value<i32> = Rc::new(RefCell::new(1));
    let b: Value<i32> = Rc::new(RefCell::new(2));
    let c: Value<i32> = Rc::new(RefCell::new(
        ({
            let _x: i32 = (*local.borrow());
            identity_0(_x)
        }),
    ));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((a.as_pointer())));
    (*p.borrow_mut()) = (b.as_pointer());
    (*p.borrow_mut()) = (a.as_pointer());
    ({
        let _a: Ptr<i32> = (*p.borrow()).clone();
        let _b: Ptr<i32> = (b.as_pointer());
        swap_by_ptr_1(_a, _b)
    });
    ({
        let _a: Ptr<i32> = a.as_pointer();
        let _b: Ptr<i32> = c.as_pointer();
        swap_by_ref_2(_a, _b)
    });
    return (*c.borrow());
}
