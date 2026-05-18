extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fib_0(n: Ptr<u64>) {
    if ((n.read()) == 0_u64) || ((n.read()) == 1_u64) {
        return;
    }
    let n_1: Value<u64> = Rc::new(RefCell::new((n.read()).wrapping_sub(1_u64)));
    let n_2: Value<u64> = Rc::new(RefCell::new((n.read()).wrapping_sub(2_u64)));
    ({
        let _n: Ptr<u64> = n_1.as_pointer();
        fib_0(_n)
    });
    ({
        let _n: Ptr<u64> = n_2.as_pointer();
        fib_0(_n)
    });
    let __rhs = (*n_1.borrow()).wrapping_add((*n_2.borrow()));
    n.write(__rhs);
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let n: Value<u64> = Rc::new(RefCell::new(45_u64));
    ({
        let _n: Ptr<u64> = n.as_pointer();
        fib_0(_n)
    });
    return ((*n.borrow()) as i32);
}
