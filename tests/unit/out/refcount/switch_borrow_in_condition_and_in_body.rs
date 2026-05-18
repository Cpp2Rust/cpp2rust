extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn borrow_in_condition_and_in_body_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    switch!(match (*x.borrow()) {
        v if v == 0 => {}
        _ => {
            return ((*x.borrow()) + 1);
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (({
            let _x: i32 = 0;
            borrow_in_condition_and_in_body_0(_x)
        }) == 1)
    );
    assert!(
        (({
            let _x: i32 = 1;
            borrow_in_condition_and_in_body_0(_x)
        }) == 2)
    );
    return 0;
}
