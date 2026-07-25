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
    let result: Value<i32> = Rc::new(RefCell::new(({ (*(*fn_.borrow()))((*x.borrow())) })));
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
        ({ (*(*fn_.borrow()))((*a.borrow()), (*b.borrow())) }),
    ));
    return (*result.borrow());
}
pub fn not_supported_5(ctx: AnyPtr, fn_: FnPtr<fn(i32) -> i32>, extra: AnyPtr) -> i32 {
    let ctx: Value<AnyPtr> = Rc::new(RefCell::new(ctx));
    let fn_: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(fn_));
    let extra: Value<AnyPtr> = Rc::new(RefCell::new(extra));
    (*ctx.borrow()).clone();
    (*fn_.borrow()).clone();
    (*extra.borrow()).clone();
    return -3_i32;
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (((({ apply_unary_3(5, &[(FnPtr::<fn(i32) -> i32>::new(square_0)).into(),]) }) == 25)
            as i32)
            != 0)
    );
    assert!(
        (((({ apply_unary_3(7, &[(FnPtr::<fn(i32) -> i32>::new(negate_1)).into(),]) }) == -7_i32)
            as i32)
            != 0)
    );
    assert!(
        (((({ apply_binary_4(3, 4, &[(FnPtr::<fn(i32, i32) -> i32>::new(add_2)).into(),]) }) == 7)
            as i32)
            != 0)
    );
    let dummy: Value<i32> = Rc::new(RefCell::new(0));
    assert!(
        (((({
            let _ctx: AnyPtr = ((dummy.as_pointer()) as Ptr<i32>).to_any();
            let _extra: AnyPtr = ((dummy.as_pointer()) as Ptr<i32>).to_any();
            not_supported_5(_ctx, FnPtr::<fn(i32) -> i32>::new(square_0), _extra)
        }) == -3_i32) as i32)
            != 0)
    );
    return 0;
}
