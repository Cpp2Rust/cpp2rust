extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fallthrough_default_0(x: i32, flag: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let flag: Value<i32> = Rc::new(RefCell::new(flag));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            v if v == 7 => {
                if ((*flag.borrow()) != 0) {
                    (*r.borrow_mut()) = 100;
                    break 'switch;
                };
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
    assert!(
        (({
            let _x: i32 = 7;
            let _flag: i32 = 0;
            fallthrough_default_0(_x, _flag)
        }) == 42)
    );
    assert!(
        (({
            let _x: i32 = 7;
            let _flag: i32 = 1;
            fallthrough_default_0(_x, _flag)
        }) == 100)
    );
    assert!(
        (({
            let _x: i32 = 99;
            let _flag: i32 = 0;
            fallthrough_default_0(_x, _flag)
        }) == 42)
    );
    return 0;
}
