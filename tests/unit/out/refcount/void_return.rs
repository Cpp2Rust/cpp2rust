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
    match ({ (*first.borrow()).clone() },) {
        (mut __tmp_0,) => {
            let __n = (*last.borrow()).clone().iter_distance(&__tmp_0);
            let mut __order: Vec<isize> = (0..__n).collect();
            __order.sort_by(|&__x, &__y| cpp_cmp(&__tmp_0.iter_at(__x), &__tmp_0.iter_at(__y)));
            reorder(__order, |__i| __tmp_0.iter_at(__i));
        }
    };
    return;
}
pub fn f1_1(mut first: Ptr<i32>, mut last: Ptr<i32>) {
    {
        let __n = last.iter_distance(&first);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| cpp_cmp(&first.iter_at(__x), &first.iter_at(__y)));
        reorder(__order, |__i| first.iter_at(__i));
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
        let _first: RandomAccessIterator<i32> = v.decay().begin();
        let _last: RandomAccessIterator<i32> = v.decay().end();
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
