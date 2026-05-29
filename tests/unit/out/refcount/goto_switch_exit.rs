extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn classify_0(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let mut ret: Value<i32> = <Value<i32>>::default();
    goto_block!({
        '__entry: {
            ret = Rc::new(RefCell::new(0));
            'switch: {
                let __match_cond = (*n.borrow());
                match __match_cond {
                    v if v == 1 => {
                        (*ret.borrow_mut()) = 10;
                        goto!('out);
                    }
                    v if v == 2 => {
                        (*ret.borrow_mut()) = 20;
                        break 'switch;
                    }
                    _ => {
                        (*ret.borrow_mut()) = 30;
                        break 'switch;
                    }
                }
            };
            (*ret.borrow_mut()) += 1;
        }
        'out: {
            return (*ret.borrow());
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (((({
            let _n: i32 = 1;
            classify_0(_n)
        }) == 10) as i32)
            != 0)
    );
    assert!(
        (((({
            let _n: i32 = 2;
            classify_0(_n)
        }) == 21) as i32)
            != 0)
    );
    assert!(
        (((({
            let _n: i32 = 9;
            classify_0(_n)
        }) == 31) as i32)
            != 0)
    );
    return 0;
}
