extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sm_0(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let ret: Value<i32> = <Value<i32>>::default();
    goto_block!({
        '__entry: {
            *ret.borrow_mut() = 0;
            switch!(match (*n.borrow()) {
                v if v == 0 => {
                    (*ret.borrow_mut()) += 1;
                }
                v if v == 1 => {
                    (*ret.borrow_mut()) += 10;
                    goto!('out);
                }
                _ => {
                    (*ret.borrow_mut()) += 100;
                    break;
                }
            });
            (*ret.borrow_mut()) += 1000;
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
            let _n: i32 = 0;
            sm_0(_n)
        }) == 11) as i32)
            != 0)
    );
    assert!(
        (((({
            let _n: i32 = 1;
            sm_0(_n)
        }) == 10) as i32)
            != 0)
    );
    assert!(
        (((({
            let _n: i32 = 9;
            sm_0(_n)
        }) == 1100) as i32)
            != 0)
    );
    return 0;
}
