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
        let __match_cond = ({
            let _v: i32 = (*x.borrow());
            double_it_0(_v)
        });
        match __match_cond {
            v if v == 0 => {
                return 100;
            }
            v if v == 2 => {
                return 200;
            }
            v if v == 4 => {
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
    assert!(
        (({
            let _x: i32 = 0;
            switch_on_call_1(_x)
        }) == 100)
    );
    assert!(
        (({
            let _x: i32 = 1;
            switch_on_call_1(_x)
        }) == 200)
    );
    assert!(
        (({
            let _x: i32 = 2;
            switch_on_call_1(_x)
        }) == 400)
    );
    assert!(
        (({
            let _x: i32 = 99;
            switch_on_call_1(_x)
        }) == -1_i32)
    );
    return 0;
}
