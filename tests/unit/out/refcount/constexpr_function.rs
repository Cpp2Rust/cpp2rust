extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn runtime_only_0(mut x: i32) -> i32 {
    return (x * 2);
}
pub fn first_1(mut p: Ptr<i32>) -> i32 {
    return (p.read());
}
pub fn scaled_2(mut x: i32) -> i32 {
    if (x < 0) {
        return ({ runtime_only_0(-x) });
    }
    return x;
}
pub fn half_3(mut x: f64) -> f64 {
    return (x / 2.0E+0);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Flag {
    #[offset(0)]
    pub v: i32,
}
pub fn use_4(f: Flag) -> i32 {
    let f: Value<Flag> = Rc::new(RefCell::new(f));
    assert!(({ FlagImpl::to_bool(&f.as_pointer(),) }));
    return { (*f.borrow()).v };
}
pub fn checked_5(mut x: i32) -> i32 {
    assert!((x > 0));
    return (x + 1);
}
pub fn in_constant_context_6() -> bool {
    if false {
        return true;
    }
    return (({ runtime_only_0(1) }) == 0);
}
const _: () = assert!(true, "in_constant_context()");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct P {
    #[offset(0)]
    pub v: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut runtime_context: bool = ({ in_constant_context_6() });
    assert!(!(runtime_context));
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([7, 8])));
    assert!((({ first_1((arr.as_pointer() as Ptr::<i32>),) }) == 7));
    assert!((({ first_1((arr.as_pointer() as Ptr::<i32>).offset((1) as isize),) }) == 8));
    assert!((({ scaled_2(3,) }) == 3));
    assert!((({ scaled_2(-3_i32,) }) == 6));
    assert!((({ half_3(5.0E+0,) }) == 2.5E+0));
    assert!((({ checked_5(1,) }) == 2));
    let mut c: i32 = ({ checked_5(4) });
    assert!((c == 5));
    assert!((({ use_4(Flag { v: 2 },) }) == 2));
    let mut u: i32 = ({ use_4(Flag { v: 3 }) });
    assert!((u == 3));
    let mut ptr: Ptr<i32> = (arr.as_pointer() as Ptr<i32>);
    assert!(!(ptr).is_null());
    let p: Value<P> = Rc::new(RefCell::new(P { v: 9 }));
    assert!((({ PImpl::get(&p.as_pointer(),) }) == 9));
    let mut k: i32 = ({ scaled_2(4) });
    assert!((k == 4));
    return 0;
}
pub trait FlagImpl {
    fn to_bool(&self) -> bool;
}
impl FlagImpl for Ptr<Flag> {
    fn to_bool(&self) -> bool {
        return ((*self).with(|__s| __s.v) != 0);
    }
}
pub trait PImpl {
    fn get(&self) -> i32;
}
impl PImpl for Ptr<P> {
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.v);
    }
}
pub fn __cpp2rust_init_globals() {}
