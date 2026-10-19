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
    'loop_: for e1 in IterRange::new(
        RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>)),
        RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>).to_end()),
    ) {
        let mut e1: i32 = (e1.iter_deref().read());
        'loop_: for e2 in IterRange::new(
            RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>)),
            RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>).to_end()),
        ) {
            let mut e2: i32 = (e2.iter_deref().read());
            square += (e1 * e2);
        }
    }
    'loop_: for e1 in IterRange::new(
        RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>)),
        RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>).to_end()),
    ) {
        let e1: Ptr<i32> = e1.iter_deref();
        'loop_: for e2 in IterRange::new(
            RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>)),
            RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>).to_end()),
        ) {
            let e2: Ptr<i32> = e2.iter_deref();
            square += { ({ (e1.read()) } * { (e2.read()) }) };
        }
    }
    'loop_: for e1 in IterRange::new(
        RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>)),
        RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>).to_end()),
    ) {
        let e1: Ptr<i32> = e1.iter_deref();
        'loop_: for e2 in IterRange::new(
            RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>)),
            RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>).to_end()),
        ) {
            let e2: Ptr<i32> = e2.iter_deref();
            square += { ({ (e1.read()) } * { (e2.read()) }) };
        }
    }
    'loop_: for e1 in IterRange::new(
        RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>)),
        RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>).to_end()),
    ) {
        let e1: Ptr<i32> = e1.iter_deref();
        'loop_: for e2 in IterRange::new(
            RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>)),
            RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>).to_end()),
        ) {
            let e2: Ptr<i32> = e2.iter_deref();
            square += { ({ (e1.read()) } * { (e2.read()) }) };
        }
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
    'loop_: for row in IterRange::new(
        RandomAccessIterator::Vec((m.as_pointer() as Ptr<Vec<i32>>)),
        RandomAccessIterator::Vec((m.as_pointer() as Ptr<Vec<i32>>).to_end()),
    ) {
        let row: Ptr<Vec<i32>> = row.iter_deref();
        'loop_: for col in IterRange::new(
            RandomAccessIterator::Vec((Ptr::<Vec<i32>>::decay(&(row)) as Ptr<i32>)),
            RandomAccessIterator::Vec((Ptr::<Vec<i32>>::decay(&(row)) as Ptr<i32>).to_end()),
        ) {
            let col: Ptr<i32> = col.iter_deref();
            square += { (col.read()) };
        }
    }
    assert!((square == 144));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
