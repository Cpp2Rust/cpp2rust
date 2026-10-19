extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn f1_0(first: RandomAccessIterator<i32>, last: RandomAccessIterator<i32>) {
    let first: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(first));
    let last: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(last));
    {
        let __n = (*last.borrow())
            .clone()
            .iter_distance(&(*first.borrow()).clone());
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            (*first.borrow()).clone().iter_at(__x).with(|__a| {
                (*first.borrow())
                    .clone()
                    .iter_at(__y)
                    .with(|__b| __a.cmp(__b))
            })
        });
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| (*first.borrow()).clone().iter_at(__i).read())
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            (*first.borrow()).clone().iter_at(__i).write(__v);
        }
    };
    return;
}
pub fn f1_1(mut first: Ptr<i32>, mut last: Ptr<i32>) {
    {
        let __n = last.iter_distance(&first);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            first
                .iter_at(__x)
                .with(|__a| first.iter_at(__y).with(|__b| __a.cmp(__b)))
        });
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| first.iter_at(__i).read())
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            first.iter_at(__i).write(__v);
        }
    };
    return;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 3, 0, 2, 8, 7])));
    ({
        let _first: RandomAccessIterator<i32> = (v.as_pointer() as Ptr<i32>).begin();
        let _last: RandomAccessIterator<i32> = (v.as_pointer() as Ptr<i32>).end();
        f1_0(_first, _last)
    });
    let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    ({
        let _first: Ptr<i32> = (a.as_pointer() as Ptr<i32>);
        let _last: Ptr<i32> = (a.as_pointer() as Ptr<i32>).offset((2) as isize);
        f1_1(_first, _last)
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
