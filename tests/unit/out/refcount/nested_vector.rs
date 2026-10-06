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
    let v1: Value<Vec<Value<Vec<i32>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut v2: Vec<i32> = vec![1];
    (v1.as_pointer() as Ptr<Vec<Value<Vec<i32>>>>)
        .with_mut(|__v: &mut Vec<Value<Vec<i32>>>| __v.push(Rc::new(RefCell::new((v2).clone()))));
    assert!(
        (((Ptr::<Vec<i32>>::decay(&((*v1.borrow())[(*v1.borrow()).len() - 1].as_pointer()))
            as Ptr<i32>)
            .offset(0_usize as isize)
            .read())
            == 1)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
