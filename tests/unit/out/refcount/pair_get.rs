extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn second_of_0(p: Ptr<(Value<f64>, Value<i32>)>) -> i32 {
    return ((*p.upgrade().deref()).get_value::<1>().as_pointer().read());
}
pub fn first_of_1(p: Ptr<(Value<f64>, Value<i32>)>) -> f64 {
    return ((*p.upgrade().deref()).get_value::<0>().as_pointer().read());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p: Value<(Value<f64>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(1.5_f64)),
        Rc::new(RefCell::new(2)),
    )));
    assert!((((*p.borrow()).get_value::<0>().as_pointer().read()) == 1.5_f64));
    assert!((((*p.borrow()).get_value::<1>().as_pointer().read()) == 2));
    assert!((({ first_of_1(p.as_pointer(),) }) == 1.5_f64));
    assert!((({ second_of_0(p.as_pointer(),) }) == 2));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
