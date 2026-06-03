extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn stacked_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            __v if __v == 1 || __v == 2 || __v == 3 => {
                (*r.borrow_mut()) = 100;
                break 'switch;
            }
            __v if __v == 4 || __v == 5 => {
                (*r.borrow_mut()) = 200;
                break 'switch;
            }
            _ => {
                (*r.borrow_mut()) = 300;
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
            let _x: i32 = 1;
            stacked_0(_x)
        }) == 100)
    );
    assert!(
        (({
            let _x: i32 = 3;
            stacked_0(_x)
        }) == 100)
    );
    assert!(
        (({
            let _x: i32 = 5;
            stacked_0(_x)
        }) == 200)
    );
    assert!(
        (({
            let _x: i32 = 9;
            stacked_0(_x)
        }) == 300)
    );
    return 0;
}
