extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn switch_complex_cond_0(p: Ptr<i32>, bias: i32) -> i32 {
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p));
    let bias: Value<i32> = Rc::new(RefCell::new(bias));
    'switch: {
        let __match_cond = {
            let _lhs = ((*p.borrow()).read());
            _lhs + (*bias.borrow())
        };
        match __match_cond {
            v if v == 0 => {
                return 1;
            }
            v if v == 5 => {
                return 2;
            }
            v if v == 10 => {
                return 3;
            }
            _ => {
                return 0;
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p_val: Value<i32> = Rc::new(RefCell::new(5));
    assert!(
        (({
            let _p: Ptr<i32> = (p_val.as_pointer());
            let _bias: i32 = 0;
            switch_complex_cond_0(_p, _bias)
        }) == 2)
    );
    assert!(
        (({
            let _p: Ptr<i32> = (p_val.as_pointer());
            let _bias: i32 = 5;
            switch_complex_cond_0(_p, _bias)
        }) == 3)
    );
    assert!(
        (({
            let _p: Ptr<i32> = (p_val.as_pointer());
            let _bias: i32 = -5_i32;
            switch_complex_cond_0(_p, _bias)
        }) == 1)
    );
    assert!(
        (({
            let _p: Ptr<i32> = (p_val.as_pointer());
            let _bias: i32 = 99;
            switch_complex_cond_0(_p, _bias)
        }) == 0)
    );
    return 0;
}
