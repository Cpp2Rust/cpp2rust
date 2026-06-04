extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn empty_case_with_break_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(5));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            __v if __v == 1 => {
                break 'switch;
            }
            __v if __v == 2 => {
                (*r.borrow_mut()) = 2;
                break 'switch;
            }
            _ => {
                (*r.borrow_mut()) = 9;
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
    assert!((({ empty_case_with_break_0(1,) }) == 5));
    assert!((({ empty_case_with_break_0(2,) }) == 2));
    assert!((({ empty_case_with_break_0(9,) }) == 9));
    return 0;
}
