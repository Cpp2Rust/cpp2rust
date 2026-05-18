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
    let val: Value<u64> = Rc::new(RefCell::new(578437695752307201_u64));
    let dwords: Value<Ptr<u32>> =
        Rc::new(RefCell::new((val.as_pointer()).reinterpret_cast::<u32>()));
    assert!((((*dwords.borrow()).offset((0) as isize).read()) == 67305985_u32));
    assert!((((*dwords.borrow()).offset((1) as isize).read()) == 134678021_u32));
    let words: Value<Ptr<u16>> = Rc::new(RefCell::new(
        ((*dwords.borrow()).reinterpret_cast::<u16>()).clone(),
    ));
    assert!(((((*words.borrow()).offset((0) as isize).read()) as i32) == 513));
    assert!(((((*words.borrow()).offset((1) as isize).read()) as i32) == 1027));
    assert!(((((*words.borrow()).offset((2) as isize).read()) as i32) == 1541));
    assert!(((((*words.borrow()).offset((3) as isize).read()) as i32) == 2055));
    (*words.borrow()).offset((1) as isize).write(48042_u16);
    assert!((((*dwords.borrow()).offset((0) as isize).read()) == 3148481025_u32));
    assert!(((*val.borrow()) == 578437698833482241_u64));
    (*dwords.borrow())
        .offset((1) as isize)
        .write(4293844428_u32);
    assert!(((*val.borrow()) == 18441921395520307713_u64));
    assert!(((((*words.borrow()).offset((2) as isize).read()) as i32) == 56780));
    assert!(((((*words.borrow()).offset((3) as isize).read()) as i32) == 65518));
    return 0;
}
