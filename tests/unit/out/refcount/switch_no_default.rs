extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn no_default_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(-1_i32));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            __v if __v == 7 => {
                (*r.borrow_mut()) = 1;
                break 'switch;
            }
            __v if __v == 8 => {
                (*r.borrow_mut()) = 2;
                break 'switch;
            }
            _ => {}
        }
    };
    return (*r.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ no_default_0(7,) }) == 1));
    assert!((({ no_default_0(42,) }) == -1_i32));
    return 0;
}
