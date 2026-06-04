extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn case_then_default_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            __v if __v == 2 => {
                (*r.borrow_mut()) = 20;
                break 'switch;
            }
            _ => {
                (*r.borrow_mut()) = 10;
                break 'switch;
            }
        }
    };
    return (*r.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ case_then_default_0(1,) }) == 10));
    assert!((({ case_then_default_0(2,) }) == 20));
    assert!((({ case_then_default_0(99,) }) == 10));
    return 0;
}
