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
        __v if __v == 1 => {
            (*r.borrow_mut()) += 1;
        }
        __v if __v == 2 => {
            (*r.borrow_mut()) += 2;
        }
        __v if __v == 3 => {
            (*r.borrow_mut()) += 4;
        }
        __v if __v == 4 => {
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
    assert!((({ fallthrough_chain_0(1,) }) == 15));
    assert!((({ fallthrough_chain_0(2,) }) == 14));
    assert!((({ fallthrough_chain_0(3,) }) == 12));
    assert!((({ fallthrough_chain_0(4,) }) == 8));
    assert!((({ fallthrough_chain_0(99,) }) == -1_i32));
    return 0;
}
