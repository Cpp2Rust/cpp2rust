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
    let mut view1: Ptr<u32> = (val.as_pointer()).reinterpret_cast::<u32>();
    let mut view2: Ptr<u32> = (val.as_pointer()).reinterpret_cast::<u32>();
    elem!(view1, 0).write(3721182122_u32);
    assert!(((elem!(view2, 0).read()) == 3721182122_u32));
    assert!(((*val.borrow()) == 578437699406183338_u64));
    elem!(view2, 1).write(4293844428_u32);
    assert!(((elem!(view1, 1).read()) == 4293844428_u32));
    assert!(((*val.borrow()) == 18441921396093008810_u64));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
