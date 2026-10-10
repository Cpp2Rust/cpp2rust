extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn square_0(mut x: i32) -> i32 {
    return (x * x);
}
pub fn negate_1(mut x: i32) -> i32 {
    return -x;
}
pub fn add_2(mut a: i32, mut b: i32) -> i32 {
    return (a + b);
}
pub fn apply_unary_3(x: i32, __args: &[VaArg]) -> i32 {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*ap.borrow_mut()) = __args.as_pointer();
    let mut fn_: FnPtr<fn(i32) -> i32> = (*ap.borrow_mut()).arg::<FnPtr<fn(i32) -> i32>>();
    let mut result: i32 = ({ fn_.call((*x.borrow())) });
    return result;
}
pub fn apply_binary_4(mut a: i32, b: i32, __args: &[VaArg]) -> i32 {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let b: Value<i32> = Rc::new(RefCell::new(b));
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*ap.borrow_mut()) = __args.as_pointer();
    let mut fn_: FnPtr<fn(i32, i32) -> i32> =
        (*ap.borrow_mut()).arg::<FnPtr<fn(i32, i32) -> i32>>();
    let mut result: i32 = ({ fn_.call(a, (*b.borrow())) });
    return result;
}
pub fn not_supported_5(mut ctx: AnyPtr, mut fn_: FnPtr<fn(i32) -> i32>, mut extra: AnyPtr) -> i32 {
    &(ctx);
    &(fn_);
    &(extra);
    return -3_i32;
}
pub fn main() {
    __cpp2rust_init_globals();
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
pub fn __cpp2rust_init_globals() {}
