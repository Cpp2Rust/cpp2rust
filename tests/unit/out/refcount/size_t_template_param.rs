extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn array_ref_0(a: Ptr<u64>) -> u64 {
    elem!((a), 0).write({ (elem!((a), 0).read()).wrapping_add(1_u64) });
    return (elem!((a), (3_u64 as u64).wrapping_sub(1_u64)).read());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct PtrCtor_unsigned_long_ {
    #[offset(0)]
    pub v: u64,
}
impl PtrCtor_unsigned_long_ {
    pub fn new(mut p: Ptr<u64>) -> Self {
        Self {
            v: (elem!(p, 1).read()),
        }
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
        Self {
            v: (x.read()).wrapping_add(1_u64),
        }
    }
}
pub fn neg_ll_1() -> i64 {
    return -9000000_i64;
}
pub fn neg_char_2() -> i8 {
    return (b'\x80' as i8);
}
pub fn neg_int_3() -> i32 {
    return -1;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a1: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([1_usize, 2_usize, 3_usize])));
    assert!(
        (({ array_ref_0((a1.as_pointer() as Ptr<usize>).reinterpret_cast::<u64>(),) }) == 3_u64)
    );
    assert!(((*a1.borrow())[(0) as usize] == 2_usize));
    let a2: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([4_usize, 5_usize])));
    let mut pc: PtrCtor_unsigned_long_ =
        PtrCtor_unsigned_long_::new({ (a2.as_pointer() as Ptr<usize>).reinterpret_cast::<u64>() });
    assert!((pc.v == 5_u64));
    let v1: Value<usize> = Rc::new(RefCell::new(6_usize));
    let mut rc: RefCtor_unsigned_long_ =
        RefCtor_unsigned_long_::new({ (v1.as_pointer()).reinterpret_cast::<u64>() });
    assert!((rc.v == 7_u64));
    assert!((({ neg_ll_1() }) == (-9000000_i32 as i64)));
    assert!(((({ neg_char_2() }) as i32) == ((b'\x80' as i8) as i32)));
    assert!((({ neg_int_3() }) == -1_i32));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
