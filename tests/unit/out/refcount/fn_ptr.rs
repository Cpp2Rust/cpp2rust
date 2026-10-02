extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn my_foo_0(p: AnyPtr) -> i32 {
    let p: Value<AnyPtr> = Rc::new(RefCell::new(p));
    return ((*p.borrow()).reinterpret_cast::<i32>().read());
}
pub fn foo_1(fn_: FnPtr<fn(AnyPtr) -> i32>, pi: Ptr<i32>) -> i32 {
    let fn_: Value<FnPtr<fn(AnyPtr) -> i32>> = Rc::new(RefCell::new(fn_));
    let pi: Value<Ptr<i32>> = Rc::new(RefCell::new(pi));
    return ({ (*fn_.borrow()).call((*pi.borrow()).to_any()) });
}
pub fn twice_2(x: u64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    return (*x.borrow()).wrapping_mul(2_u64);
}
pub fn twice_in_place_3(x: Ptr<u64>) -> u64 {
    x.write({ (x.read()).wrapping_mul(2_u64) });
    return (x.read());
}
pub fn ret_size_4(v: i32) -> usize {
    let v: Value<i32> = Rc::new(RefCell::new(v));
    return (((*v.borrow()) + 1) as usize);
}
pub fn call_fn_5(f: FnPtr<fn(i32) -> usize>, v: i32) -> usize {
    let f: Value<FnPtr<fn(i32) -> usize>> = Rc::new(RefCell::new(f));
    let v: Value<i32> = Rc::new(RefCell::new(v));
    return ({ (*f.borrow()).call((*v.borrow())) }).wrapping_mul(2_usize);
}
pub fn identity_hash_6(v: bool) -> usize {
    let v: Value<bool> = Rc::new(RefCell::new(v));
    return ((*v.borrow()) as usize);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct HashHolder_unsigned_long__ptr__bool__ {
    #[offset(0)]
    #[byte_size(8)]
    pub h: FnPtr<fn(bool) -> u64>,
}
impl HashHolder_unsigned_long__ptr__bool__ {
    pub fn new(h: Ptr<FnPtr<fn(bool) -> u64>>) -> Self {
        let __this: Value<HashHolder_unsigned_long__ptr__bool__> = Rc::new(RefCell::new(Self {
            h: (h.read()).clone(),
        }));
        let this: Ptr<HashHolder_unsigned_long__ptr__bool__> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let fn_: Value<FnPtr<fn(AnyPtr) -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn(AnyPtr) -> i32>::null()));
    assert!((*fn_.borrow()).is_null());
    assert!(({ (*fn_.borrow()).clone() } != { FnPtr::<fn(AnyPtr) -> i32>::new(my_foo_0) }));
    (*fn_.borrow_mut()) = FnPtr::<fn(AnyPtr) -> i32>::new(my_foo_0);
    assert!(!((*fn_.borrow()).is_null()));
    assert!(({ (*fn_.borrow()).clone() } == { FnPtr::<fn(AnyPtr) -> i32>::new(my_foo_0) }));
    let a: Value<i32> = Rc::new(RefCell::new(10));
    assert!(({ ({ foo_1((*fn_.borrow()).clone(), (a.as_pointer()),) }) } == { (*a.borrow()) }));
    let ul_fn: Value<FnPtr<fn(u64) -> u64>> =
        Rc::new(RefCell::new((FnPtr::<fn(u64) -> u64>::new(twice_2))));
    let n: Value<usize> = Rc::new(RefCell::new(21_usize));
    let r: Value<usize> = Rc::new(RefCell::new(
        (({ (*ul_fn.borrow()).call(((*n.borrow()) as u64)) }) as usize),
    ));
    assert!(((*r.borrow()) == 42_usize));
    let ul_ref_fn: Value<FnPtr<fn(Ptr<u64>) -> u64>> = Rc::new(RefCell::new(
        (FnPtr::<fn(Ptr<u64>) -> u64>::new(twice_in_place_3)),
    ));
    let m: Value<usize> = Rc::new(RefCell::new(21_usize));
    let q: Value<usize> = Rc::new(RefCell::new(
        (({ (*ul_ref_fn.borrow()).call((m.as_pointer()).reinterpret_cast::<u64>()) }) as usize),
    ));
    assert!(((*q.borrow()) == 42_usize));
    assert!(((*m.borrow()) == 42_usize));
    assert!((({ call_fn_5(FnPtr::<fn(i32) -> usize>::new(ret_size_4), 3,) }) == 8_usize));
    let hh: Value<HashHolder_unsigned_long__ptr__bool__> = Rc::new(RefCell::new({
        let __tmp_0: Value<FnPtr<fn(bool) -> u64>> = Rc::new(RefCell::new(
            (FnPtr::<fn(bool) -> usize>::new(identity_hash_6)).cast::<fn(bool) -> u64>(),
        ));
        HashHolder_unsigned_long__ptr__bool__::new({ __tmp_0.as_pointer() })
    }));
    assert!((({ { (*hh.borrow()).h.clone() }.call(true,) }) == 1_u64));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
