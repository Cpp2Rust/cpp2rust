extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<i64> = Rc::new(RefCell::new(0_i64));
    assert!(
        ((!{
            let (val, ovf) = 3_i64.overflowing_mul(7_i64);
            (a.as_pointer()).write(val);
            ovf
        } as i32)
            != 0)
    );
    assert!(((((*a.borrow()) == 21_i64) as i32) != 0));
    let b: Value<i64> = Rc::new(RefCell::new(0_i64));
    assert!({
        let (val, ovf) = 9223372036854775807_i64.overflowing_mul(2_i64);
        (b.as_pointer()).write(val);
        ovf
    });
    let c: Value<i64> = Rc::new(RefCell::new(0_i64));
    assert!(
        ((!{
            let (val, ovf) = 1000_i64.overflowing_mul(1000_i64);
            (c.as_pointer()).write(val);
            ovf
        } as i32)
            != 0)
    );
    assert!(((((*c.borrow()) == 1000000_i64) as i32) != 0));
    let d: Value<i64> = Rc::new(RefCell::new(0_i64));
    assert!({
        let (val, ovf) = 9223372036854775807_i64.overflowing_mul(2_i64);
        (d.as_pointer()).write(val);
        ovf
    });
    return 0;
}
