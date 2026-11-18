extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    let l: Ptr<std::cmp::Ordering> = libcc2rs::strong_ordering_less();
    let e: Ptr<std::cmp::Ordering> = libcc2rs::strong_ordering_equal();
    let g: Ptr<std::cmp::Ordering> = libcc2rs::strong_ordering_greater();
    assert!(({ (l).clone() } == { (libcc2rs::strong_ordering_less()) }));
    assert!(({ (e).clone() } == { (libcc2rs::strong_ordering_equal()) }));
    assert!(({ (g).clone() } == { (libcc2rs::strong_ordering_greater()) }));
    assert!(({ (*l.upgrade().deref()) }) == std::cmp::Ordering::Less);
    assert!(({ (*e.upgrade().deref()) }) == std::cmp::Ordering::Equal);
    assert!(({ (*g.upgrade().deref()) }) == std::cmp::Ordering::Greater);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
