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
    let view1: Value<Ptr<u32>> =
        Rc::new(RefCell::new((val.as_pointer()).reinterpret_cast::<u32>()));
    let view2: Value<Ptr<u32>> =
        Rc::new(RefCell::new((val.as_pointer()).reinterpret_cast::<u32>()));
    (*view1.borrow()).offset((0) as isize).write(3721182122_u32);
    assert!((((*view2.borrow()).offset((0) as isize).read()) == 3721182122_u32));
    assert!(((*val.borrow()) == 578437699406183338_u64));
    (*view2.borrow()).offset((1) as isize).write(4293844428_u32);
    assert!((((*view1.borrow()).offset((1) as isize).read()) == 4293844428_u32));
    assert!(((*val.borrow()) == 18441921396093008810_u64));
    return 0;
}
