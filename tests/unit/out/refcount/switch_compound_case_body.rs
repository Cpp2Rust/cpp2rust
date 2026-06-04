extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn compound_case_body_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            __v if __v == 1 => {
                let y: Value<i32> = Rc::new(RefCell::new(10));
                let z: Value<i32> = Rc::new(RefCell::new(20));
                (*r.borrow_mut()) = ((*y.borrow()) + (*z.borrow()));
                break 'switch;
            }
            __v if __v == 2 => {
                let y: Value<i32> = Rc::new(RefCell::new(100));
                (*r.borrow_mut()) = ((*y.borrow()) - 1);
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
    assert!((({ compound_case_body_0(1,) }) == 30));
    assert!((({ compound_case_body_0(2,) }) == 99));
    assert!((({ compound_case_body_0(9,) }) == -1_i32));
    return 0;
}
