extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn my_foo_0(mut p: AnyPtr) -> i32 {
    return (p.reinterpret_cast::<i32>().read());
}
pub fn foo_1(mut fn_: FnPtr<fn(AnyPtr) -> i32>, mut pi: Ptr<i32>) -> i32 {
    return ({ fn_.call((pi).to_any()) });
}
pub fn twice_2(mut x: u64) -> u64 {
    return (x).wrapping_mul(2_u64);
}
pub fn twice_in_place_3(x: Ptr<u64>) -> u64 {
    x.write({ (x.read()).wrapping_mul(2_u64) });
    return (x.read());
}
pub fn ret_size_4(mut v: i32) -> usize {
    return ((v + 1) as usize);
}
pub fn call_fn_5(mut f: FnPtr<fn(i32) -> usize>, mut v: i32) -> usize {
    return ({ f.call(v) }).wrapping_mul(2_usize);
}
pub fn identity_hash_6(mut v: bool) -> usize {
    return (v as usize);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(FnPtr<fn(bool  ,) -> u64  >  , new, ptr)]
#[byte_size(8)]
pub struct HashHolder_unsigned_long__ptr__bool__ {
    #[offset(0)]
    #[byte_size(8)]
    pub h: FnPtr<fn(bool) -> u64>,
}
impl HashHolder_unsigned_long__ptr__bool__ {
    pub fn new(h: Ptr<FnPtr<fn(bool) -> u64>>) -> Self {
        Self { h: (h.read()) }
    }
}
thread_local!(
    pub static total_7: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn add_to_total_8(mut x: i32) {
    total_7.with(|rc| *rc.borrow_mut() += x);
}
pub fn pair_sum_9(p: (Value<i32>, Value<i32>)) -> i32 {
    let p: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new(p));
    return ((*(*p.borrow()).0.borrow()) + (*(*p.borrow()).1.borrow()));
}
pub fn string_size_10(mut s: Vec<i8>) -> usize {
    return (s.len() - 1);
}
pub fn vector_sum_11(mut v: Vec<i32>) -> i32 {
    return (v[0_usize] + v[1_usize]);
}
pub fn is_less_12(o: std::cmp::Ordering) -> bool {
    let o: Value<std::cmp::Ordering> = Rc::new(RefCell::new(o));
    return ({ (*o.borrow()) }) == std::cmp::Ordering::Less;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut fn_: FnPtr<fn(AnyPtr) -> i32> = FnPtr::<fn(AnyPtr) -> i32>::null();
    assert!((fn_).is_null());
    assert!(({ (fn_).clone() } != { FnPtr::<fn(AnyPtr) -> i32>::new(my_foo_0) }));
    fn_ = FnPtr::<fn(AnyPtr) -> i32>::new(my_foo_0);
    assert!(!((fn_).is_null()));
    assert!(({ (fn_).clone() } == { FnPtr::<fn(AnyPtr) -> i32>::new(my_foo_0) }));
    let a: Value<i32> = Rc::new(RefCell::new(10));
    assert!(({ ({ ({ foo_1((fn_).clone(), (a.as_pointer()),) }) } == { (*a.borrow()) }) }));
    let mut ul_fn: FnPtr<fn(u64) -> u64> = (FnPtr::<fn(u64) -> u64>::new(twice_2));
    let mut n: usize = 21_usize;
    let mut r: usize = (({ ul_fn.call((n as u64)) }) as usize);
    assert!((r == 42_usize));
    let mut ul_ref_fn: FnPtr<fn(Ptr<u64>) -> u64> =
        (FnPtr::<fn(Ptr<u64>) -> u64>::new(twice_in_place_3));
    let m: Value<usize> = Rc::new(RefCell::new(21_usize));
    let mut q: usize = (({ ul_ref_fn.call((m.as_pointer()).reinterpret_cast::<u64>()) }) as usize);
    assert!((q == 42_usize));
    assert!(({ ((*m.borrow()) == 42_usize) }));
    assert!((({ call_fn_5(FnPtr::<fn(i32) -> usize>::new(ret_size_4), 3,) }) == 8_usize));
    let mut hh: HashHolder_unsigned_long__ptr__bool__ = {
        let __tmp_0: Value<FnPtr<fn(bool) -> u64>> = Rc::new(RefCell::new(
            (FnPtr::<fn(bool) -> usize>::new(identity_hash_6)).cast::<fn(bool) -> u64>(),
        ));
        HashHolder_unsigned_long__ptr__bool__::new({ __tmp_0.as_pointer() })
    };
    assert!((({ hh.h.call(true,) }) == 1_u64));
    let mut handler: FnPtr<fn(i32)> = FnPtr::<fn(i32)>::new(add_to_total_8);
    ({ handler.call(2) });
    ({ handler.call(3) });
    assert!((total_7.with(|rc| *rc.borrow()) == 5));
    let mut pf: FnPtr<fn((Value<i32>, Value<i32>)) -> i32> =
        FnPtr::<fn((Value<i32>, Value<i32>)) -> i32>::new(pair_sum_9);
    assert!((({ pf.call((Rc::new(RefCell::new(1)), Rc::new(RefCell::new(2))),) }) == 3));
    let mut sf: FnPtr<fn(Vec<i8>) -> usize> = FnPtr::<fn(Vec<i8>) -> usize>::new(string_size_10);
    assert!(
        (({
            sf.call({
                let mut __bytes = Ptr::<i8>::from_string_literal(b"abc").to_c_bytes();
                __bytes.push(0);
                __bytes
            })
        }) == 3_usize)
    );
    let mut vf: FnPtr<fn(Vec<i32>) -> i32> = FnPtr::<fn(Vec<i32>) -> i32>::new(vector_sum_11);
    assert!((({ vf.call(Vec::from([1, 2,]),) }) == 3));
    let mut of: FnPtr<fn(std::cmp::Ordering) -> bool> =
        FnPtr::<fn(std::cmp::Ordering) -> bool>::new(is_less_12);
    assert!(({ of.call(std::cmp::Ord::cmp(&(1), &(2)),) }));
    assert!(!({ of.call(std::cmp::Ord::cmp(&(2), &(1)),) }));
    let o: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(&(1), &(2))));
    let mut op: Ptr<std::cmp::Ordering> = (o.as_pointer());
    assert!(({ (*op.upgrade().deref()) }) == std::cmp::Ordering::Less);
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = total_7.with(|_| ());
}
