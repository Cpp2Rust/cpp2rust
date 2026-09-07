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
    let set: Value<CFdSet> = Rc::new(RefCell::new(Default::default()));
    (set.as_pointer()).with_mut(|__s| __s.zero());
    assert!(
        ((!(if (set.as_pointer()).with(|__s| __s.isset(3)) {
            1
        } else {
            0
        } != 0) as i32)
            != 0)
    );
    (set.as_pointer()).with_mut(|__s| __s.set(3));
    assert!(
        (if (set.as_pointer()).with(|__s| __s.isset(3)) {
            1
        } else {
            0
        } != 0)
    );
    (set.as_pointer()).with_mut(|__s| __s.clr(3));
    assert!(
        ((!(if (set.as_pointer()).with(|__s| __s.isset(3)) {
            1
        } else {
            0
        } != 0) as i32)
            != 0)
    );
    return 0;
}
