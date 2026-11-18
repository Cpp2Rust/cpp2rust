extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct p1_result_is_updated_operand_Counter {
    #[offset(0)]
    pub n: i32,
}
pub fn test_0() {
    let x: Value<i32> = Rc::new(RefCell::new(0));
    let r: Ptr<i32> = {
        let __tmp_0 = x.as_pointer();
        __tmp_0.with_mut(|__v| {
            __v.prefix_inc();
        });
        __tmp_0
    };
    r.write(5);
    assert!(({ ((*x.borrow()) == 5) }));
    assert!(({ (r).clone() } == { (x.as_pointer()) }));
    (*(*x.borrow_mut()).prefix_inc()) = 7;
    assert!(({ ((*x.borrow()) == 7) }));
    let mut y: i32 = (*(*x.borrow_mut()).prefix_inc());
    assert!(({ (y == 8) && ((*x.borrow()) == 8) }));
    let s: Ptr<i32> = (r).clone();
    {
        let __tmp_1 = s.clone();
        __tmp_1.with_mut(|__v| {
            __v.prefix_inc();
        });
        __tmp_1
    }
    .write(1);
    assert!(({ ((*x.borrow()) == 1) }));
    let mut t: i32 = s.with_mut(|__v| __v.prefix_inc().clone());
    assert!(({ (t == 2) && ((*x.borrow()) == 2) }));
    let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(a.decay()));
    let q: Ptr<Ptr<i32>> = {
        let __tmp_2 = p.as_pointer();
        __tmp_2.with_mut(|__v| {
            __v.prefix_inc();
        });
        __tmp_2
    };
    assert!(({ (q).clone() } == { (p.as_pointer()) }));
    assert!((((q.read()).read()) == 2));
    assert!(({ (((*(*p.borrow_mut()).prefix_inc()).read()) == 3) }));
    let c: Value<p1_result_is_updated_operand_Counter> =
        Rc::new(RefCell::new(p1_result_is_updated_operand_Counter { n: 0 }));
    let mut pc: Ptr<p1_result_is_updated_operand_Counter> = (c.as_pointer());
    let mut v: i32 = { (*c.borrow_mut()).n.prefix_inc().clone() };
    assert!((v == 1) && (pc.with(|__s| __s.n) == 1));
    (*(*c.borrow_mut()).n.prefix_inc()) = 4;
    assert!((pc.with(|__s| __s.n) == 4));
    let mut sum: i32 = 0;
    {
        let mut k: i32 = 0;
        'loop_: while (k < 3) {
            sum += (*y.prefix_inc());
            k.prefix_inc();
        }
    }
    assert!((sum == 30) && (y == 11));
}
pub fn test_1() {
    let counter: Value<u32> = Rc::new(RefCell::new(0_u32));
    let pre: Ptr<u32> = {
        let __tmp_3 = counter.as_pointer();
        __tmp_3.with_mut(|__v| {
            __v.prefix_dec();
        });
        __tmp_3
    };
    assert!(
        ({ ({ (pre).clone() } == { (counter.as_pointer()) }) && ((*counter.borrow()) == !0_u32) })
    );
    let x: Value<i32> = Rc::new(RefCell::new(5));
    let r: Ptr<i32> = x.as_pointer();
    r.with_mut(|__v| {
        __v.prefix_dec();
    });
    assert!(({ ((*x.borrow()) == 4) }));
    {
        let __tmp_4 = r.clone();
        __tmp_4.with_mut(|__v| {
            __v.prefix_dec();
        });
        __tmp_4
    }
    .write(9);
    assert!(({ ((*x.borrow()) == 9) }));
    let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    let mut p: Ptr<i32> = a.decay().offset((2) as isize);
    let mut n: Ptr<i32> = (*p.prefix_dec()).clone();
    assert!(((n.read()) == 2) && ({ (n).clone() } == { (p).clone() }));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_0() });
    ({ test_1() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
