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
    let val: Value<u64> = Rc::new(RefCell::new(578437695752307201_u64));
    let dwords: Value<Ptr<u32>> =
        Rc::new(RefCell::new((val.as_pointer()).reinterpret_cast::<u32>()));
    assert!(((elem!((*dwords.borrow()), 0).read()) == 67305985_u32));
    assert!(((elem!((*dwords.borrow()), 1).read()) == 134678021_u32));
    let words: Value<Ptr<u16>> =
        Rc::new(RefCell::new((*dwords.borrow()).reinterpret_cast::<u16>()));
    assert!((((elem!((*words.borrow()), 0).read()) as i32) == 513));
    assert!((((elem!((*words.borrow()), 1).read()) as i32) == 1027));
    assert!((((elem!((*words.borrow()), 2).read()) as i32) == 1541));
    assert!((((elem!((*words.borrow()), 3).read()) as i32) == 2055));
    elem!((*words.borrow()), 1).write(48042_u16);
    assert!(((elem!((*dwords.borrow()), 0).read()) == 3148481025_u32));
    assert!(((*val.borrow()) == 578437698833482241_u64));
    elem!((*dwords.borrow()), 1).write(4293844428_u32);
    assert!(((*val.borrow()) == 18441921395520307713_u64));
    assert!((((elem!((*words.borrow()), 2).read()) as i32) == 56780));
    assert!((((elem!((*words.borrow()), 3).read()) as i32) == 65518));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
