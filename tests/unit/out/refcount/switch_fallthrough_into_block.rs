extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fallthrough_into_block_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            v if v == 1 => {
                (*r.borrow_mut()) += 1;
            }
            v if v == 2 => {
                let tmp: Value<i32> = Rc::new(RefCell::new(((*r.borrow()) * 10)));
                (*r.borrow_mut()) = ((*tmp.borrow()) + 5);
                break 'switch;
            }
            _ => {
                (*r.borrow_mut()) = -1_i32;
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
            fallthrough_into_block_0(_x)
        }) == 15)
    );
    assert!(
        (({
            let _x: i32 = 2;
            fallthrough_into_block_0(_x)
        }) == 5)
    );
    assert!(
        (({
            let _x: i32 = 99;
            fallthrough_into_block_0(_x)
        }) == -1_i32)
    );
    return 0;
}
