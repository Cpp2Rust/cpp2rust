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
    let arr1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([5, 2, 8, 1, 3])));
    {
        let __cmp = FnPtr::<fn(i32, i32) -> bool>::new(|mut x: i32, mut y: i32| -> bool {
            {
                return (x < y);
            }
        });
        let __n = (arr1.as_pointer() as Ptr<i32>)
            .offset((5) as isize)
            .iter_distance(&(arr1.as_pointer() as Ptr<i32>));
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            let __a = (arr1.as_pointer() as Ptr<i32>).iter_at(__x);
            let __b = (arr1.as_pointer() as Ptr<i32>).iter_at(__y);
            if __cmp.call(__a.read(), __b.read()) {
                std::cmp::Ordering::Less
            } else if __cmp.call(__b.read(), __a.read()) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        reorder(__order, |__i| (arr1.as_pointer() as Ptr<i32>).iter_at(__i));
    };
    return 0;
}
pub fn __cpp2rust_init_globals() {}
