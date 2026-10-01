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
    return 0;
}
pub fn __cpp2rust_init_globals() {}
