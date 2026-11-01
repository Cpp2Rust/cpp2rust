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
    let p: Value<(Value<f64>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(1.5_f64)),
        Rc::new(RefCell::new(2)),
    )));
    let mut first: f64 = ((*p.borrow()).get_value::<0>().as_pointer().read());
    let mut second: i32 = ((*p.borrow()).get_value::<1>().as_pointer().read());
    let ref_: Ptr<i32> = (*p.borrow()).get_value::<1>().as_pointer();
    ref_.write(7);
    assert!((first == 1.5_f64));
    assert!((second == 2));
    assert!(((*(*p.borrow()).1.borrow()) == 7));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
