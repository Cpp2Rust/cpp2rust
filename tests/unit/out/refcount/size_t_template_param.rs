extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn array_ref_0(a: Ptr<u64>) -> u64 {
    (a).offset((0) as isize)
        .write({ ((a).offset((0) as isize).read()).wrapping_add(1_u64) });
    return ((a)
        .offset(((3_u64 as u64).wrapping_sub(1_u64)) as isize)
        .read());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct PtrCtor_unsigned_long_ {
    #[offset(0)]
    pub v: u64,
}
impl PtrCtor_unsigned_long_ {
    pub fn new(p: Ptr<u64>) -> Self {
        let p: Value<Ptr<u64>> = Rc::new(RefCell::new(p));
        let __this: Value<PtrCtor_unsigned_long_> = Rc::new(RefCell::new(Self {
            v: ((*p.borrow()).offset((1) as isize).read()),
        }));
        let this: Ptr<PtrCtor_unsigned_long_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct RefCtor_unsigned_long_ {
    #[offset(0)]
    pub v: u64,
}
impl RefCtor_unsigned_long_ {
    pub fn new(x: Ptr<u64>) -> Self {
        let __this: Value<RefCtor_unsigned_long_> = Rc::new(RefCell::new(Self {
            v: (x.read()).wrapping_add(1_u64),
        }));
        let this: Ptr<RefCtor_unsigned_long_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a1: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([1_usize, 2_usize, 3_usize])));
    assert!((({ array_ref_0((a1.as_pointer() as Ptr<u64>),) }) == 3_u64));
    assert!(((*a1.borrow())[(0) as usize] == 2_usize));
    let a2: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([4_usize, 5_usize])));
    let pc: Value<PtrCtor_unsigned_long_> = Rc::new(RefCell::new(PtrCtor_unsigned_long_::new({
        (a2.as_pointer() as Ptr<usize>).reinterpret_cast::<u64>()
    })));
    assert!(({ (*pc.borrow()).v } == 5_u64));
    let v1: Value<usize> = Rc::new(RefCell::new(6_usize));
    let rc: Value<RefCtor_unsigned_long_> = Rc::new(RefCell::new(RefCtor_unsigned_long_::new({
        (v1.as_pointer()).reinterpret_cast::<u64>()
    })));
    assert!(({ (*rc.borrow()).v } == 7_u64));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
