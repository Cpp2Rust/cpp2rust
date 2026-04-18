extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn add_0(a: i32, b: i32) -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    let b: Value<i32> = Rc::new(RefCell::new(b));
    return ((*a.borrow()) + (*b.borrow()));
}
pub fn sub_1(a: i32, b: i32) -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    let b: Value<i32> = Rc::new(RefCell::new(b));
    return ((*a.borrow()) - (*b.borrow()));
}
pub fn mul_2(a: i32, b: i32) -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    let b: Value<i32> = Rc::new(RefCell::new(b));
    return ((*a.borrow()) * (*b.borrow()));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let ops: Value<Box<[FnPtr<fn(i32, i32) -> i32>]>> = Rc::new(RefCell::new(Box::new([
        FnPtr::<fn(i32, i32) -> i32>::new(add_0),
        FnPtr::<fn(i32, i32) -> i32>::new(sub_1),
        FnPtr::<fn(i32, i32) -> i32>::new(mul_2),
    ])));
    assert!(
        (({
            let _arg0: i32 = 2;
            let _arg1: i32 = 3;
            (*(*ops.borrow())[(0) as usize])(_arg0, _arg1)
        }) == 5)
    );
    assert!(
        (({
            let _arg0: i32 = 7;
            let _arg1: i32 = 4;
            (*(*ops.borrow())[(1) as usize])(_arg0, _arg1)
        }) == 3)
    );
    assert!(
        (({
            let _arg0: i32 = 6;
            let _arg1: i32 = 5;
            (*(*ops.borrow())[(2) as usize])(_arg0, _arg1)
        }) == 30)
    );
    assert!(!(((*ops.borrow())[(0) as usize]).is_null()));
    assert!(((*ops.borrow())[(0) as usize] == FnPtr::<fn(i32, i32) -> i32>::new(add_0)));
    assert!(((*ops.borrow())[(0) as usize] != FnPtr::<fn(i32, i32) -> i32>::new(sub_1)));
    return 0;
}
