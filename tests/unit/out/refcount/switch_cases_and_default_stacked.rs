extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn cases_and_default_stacked_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            __v if __v == 3 => {
                (*r.borrow_mut()) = 3;
                break 'switch;
            }
            _ => {
                (*r.borrow_mut()) = 42;
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
    assert!((({ cases_and_default_stacked_0(1,) }) == 42));
    assert!((({ cases_and_default_stacked_0(2,) }) == 42));
    assert!((({ cases_and_default_stacked_0(3,) }) == 3));
    assert!((({ cases_and_default_stacked_0(99,) }) == 42));
    return 0;
}
