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
    let v: Value<Vec<Vec<i32>>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a0 = 2_usize as usize;
        (*v.borrow_mut()).resize_with(__a0, || <Vec<i32>>::default())
    };
    {
        let __a0 = 2_usize as usize;
        elem!((v.as_pointer() as Ptr<Vec<i32>>), 0_usize)
            .with_mut(|__v: &mut Vec<i32>| __v.resize_with(__a0, || <i32>::default()))
    };
    {
        let __a0 = 1_usize as usize;
        elem!((v.as_pointer() as Ptr<Vec<i32>>), 1_usize)
            .with_mut(|__v: &mut Vec<i32>| __v.resize_with(__a0, || <i32>::default()))
    };
    index!(elem!((v.as_pointer() as Ptr<Vec<i32>>), 0_usize), 0_usize).write(1);
    index!(elem!((v.as_pointer() as Ptr<Vec<i32>>), 0_usize), 1_usize).write(5);
    index!(elem!((v.as_pointer() as Ptr<Vec<i32>>), 1_usize), 0_usize).write(6);
    let __begin1: Value<RandomAccessIterator<Vec<i32>>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v.as_pointer() as Ptr<Vec<i32>>)),
    ));
    let __end1: Value<RandomAccessIterator<Vec<i32>>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v.as_pointer() as Ptr<Vec<i32>>).to_end()),
    ));
    'loop_: while (*__begin1.borrow()) != (*__end1.borrow()) {
        let v2: Ptr<Vec<i32>> = (*__begin1.borrow()).iter_deref();
        let __begin2: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
            RandomAccessIterator::Vec((Ptr::<Vec<i32>>::decay(&(v2)) as Ptr<i32>)),
        ));
        let __end2: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
            RandomAccessIterator::Vec((Ptr::<Vec<i32>>::decay(&(v2)) as Ptr<i32>).to_end()),
        ));
        'loop_: while (*__begin2.borrow()) != (*__end2.borrow()) {
            let i: Ptr<i32> = (*__begin2.borrow()).iter_deref();
            i.with_mut(|__v| __v.prefix_inc());
            (*__begin2.borrow_mut()).iter_inc();
        }
        (*__begin1.borrow_mut()).iter_inc();
    }
    let __begin1: Value<RandomAccessIterator<Vec<i32>>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v.as_pointer() as Ptr<Vec<i32>>)),
    ));
    let __end1: Value<RandomAccessIterator<Vec<i32>>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v.as_pointer() as Ptr<Vec<i32>>).to_end()),
    ));
    'loop_: while (*__begin1.borrow()) != (*__end1.borrow()) {
        let v2: Ptr<Vec<i32>> = (*__begin1.borrow()).iter_deref();
        let __begin2: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
            RandomAccessIterator::Vec((Ptr::<Vec<i32>>::decay(&(v2)) as Ptr<i32>)),
        ));
        let __end2: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
            RandomAccessIterator::Vec((Ptr::<Vec<i32>>::decay(&(v2)) as Ptr<i32>).to_end()),
        ));
        'loop_: while (*__begin2.borrow()) != (*__end2.borrow()) {
            let mut i: i32 = ((*__begin2.borrow()).iter_deref().read());
            println!("{}", (i + 3));
            (*__begin2.borrow_mut()).iter_inc();
        }
        (*__begin1.borrow_mut()).iter_inc();
    }
    let __begin1: Value<RandomAccessIterator<Vec<i32>>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v.as_pointer() as Ptr<Vec<i32>>)),
    ));
    let __end1: Value<RandomAccessIterator<Vec<i32>>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v.as_pointer() as Ptr<Vec<i32>>).to_end()),
    ));
    'loop_: while (*__begin1.borrow()) != (*__end1.borrow()) {
        let v2: Value<Vec<i32>> = Rc::new(RefCell::new(
            (*(*__begin1.borrow()).iter_deref().upgrade().deref()).clone(),
        ));
        let __begin2: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
            RandomAccessIterator::Vec((v2.as_pointer() as Ptr<i32>)),
        ));
        let __end2: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
            RandomAccessIterator::Vec((v2.as_pointer() as Ptr<i32>).to_end()),
        ));
        'loop_: while (*__begin2.borrow()) != (*__end2.borrow()) {
            let mut i: i32 = ((*__begin2.borrow()).iter_deref().read());
            println!("{}", i);
            (*__begin2.borrow_mut()).iter_inc();
        }
        (*__begin1.borrow_mut()).iter_inc();
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
