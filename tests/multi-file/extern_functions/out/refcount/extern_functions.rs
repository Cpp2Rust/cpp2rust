extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

// a.rs
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (((({
            let _x: i32 = 42;
            helper_0(_x)
        }) == 43) as i32)
            != 0)
    );
    return 0;
}
// b.rs
pub fn unrelated1_1() -> i32 {
    return 1;
}
pub fn unrelated2_2() -> i32 {
    return 2;
}
pub fn unrelated3_3() -> i32 {
    return 3;
}
pub fn helper_4(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    ({ unrelated1_1() });
    ({ unrelated2_2() });
    ({ unrelated3_3() });
    return ((*x.borrow()) + 1);
}
