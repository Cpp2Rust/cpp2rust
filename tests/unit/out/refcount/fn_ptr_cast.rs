extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn double_it_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ((*x.borrow()) * 2);
}
pub fn test_roundtrip_1() {
    let fn_: Value<FnPtr<fn(i32) -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn(i32) -> i32>::new(double_it_0)));
    assert!((({ (*fn_.borrow()).call(5,) }) == 10));
    let gfn: Value<FnPtr<fn()>> = Rc::new(RefCell::new((*fn_.borrow()).cast::<fn()>()));
    assert!(!((*gfn.borrow()).is_null()));
    let fn2: Value<FnPtr<fn(i32) -> i32>> =
        Rc::new(RefCell::new((*gfn.borrow()).cast::<fn(i32) -> i32>()));
    assert!((({ (*fn2.borrow()).call(5,) }) == 10));
    assert!(({ (*fn2.borrow()).clone() } == { (*fn_.borrow()).clone() }));
}
pub fn test_double_cast_2() {
    let fn_: Value<FnPtr<fn(i32) -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn(i32) -> i32>::new(double_it_0)));
    let fn2: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(
        (*fn_.borrow()).cast::<fn()>().cast::<fn(i32) -> i32>(),
    ));
    assert!((({ (*fn2.borrow()).call(5,) }) == 10));
    assert!(({ (*fn2.borrow()).clone() } == { (*fn_.borrow()).clone() }));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Command {
    #[offset(0)]
    #[byte_size(8)]
    pub data: AnyPtr,
}
pub fn test_void_ptr_to_fn_3() {
    let cmd: Value<Command> = Rc::new(RefCell::new(<Command>::default()));
    (*cmd.borrow_mut()).data = FnPtr::<fn(i32) -> i32>::new(double_it_0).to_any();
    let fn_: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(
        { (*cmd.borrow()).data.clone() }
            .cast_fn::<fn(i32) -> i32>()
            .expect("ub:wrong fn type"),
    ));
    assert!((({ (*fn_.borrow()).call(5,) }) == 10));
}
pub fn add_offset_4(base: Ptr<i32>, offset: i32) -> i32 {
    let base: Value<Ptr<i32>> = Rc::new(RefCell::new(base));
    let offset: Value<i32> = Rc::new(RefCell::new(offset));
    return ({ ((*base.borrow()).read()) } + { (*offset.borrow()) });
}
pub fn test_call_through_cast_5() {
    let gfn: Value<FnPtr<fn(AnyPtr, i32) -> i32>> = Rc::new(RefCell::new(
        FnPtr::<fn(Ptr<i32>, i32) -> i32>::new(add_offset_4).cast::<fn(AnyPtr, i32) -> i32>(),
    ));
    let val: Value<i32> = Rc::new(RefCell::new(100));
    let result: Value<i32> = Rc::new(RefCell::new(
        ({ (*gfn.borrow()).call(((val.as_pointer()) as Ptr<i32>).to_any(), 42) }),
    ));
    assert!(((*result.borrow()) == 142));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_roundtrip_1() });
    ({ test_double_cast_2() });
    ({ test_void_ptr_to_fn_3() });
    ({ test_call_through_cast_5() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
