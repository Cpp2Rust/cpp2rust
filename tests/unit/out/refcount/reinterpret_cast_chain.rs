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
    let mut dwords: Ptr<u32> = (val.as_pointer()).reinterpret_cast::<u32>();
    assert!(((elem!(dwords, 0).read()) == 67305985_u32));
    assert!(((elem!(dwords, 1).read()) == 134678021_u32));
    let mut words: Ptr<u16> = dwords.reinterpret_cast::<u16>();
    assert!((((elem!(words, 0).read()) as i32) == 513));
    assert!((((elem!(words, 1).read()) as i32) == 1027));
    assert!((((elem!(words, 2).read()) as i32) == 1541));
    assert!((((elem!(words, 3).read()) as i32) == 2055));
    elem!(words, 1).write(48042_u16);
    assert!(((elem!(dwords, 0).read()) == 3148481025_u32));
    assert!(((*val.borrow()) == 578437698833482241_u64));
    elem!(dwords, 1).write(4293844428_u32);
    assert!(((*val.borrow()) == 18441921395520307713_u64));
    assert!((((elem!(words, 2).read()) as i32) == 56780));
    assert!((((elem!(words, 3).read()) as i32) == 65518));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
