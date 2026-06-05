extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sm_0(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let steps: Value<i32> = Rc::new(RefCell::new(0));
    switch!(match (*n.borrow()) {
        __v if __v == 0 => 'target: {
            (*steps.borrow_mut()) += 1;
            break;
        }
        __v if __v == 1 => {
            (*steps.borrow_mut()) += 10;
            goto!('target);
        }
        _ => {
            (*steps.borrow_mut()) = -1_i32;
            break;
        }
    });
    return (*steps.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((((({ sm_0(0,) }) == 1) as i32) != 0));
    assert!((((({ sm_0(1,) }) == 11) as i32) != 0));
    assert!((((({ sm_0(7,) }) == -1_i32) as i32) != 0));
    return 0;
}
