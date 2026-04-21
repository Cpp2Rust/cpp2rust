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
pub fn stacked_1(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            v if v == 1 || v == 2 || v == 3 => {
                (*r.borrow_mut()) = 100;
                break 'switch;
            }
            v if v == 4 || v == 5 => {
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
pub fn no_default_2(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(-1_i32));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            v if v == 7 => {
                (*r.borrow_mut()) = 1;
                break 'switch;
            }
            v if v == 8 => {
                (*r.borrow_mut()) = 2;
                break 'switch;
            }
            _ => {}
        }
    };
    return (*r.borrow());
}
pub fn only_default_3(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            _ => {
                (*r.borrow_mut()) = 42;
                break 'switch;
            }
        }
    };
    return (*r.borrow());
}
pub fn default_middle_4(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let r: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = (*x.borrow());
        match __match_cond {
            v if v == 1 => {
                (*r.borrow_mut()) = 1;
                break 'switch;
            }
            v if v == 2 => {
                (*r.borrow_mut()) = 2;
                break 'switch;
            }
            _ => {
                (*r.borrow_mut()) = 99;
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
    assert!(
        (({
            let _x: i32 = 1;
            stacked_1(_x)
        }) == 100)
    );
    assert!(
        (({
            let _x: i32 = 3;
            stacked_1(_x)
        }) == 100)
    );
    assert!(
        (({
            let _x: i32 = 5;
            stacked_1(_x)
        }) == 200)
    );
    assert!(
        (({
            let _x: i32 = 9;
            stacked_1(_x)
        }) == 300)
    );
    assert!(
        (({
            let _x: i32 = 7;
            no_default_2(_x)
        }) == 1)
    );
    assert!(
        (({
            let _x: i32 = 42;
            no_default_2(_x)
        }) == -1_i32)
    );
    assert!(
        (({
            let _x: i32 = 1;
            only_default_3(_x)
        }) == 42)
    );
    assert!(
        (({
            let _x: i32 = 1;
            default_middle_4(_x)
        }) == 1)
    );
    assert!(
        (({
            let _x: i32 = 2;
            default_middle_4(_x)
        }) == 2)
    );
    assert!(
        (({
            let _x: i32 = 99;
            default_middle_4(_x)
        }) == 99)
    );
    return 0;
}
