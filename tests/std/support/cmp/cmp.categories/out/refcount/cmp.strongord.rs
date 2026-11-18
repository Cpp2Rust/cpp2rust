extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn less_than_zero_0(o: std::cmp::Ordering) -> i32 {
    let o: Value<std::cmp::Ordering> = Rc::new(RefCell::new(o));
    return ((({ (*o.borrow()) }) == std::cmp::Ordering::Less) as i32);
}
pub fn test_1() {
    let o: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(&(1), &(2))));
    let mut a: i32 = ((({ (*o.borrow()) }) == std::cmp::Ordering::Less) as i32);
    assert!((a == 1));
    assert!(({ (({ less_than_zero_0((*o.borrow()),) }) == 1) }));
    assert!((((({ (*o.borrow()) }) == std::cmp::Ordering::Less) as i32) == 1));
    let mut d: i64 = ((({ (*o.borrow()) }) == std::cmp::Ordering::Greater) as i64);
    assert!((d == 0_i64));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_1() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
