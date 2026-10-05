extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn out_param_0(out: Option<Ptr<usize>>) {
    let mut out: Ptr<usize> = out.unwrap_or(Ptr::<usize>::null());
    if !(out).is_null() {
        out.write(4_usize);
    }
}
pub fn parse_1(mut v: i32, idx: Option<Ptr<usize>>) -> i32 {
    let mut idx: Ptr<usize> = idx.unwrap_or(Ptr::<usize>::null());
    if !(idx).is_null() {
        idx.write(3_usize);
    }
    return v;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v2: Value<usize> = Rc::new(RefCell::new(0_usize));
    ({ out_param_0(Some((v2.as_pointer()))) });
    assert!(((*v2.borrow()) == 4_usize));
    let pidx: Value<usize> = Rc::new(RefCell::new(0_usize));
    assert!((({ parse_1(1, Some((pidx.as_pointer())),) }) == 1));
    assert!(((*pidx.borrow()) == 3_usize));
    let sv: Value<usize> = Rc::new(RefCell::new(7_usize));
    let sp: Value<Ptr<usize>> = Rc::new(RefCell::new((sv.as_pointer())));
    let mut spp: Ptr<Ptr<usize>> = (sp.as_pointer());
    assert!((((spp.read()).read()) == 7_usize));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
