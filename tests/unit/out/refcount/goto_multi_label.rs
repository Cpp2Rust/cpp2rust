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
            if ((((*n.borrow()) < 0) as i32) != 0) {
                goto!('error);
            }
            if ((((*n.borrow()) == 0) as i32) != 0) {
                goto!('out);
            }
            (*ret.borrow_mut()) = (*n.borrow());
            goto!('out);
        }
        'error: {
            (*ret.borrow_mut()) = -1_i32;
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
            let _n: i32 = 5;
            classify_0(_n)
        }) == 5) as i32)
            != 0)
    );
    assert!(
        (((({
            let _n: i32 = 0;
            classify_0(_n)
        }) == 0) as i32)
            != 0)
    );
    assert!(
        (((({
            let _n: i32 = -2_i32;
            classify_0(_n)
        }) == -1_i32) as i32)
            != 0)
    );
    return 0;
}
