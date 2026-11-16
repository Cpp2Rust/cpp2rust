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
    let mut amount: i32 = 6;
    let total: Value<i32> = Rc::new(RefCell::new(0));
    let add_doubled: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let total: Ptr<i32> = total.as_pointer();
        },
        |mut amount: i32| -> i32 {
            amount *= 2;
            {
                let __rhs = amount;
                total.with_mut(|__v| *__v = *__v + __rhs)
            };
            return amount;
        }
    )));
    assert!(({ (({ (*add_doubled.borrow()).call(4,) }) == 8) }));
    assert!(({ ((*total.borrow()) == 8) }));
    assert!((amount == 6));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
