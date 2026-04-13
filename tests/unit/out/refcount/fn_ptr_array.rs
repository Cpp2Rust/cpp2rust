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
    let ops: Value<Box<[Option<fn(i32, i32) -> i32>]>> = Rc::new(RefCell::new(Box::new([
        Some(add_0 as _),
        Some(sub_1 as _),
        Some(mul_2 as _),
    ])));
    assert!(
        (({
            let _arg0: i32 = 2;
            let _arg1: i32 = 3;
            ((*ops.borrow())[(0) as usize]).unwrap()(_arg0, _arg1)
        }) == 5)
    );
    assert!(
        (({
            let _arg0: i32 = 7;
            let _arg1: i32 = 4;
            ((*ops.borrow())[(1) as usize]).unwrap()(_arg0, _arg1)
        }) == 3)
    );
    assert!(
        (({
            let _arg0: i32 = 6;
            let _arg1: i32 = 5;
            ((*ops.borrow())[(2) as usize]).unwrap()(_arg0, _arg1)
        }) == 30)
    );
    assert!(!(((*ops.borrow())[(0) as usize]).is_none()));
    assert!(((*ops.borrow())[(0) as usize] == Some(add_0 as _)));
    assert!(((*ops.borrow())[(0) as usize] != Some(sub_1 as _)));
    return 0;
}
