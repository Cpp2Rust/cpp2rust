extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fallthrough_chain_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    switch!(match (*x.borrow()) {
        v if v == 1 => {
            (*r.borrow_mut()) += 1;
        }
        v if v == 2 => {
            (*r.borrow_mut()) += 2;
        }
        v if v == 3 => {
            (*r.borrow_mut()) += 4;
        }
        v if v == 4 => {
            (*r.borrow_mut()) += 8;
            break;
        }
        _ => {
            (*r.borrow_mut()) = -1_i32;
            break;
        }
    });
    return (*r.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (({
            let _x: i32 = 1;
            fallthrough_chain_0(_x)
        }) == 15)
    );
    assert!(
        (({
            let _x: i32 = 2;
            fallthrough_chain_0(_x)
        }) == 14)
    );
    assert!(
        (({
            let _x: i32 = 3;
            fallthrough_chain_0(_x)
        }) == 12)
    );
    assert!(
        (({
            let _x: i32 = 4;
            fallthrough_chain_0(_x)
        }) == 8)
    );
    assert!(
        (({
            let _x: i32 = 99;
            fallthrough_chain_0(_x)
        }) == -1_i32)
    );
    return 0;
}
