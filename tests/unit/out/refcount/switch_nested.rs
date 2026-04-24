extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn nested_0(a: i32, b: i32) -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    let b: Value<i32> = Rc::new(RefCell::new(b));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = (*a.borrow());
        match __match_cond {
            v if v == 1 => {
                'switch: {
                    let __match_cond = (*b.borrow());
                    match __match_cond {
                        v if v == 10 => {
                            (*r.borrow_mut()) = 11;
                            break 'switch;
                        }
                        v if v == 20 => {
                            (*r.borrow_mut()) = 12;
                            break 'switch;
                        }
                        _ => {
                            (*r.borrow_mut()) = 13;
                            break 'switch;
                        }
                    }
                };
                (*r.borrow_mut()) += 1;
                break 'switch;
            }
            v if v == 2 => {
                (*r.borrow_mut()) = 2;
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
            let _a: i32 = 1;
            let _b: i32 = 10;
            nested_0(_a, _b)
        }) == 12)
    );
    assert!(
        (({
            let _a: i32 = 1;
            let _b: i32 = 99;
            nested_0(_a, _b)
        }) == 14)
    );
    assert!(
        (({
            let _a: i32 = 2;
            let _b: i32 = 0;
            nested_0(_a, _b)
        }) == 2)
    );
    assert!(
        (({
            let _a: i32 = 3;
            let _b: i32 = 3;
            nested_0(_a, _b)
        }) == -1_i32)
    );
    return 0;
}
