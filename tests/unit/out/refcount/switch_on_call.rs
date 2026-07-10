extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn double_it_0(v: i32) -> i32 {
    let v: Value<i32> = Rc::new(RefCell::new(v));
    return ((*v.borrow()) * 2);
}
pub fn switch_on_call_1(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    'switch: {
        let __match_cond = ({ double_it_0((*x.borrow())) });
        match __match_cond {
            __v if __v == 0 => {
                return 100;
            }
            __v if __v == 2 => {
                return 200;
            }
            __v if __v == 4 => {
                return 400;
            }
            _ => {
                return -1_i32;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ switch_on_call_1(0,) }) == 100));
    assert!((({ switch_on_call_1(1,) }) == 200));
    assert!((({ switch_on_call_1(2,) }) == 400));
    assert!((({ switch_on_call_1(99,) }) == -1_i32));
    return 0;
}
