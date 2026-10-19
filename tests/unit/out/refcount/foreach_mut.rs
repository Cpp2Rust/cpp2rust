extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v1: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = 1;
        (*v1.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 2;
        (*v1.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 3;
        (*v1.borrow_mut()).push(__a1)
    };
    let mut sum: i32 = 0;
    'loop_: for x in IterRange::new(
        (v1.as_pointer() as Ptr<i32>).begin(),
        (v1.as_pointer() as Ptr<i32>).end(),
    ) {
        let mut x: i32 = (x.iter_deref().read());
        sum += x.prefix_inc();
    }
    'loop_: for x in IterRange::new(
        (v1.as_pointer() as Ptr<i32>).begin(),
        (v1.as_pointer() as Ptr<i32>).end(),
    ) {
        let mut x: i32 = (x.iter_deref().read());
        sum += x;
    }
    'loop_: for x in IterRange::new(
        (v1.as_pointer() as Ptr<i32>).begin(),
        (v1.as_pointer() as Ptr<i32>).end(),
    ) {
        let x: Ptr<i32> = x.iter_deref();
        {
            x.with_mut(|__v| *__v = *__v + 10)
        };
    }
    'loop_: for x in IterRange::new(
        (v1.as_pointer() as Ptr<i32>).begin(),
        (v1.as_pointer() as Ptr<i32>).end(),
    ) {
        let x: Ptr<i32> = x.iter_deref();
        sum += { (x.read()) };
    }
    let v2: Value<Vec<Ptr<i32>>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = ((v1.as_pointer() as Ptr<i32>).offset(0_usize));
        (*v2.borrow_mut()).push(__a1)
    };
    {
        let __a1 = ((v1.as_pointer() as Ptr<i32>).offset(1_usize));
        (*v2.borrow_mut()).push(__a1)
    };
    {
        let __a1 = ((v1.as_pointer() as Ptr<i32>).offset(2_usize));
        (*v2.borrow_mut()).push(__a1)
    };
    'loop_: for p in IterRange::new(
        (v2.as_pointer() as Ptr<Ptr<i32>>).begin(),
        (v2.as_pointer() as Ptr<Ptr<i32>>).end(),
    ) {
        let mut p: Ptr<i32> = (p.iter_deref().read());
        {
            p.with_mut(|__v| *__v = *__v + 5)
        };
    }
    'loop_: for p in IterRange::new(
        (v2.as_pointer() as Ptr<Ptr<i32>>).begin(),
        (v2.as_pointer() as Ptr<Ptr<i32>>).end(),
    ) {
        let mut p: Ptr<i32> = (p.iter_deref().read());
        sum += { (p.read()) };
    }
    'loop_: for p in IterRange::new(
        (v2.as_pointer() as Ptr<Ptr<i32>>).begin(),
        (v2.as_pointer() as Ptr<Ptr<i32>>).end(),
    ) {
        let mut p: Ptr<i32> = (p.iter_deref().read());
        {
            p.with_mut(|__v| *__v = *__v + 5)
        };
    }
    'loop_: for p in IterRange::new(
        (v2.as_pointer() as Ptr<Ptr<i32>>).begin(),
        (v2.as_pointer() as Ptr<Ptr<i32>>).end(),
    ) {
        let mut p: Ptr<i32> = (p.iter_deref().read());
        sum += { (p.read()) };
    }
    assert!((sum == 168));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
