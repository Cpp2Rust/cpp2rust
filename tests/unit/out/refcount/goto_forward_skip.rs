extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn skip_0(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let mut x: Value<i32> = <Value<i32>>::default();
    goto_block!({
        '__entry: {
            x = Rc::new(RefCell::new(0));
            if ((((*n.borrow()) > 0) as i32) != 0) {
                goto!('mid);
            }
            (*x.borrow_mut()) += 10;
        }
        'mid: {
            (*x.borrow_mut()) += 1;
            return (*x.borrow());
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
            skip_0(_n)
        }) == 1) as i32)
            != 0)
    );
    assert!(
        (((({
            let _n: i32 = -1_i32;
            skip_0(_n)
        }) == 11) as i32)
            != 0)
    );
    return 0;
}
