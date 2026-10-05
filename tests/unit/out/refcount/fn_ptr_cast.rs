extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn double_it_0(mut x: i32) -> i32 {
    return (x * 2);
}
pub fn test_roundtrip_1() {
    let mut fn_: FnPtr<fn(i32) -> i32> = FnPtr::<fn(i32) -> i32>::new(double_it_0);
    assert!((({ fn_.call(5,) }) == 10));
    let mut gfn: FnPtr<fn()> = fn_.cast::<fn()>();
    assert!(!((gfn).is_null()));
    let mut fn2: FnPtr<fn(i32) -> i32> = gfn.cast::<fn(i32) -> i32>();
    assert!((({ fn2.call(5,) }) == 10));
    assert!(({ (fn2).clone() } == { (fn_).clone() }));
}
pub fn test_double_cast_2() {
    let mut fn_: FnPtr<fn(i32) -> i32> = FnPtr::<fn(i32) -> i32>::new(double_it_0);
    let mut fn2: FnPtr<fn(i32) -> i32> = fn_.cast::<fn()>().cast::<fn(i32) -> i32>();
    assert!((({ fn2.call(5,) }) == 10));
    assert!(({ (fn2).clone() } == { (fn_).clone() }));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Command {
    #[offset(0)]
    #[byte_size(8)]
    pub data: AnyPtr,
}
pub fn test_void_ptr_to_fn_3() {
    let mut cmd: Command = <Command>::default();
    cmd.data = FnPtr::<fn(i32) -> i32>::new(double_it_0).to_any();
    let mut fn_: FnPtr<fn(i32) -> i32> = cmd
        .data
        .cast_fn::<fn(i32) -> i32>()
        .expect("ub:wrong fn type");
    assert!((({ fn_.call(5,) }) == 10));
}
pub fn add_offset_4(mut base: Ptr<i32>, mut offset: i32) -> i32 {
    return ({ (base.read()) } + { offset });
}
pub fn test_call_through_cast_5() {
    let mut gfn: FnPtr<fn(AnyPtr, i32) -> i32> =
        FnPtr::<fn(Ptr<i32>, i32) -> i32>::new(add_offset_4).cast::<fn(AnyPtr, i32) -> i32>();
    let val: Value<i32> = Rc::new(RefCell::new(100));
    let mut result: i32 = ({ gfn.call(((val.as_pointer()) as Ptr<i32>).to_any(), 42) });
    assert!((result == 142));
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
