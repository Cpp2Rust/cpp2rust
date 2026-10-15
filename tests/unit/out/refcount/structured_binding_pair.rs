extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn make_std_pair_0(mut x: i32) -> (Value<i32>, Value<bool>) {
    return (
        Rc::new(RefCell::new(x.try_into().expect("failed conversion"))),
        Rc::new(RefCell::new((x > 0).try_into().expect("failed conversion"))),
    );
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let __decomp_1: Value<(Value<i32>, Value<bool>)> =
        Rc::new(RefCell::new(({ make_std_pair_0(5) })));
    let value: Ptr<i32> = (*__decomp_1.borrow()).0.as_pointer();
    let positive: Ptr<bool> = (*__decomp_1.borrow()).1.as_pointer();
    assert!(((value.read()) == 5));
    assert!(((positive.read()) as bool));
    let p: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(1.try_into().expect("failed conversion"))),
        Rc::new(RefCell::new(2.try_into().expect("failed conversion"))),
    )));
    let __decomp_2: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new((*p.borrow()).0.borrow().clone())),
        Rc::new(RefCell::new((*p.borrow()).1.borrow().clone())),
    )));
    let a: Ptr<i32> = (*__decomp_2.borrow()).0.as_pointer();
    let b: Ptr<i32> = (*__decomp_2.borrow()).1.as_pointer();
    a.write(10);
    assert!(((a.read()) == 10));
    assert!(((b.read()) == 2));
    assert!(((*(*p.borrow()).0.borrow()) == 1));
    let __decomp_3: Ptr<(Value<i32>, Value<i32>)> = p.as_pointer();
    let x: Ptr<i32> = (*__decomp_3.upgrade().deref()).0.as_pointer();
    let y: Ptr<i32> = (*__decomp_3.upgrade().deref()).1.as_pointer();
    x.write(3);
    {
        y.with_mut(|__v| *__v = *__v + 4)
    };
    assert!(((*(*p.borrow()).0.borrow()) == 3));
    assert!(((*(*p.borrow()).1.borrow()) == 6));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
