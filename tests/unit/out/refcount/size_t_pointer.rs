extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn out_param_0(out: Option<Ptr<usize>>) {
    let out: Value<Ptr<usize>> = Rc::new(RefCell::new(out.unwrap_or(Ptr::<usize>::null())));
    if !(*out.borrow()).is_null() {
        (*out.borrow()).write(4_usize);
    }
}
pub fn parse_1(v: i32, idx: Option<Ptr<usize>>) -> i32 {
    let v: Value<i32> = Rc::new(RefCell::new(v));
    let idx: Value<Ptr<usize>> = Rc::new(RefCell::new(idx.unwrap_or(Ptr::<usize>::null())));
    if !(*idx.borrow()).is_null() {
        (*idx.borrow()).write(3_usize);
    }
    return (*v.borrow());
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
    let spp: Value<Ptr<Ptr<usize>>> = Rc::new(RefCell::new((sp.as_pointer())));
    assert!(((((*spp.borrow()).read()).read()) == 7_usize));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
