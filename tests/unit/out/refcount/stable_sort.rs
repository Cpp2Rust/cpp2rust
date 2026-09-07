extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([5, 2, 8, 1, 3])));
    {
        let fun = |x: Ptr<i32>, y: Ptr<i32>| {
            (|x: i32, y: i32| {
                let x: Value<i32> = Rc::new(RefCell::new(x));
                let y: Value<i32> = Rc::new(RefCell::new(y));
                return ((*x.borrow()) < (*y.borrow()));
            })((x.read()).clone(), (y.read()).clone())
        };
        (arr1.as_pointer() as Ptr<i32>).sort_with_cmp(
            (arr1.as_pointer() as Ptr<i32>)
                .offset((5) as isize)
                .get_offset(),
            fun,
        )
    };
    return 0;
}
