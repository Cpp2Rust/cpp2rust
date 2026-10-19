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
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = 1;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 2;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 3;
        (*v.borrow_mut()).push(__a1)
    };
    let mut square: i32 = 0;
    let __begin1: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>)),
    ));
    let __end1: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>).to_end()),
    ));
    'loop_: while (*__begin1.borrow()) != (*__end1.borrow()) {
        let mut e1: i32 = ((*__begin1.borrow()).iter_deref().read());
        let __begin2: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
            RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>)),
        ));
        let __end2: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
            RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>).to_end()),
        ));
        'loop_: while (*__begin2.borrow()) != (*__end2.borrow()) {
            let mut e2: i32 = ((*__begin2.borrow()).iter_deref().read());
            square += (e1 * e2);
            (*__begin2.borrow_mut()).iter_inc();
        }
        (*__begin1.borrow_mut()).iter_inc();
    }
    let __begin1: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>)),
    ));
    let __end1: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>).to_end()),
    ));
    'loop_: while (*__begin1.borrow()) != (*__end1.borrow()) {
        let e1: Ptr<i32> = (*__begin1.borrow()).iter_deref();
        let __begin2: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
            RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>)),
        ));
        let __end2: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
            RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>).to_end()),
        ));
        'loop_: while (*__begin2.borrow()) != (*__end2.borrow()) {
            let e2: Ptr<i32> = (*__begin2.borrow()).iter_deref();
            square += { ({ (e1.read()) } * { (e2.read()) }) };
            (*__begin2.borrow_mut()).iter_inc();
        }
        (*__begin1.borrow_mut()).iter_inc();
    }
    let __begin1: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>)),
    ));
    let __end1: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>).to_end()),
    ));
    'loop_: while (*__begin1.borrow()) != (*__end1.borrow()) {
        let e1: Ptr<i32> = (*__begin1.borrow()).iter_deref();
        let __begin2: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
            RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>)),
        ));
        let __end2: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
            RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>).to_end()),
        ));
        'loop_: while (*__begin2.borrow()) != (*__end2.borrow()) {
            let e2: Ptr<i32> = (*__begin2.borrow()).iter_deref();
            square += { ({ (e1.read()) } * { (e2.read()) }) };
            (*__begin2.borrow_mut()).iter_inc();
        }
        (*__begin1.borrow_mut()).iter_inc();
    }
    let __begin1: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>)),
    ));
    let __end1: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>).to_end()),
    ));
    'loop_: while (*__begin1.borrow()) != (*__end1.borrow()) {
        let e1: Ptr<i32> = (*__begin1.borrow()).iter_deref();
        let __begin2: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
            RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>)),
        ));
        let __end2: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
            RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>).to_end()),
        ));
        'loop_: while (*__begin2.borrow()) != (*__end2.borrow()) {
            let e2: Ptr<i32> = (*__begin2.borrow()).iter_deref();
            square += { ({ (e1.read()) } * { (e2.read()) }) };
            (*__begin2.borrow_mut()).iter_inc();
        }
        (*__begin1.borrow_mut()).iter_inc();
    }
    let m: Value<Vec<Vec<i32>>> = Rc::new(RefCell::new(Vec::new()));
    let v1: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = std::mem::take(&mut (*v1.borrow_mut()));
        (*m.borrow_mut()).push(__a1)
    };
    let v2: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = std::mem::take(&mut (*v2.borrow_mut()));
        (*m.borrow_mut()).push(__a1)
    };
    let v3: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = std::mem::take(&mut (*v3.borrow_mut()));
        (*m.borrow_mut()).push(__a1)
    };
    let __begin1: Value<RandomAccessIterator<Vec<i32>>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((m.as_pointer() as Ptr<Vec<i32>>)),
    ));
    let __end1: Value<RandomAccessIterator<Vec<i32>>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((m.as_pointer() as Ptr<Vec<i32>>).to_end()),
    ));
    'loop_: while (*__begin1.borrow()) != (*__end1.borrow()) {
        let row: Ptr<Vec<i32>> = (*__begin1.borrow()).iter_deref();
        let __begin2: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
            RandomAccessIterator::Vec((Ptr::<Vec<i32>>::decay(&(row)) as Ptr<i32>)),
        ));
        let __end2: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
            RandomAccessIterator::Vec((Ptr::<Vec<i32>>::decay(&(row)) as Ptr<i32>).to_end()),
        ));
        'loop_: while (*__begin2.borrow()) != (*__end2.borrow()) {
            let col: Ptr<i32> = (*__begin2.borrow()).iter_deref();
            square += { (col.read()) };
            (*__begin2.borrow_mut()).iter_inc();
        }
        (*__begin1.borrow_mut()).iter_inc();
    }
    assert!((square == 144));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
