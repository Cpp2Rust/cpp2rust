extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let negate: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |x: i32| -> i32 {
            return -x;
        },
        |x: f64| -> f64 {
            return -x;
        }
    )));
    let mut fi: FnPtr<fn(i32) -> i32> = (*negate.borrow()).clone().spec::<fn(i32) -> i32>(0);
    let mut fd: FnPtr<fn(f64) -> f64> = (*negate.borrow()).clone().spec::<fn(f64) -> f64>(1);
    assert!((({ fi.call(3,) }) == -3_i32));
    assert!((({ fd.call(1.5_f64,) }) == -1.5_f64));
    let square: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |x: i32| -> i32 {
            return (x * x);
        },
        |x: f64| -> f64 {
            return (x * x);
        }
    )));
    let mut si: FnPtr<fn(i32) -> i32> = (*square.borrow()).clone().spec::<fn(i32) -> i32>(0);
    let mut sd: FnPtr<fn(f64) -> f64> = (*square.borrow()).clone().spec::<fn(f64) -> f64>(1);
    assert!((({ si.call(3,) }) == 9));
    assert!((({ sd.call(1.5_f64,) }) == 2.25_f64));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
