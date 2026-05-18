extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn square_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ((*x.borrow()) * (*x.borrow()));
}
pub fn negate_1(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return -(*x.borrow());
}
pub fn add_2(a: i32, b: i32) -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    let b: Value<i32> = Rc::new(RefCell::new(b));
    return ((*a.borrow()) + (*b.borrow()));
}
pub fn apply_unary_3(x: i32, __args: &[VaArg]) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    (*ap.borrow_mut()) = VaList::new(__args);
    let fn_: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(
        ((*ap.borrow_mut()).arg::<FnPtr<fn(i32) -> i32>>()).clone(),
    ));
    let result: Value<i32> = Rc::new(RefCell::new(
        ({
            let _arg0: i32 = (*x.borrow());
            (*(*fn_.borrow()))(_arg0)
        }),
    ));
    return (*result.borrow());
}
pub fn apply_binary_4(a: i32, b: i32, __args: &[VaArg]) -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    let b: Value<i32> = Rc::new(RefCell::new(b));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    (*ap.borrow_mut()) = VaList::new(__args);
    let fn_: Value<FnPtr<fn(i32, i32) -> i32>> = Rc::new(RefCell::new(
        ((*ap.borrow_mut()).arg::<FnPtr<fn(i32, i32) -> i32>>()).clone(),
    ));
    let result: Value<i32> = Rc::new(RefCell::new(
        ({
            let _arg0: i32 = (*a.borrow());
            let _arg1: i32 = (*b.borrow());
            (*(*fn_.borrow()))(_arg0, _arg1)
        }),
    ));
    return (*result.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (((({
            let _x: i32 = 5;
            apply_unary_3(_x, &[FnPtr::<fn(i32) -> i32>::new(square_0).into()])
        }) == 25) as i32)
            != 0)
    );
    assert!(
        (((({
            let _x: i32 = 7;
            apply_unary_3(_x, &[FnPtr::<fn(i32) -> i32>::new(negate_1).into()])
        }) == -7_i32) as i32)
            != 0)
    );
    assert!(
        (((({
            let _a: i32 = 3;
            let _b: i32 = 4;
            apply_binary_4(_a, _b, &[FnPtr::<fn(i32, i32) -> i32>::new(add_2).into()])
        }) == 7) as i32)
            != 0)
    );
    return 0;
}
