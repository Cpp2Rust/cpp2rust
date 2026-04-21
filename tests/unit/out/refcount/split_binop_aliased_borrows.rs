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
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(vec![1, 2]));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((v.as_pointer() as Ptr<i32>)));
    let r: Ptr<i32> = (v.as_pointer() as Ptr<i32>).offset(1_u64 as isize);
    let __rhs = (r.read());
    (*p.borrow()).write(__rhs);
    return ((v.as_pointer() as Ptr<i32>).offset(0_u64 as isize).read());
}
