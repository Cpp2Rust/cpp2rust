extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn basic_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            v if v == 0 => {
                (*r.borrow_mut()) = 10;
                break 'switch;
            }
            v if v == 1 => {
                (*r.borrow_mut()) = 20;
                break 'switch;
            }
            v if v == 2 => {
                (*r.borrow_mut()) = 30;
                break 'switch;
            }
            _ => {
                (*r.borrow_mut()) = 40;
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
    assert!(
        (({
            let _x: i32 = 0;
            basic_0(_x)
        }) == 10)
    );
    assert!(
        (({
            let _x: i32 = 2;
            basic_0(_x)
        }) == 30)
    );
    assert!(
        (({
            let _x: i32 = 99;
            basic_0(_x)
        }) == 40)
    );
    return 0;
}
