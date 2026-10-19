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
    let __begin1: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v1.as_pointer() as Ptr<i32>)),
    ));
    let __end1: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v1.as_pointer() as Ptr<i32>).to_end()),
    ));
    'loop_: while (*__begin1.borrow()) != (*__end1.borrow()) {
        let mut x: i32 = ((*__begin1.borrow()).iter_deref().read());
        sum += x.prefix_inc();
        (*__begin1.borrow_mut()).iter_inc();
    }
    let __begin1: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v1.as_pointer() as Ptr<i32>)),
    ));
    let __end1: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v1.as_pointer() as Ptr<i32>).to_end()),
    ));
    'loop_: while (*__begin1.borrow()) != (*__end1.borrow()) {
        let mut x: i32 = ((*__begin1.borrow()).iter_deref().read());
        sum += x;
        (*__begin1.borrow_mut()).iter_inc();
    }
    let __begin1: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v1.as_pointer() as Ptr<i32>)),
    ));
    let __end1: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v1.as_pointer() as Ptr<i32>).to_end()),
    ));
    'loop_: while (*__begin1.borrow()) != (*__end1.borrow()) {
        let x: Ptr<i32> = (*__begin1.borrow()).iter_deref();
        {
            x.with_mut(|__v| *__v = *__v + 10)
        };
        (*__begin1.borrow_mut()).iter_inc();
    }
    let __begin1: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v1.as_pointer() as Ptr<i32>)),
    ));
    let __end1: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v1.as_pointer() as Ptr<i32>).to_end()),
    ));
    'loop_: while (*__begin1.borrow()) != (*__end1.borrow()) {
        let x: Ptr<i32> = (*__begin1.borrow()).iter_deref();
        sum += { (x.read()) };
        (*__begin1.borrow_mut()).iter_inc();
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
    let __begin1: Value<RandomAccessIterator<Ptr<i32>>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v2.as_pointer() as Ptr<Ptr<i32>>)),
    ));
    let __end1: Value<RandomAccessIterator<Ptr<i32>>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v2.as_pointer() as Ptr<Ptr<i32>>).to_end()),
    ));
    'loop_: while (*__begin1.borrow()) != (*__end1.borrow()) {
        let mut p: Ptr<i32> = ((*__begin1.borrow()).iter_deref().read());
        {
            p.with_mut(|__v| *__v = *__v + 5)
        };
        (*__begin1.borrow_mut()).iter_inc();
    }
    let __begin1: Value<RandomAccessIterator<Ptr<i32>>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v2.as_pointer() as Ptr<Ptr<i32>>)),
    ));
    let __end1: Value<RandomAccessIterator<Ptr<i32>>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v2.as_pointer() as Ptr<Ptr<i32>>).to_end()),
    ));
    'loop_: while (*__begin1.borrow()) != (*__end1.borrow()) {
        let mut p: Ptr<i32> = ((*__begin1.borrow()).iter_deref().read());
        sum += { (p.read()) };
        (*__begin1.borrow_mut()).iter_inc();
    }
    let __begin1: Value<RandomAccessIterator<Ptr<i32>>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v2.as_pointer() as Ptr<Ptr<i32>>)),
    ));
    let __end1: Value<RandomAccessIterator<Ptr<i32>>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v2.as_pointer() as Ptr<Ptr<i32>>).to_end()),
    ));
    'loop_: while (*__begin1.borrow()) != (*__end1.borrow()) {
        let mut p: Ptr<i32> = ((*__begin1.borrow()).iter_deref().read());
        {
            p.with_mut(|__v| *__v = *__v + 5)
        };
        (*__begin1.borrow_mut()).iter_inc();
    }
    let __begin1: Value<RandomAccessIterator<Ptr<i32>>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v2.as_pointer() as Ptr<Ptr<i32>>)),
    ));
    let __end1: Value<RandomAccessIterator<Ptr<i32>>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v2.as_pointer() as Ptr<Ptr<i32>>).to_end()),
    ));
    'loop_: while (*__begin1.borrow()) != (*__end1.borrow()) {
        let mut p: Ptr<i32> = ((*__begin1.borrow()).iter_deref().read());
        sum += { (p.read()) };
        (*__begin1.borrow_mut()).iter_inc();
    }
    assert!((sum == 168));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
