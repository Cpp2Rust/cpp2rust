extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn cmp_eq_0(rc: i32) -> i32 {
    let rc: Value<i32> = Rc::new(RefCell::new(rc));
    return (((*rc.borrow()) == -1_i32) as i32);
}
pub fn cmp_or_ptr_1(p: Ptr<u8>, q: Ptr<u8>) -> i32 {
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new(p));
    let q: Value<Ptr<u8>> = Rc::new(RefCell::new(q));
    return (((!(*p.borrow()).is_null()) || (!(*q.borrow()).is_null())) as i32).clone();
}
pub fn both_null_2(s1: Ptr<u8>, s2: Ptr<u8>) -> i32 {
    let s1: Value<Ptr<u8>> = Rc::new(RefCell::new(s1));
    let s2: Value<Ptr<u8>> = Rc::new(RefCell::new(s2));
    return ((((*s1.borrow()).is_null()) && ((*s2.borrow()).is_null())) as i32).clone();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(0));
    let b: Value<i32> = <Value<i32>>::default();
    if ({
        (*b.borrow_mut()) = (*a.borrow());
        (*b.borrow())
    } != 0)
    {}
    'loop_: while (({
        (*b.borrow_mut()) = (*a.borrow());
        (*b.borrow())
    }) != 0)
    {}
    if ((*a.borrow()) != 0) {}
    if ((*a.borrow()) == (*b.borrow())) {}
    if ((*a.borrow()) < (*b.borrow())) {}
    assert!(((*a.borrow()) == (*b.borrow())));
    assert!(
        !(({
            (*a.borrow_mut()) = (*b.borrow());
            (*a.borrow())
        }) != 0)
    );
    let c: Value<bool> = <Value<bool>>::default();
    (*c.borrow_mut()) = ({
        (*a.borrow_mut()) = (*b.borrow());
        (*a.borrow())
    } != 0);
    (*c.borrow_mut()) = (({
        (*b.borrow_mut()) = (*a.borrow());
        (*b.borrow())
    }) != 0);
    (*c.borrow_mut()) = ((*a.borrow()) != 0);
    (*c.borrow_mut()) = ((*a.borrow()) == (*b.borrow()));
    (*c.borrow_mut()) = ((*a.borrow()) < (*b.borrow()));
    let x: Value<i32> = Rc::new(RefCell::new(5));
    let y: Value<i32> = Rc::new(RefCell::new(5));
    let eq: Value<i32> = Rc::new(RefCell::new((((*x.borrow()) == (*y.borrow())) as i32)));
    let lt: Value<i32> = Rc::new(RefCell::new((((*x.borrow()) < (*y.borrow())) as i32)));
    let neq: Value<i32> = Rc::new(RefCell::new((((*x.borrow()) != (*y.borrow())) as i32)));
    assert!(((*eq.borrow()) == 1));
    assert!(((*lt.borrow()) == 0));
    assert!(((*neq.borrow()) == 0));
    let p1: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal("hi")));
    let p2: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::<u8>::null()));
    let either: Value<i32> = Rc::new(RefCell::new(
        (((!(*p1.borrow()).is_null()) || (!(*p2.borrow()).is_null())) as i32).clone(),
    ));
    let both: Value<i32> = Rc::new(RefCell::new(
        (((!(*p1.borrow()).is_null()) && (!(*p2.borrow()).is_null())) as i32).clone(),
    ));
    assert!(((*either.borrow()) == 1));
    assert!(((*both.borrow()) == 0));
    assert!(
        (({
            let _rc: i32 = -1_i32;
            cmp_eq_0(_rc)
        }) == 1)
    );
    assert!(
        (({
            let _rc: i32 = 0;
            cmp_eq_0(_rc)
        }) == 0)
    );
    assert!(
        (({
            let _p: Ptr<u8> = (*p1.borrow()).clone();
            let _q: Ptr<u8> = (*p2.borrow()).clone();
            cmp_or_ptr_1(_p, _q)
        }) == 1)
    );
    assert!(
        (({
            let _p: Ptr<u8> = Ptr::<u8>::null();
            let _q: Ptr<u8> = Ptr::<u8>::null();
            cmp_or_ptr_1(_p, _q)
        }) == 0)
    );
    assert!(
        (({
            let _s1: Ptr<u8> = Ptr::<u8>::null();
            let _s2: Ptr<u8> = Ptr::<u8>::null();
            both_null_2(_s1, _s2)
        }) == 1)
    );
    assert!(
        (({
            let _s1: Ptr<u8> = (*p1.borrow()).clone();
            let _s2: Ptr<u8> = Ptr::<u8>::null();
            both_null_2(_s1, _s2)
        }) == 0)
    );
    return 0;
}
