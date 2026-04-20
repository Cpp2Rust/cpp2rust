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
    let v: Value<Vec<Value<Vec<i32>>>> = Rc::new(RefCell::new(Vec::new()));
    (v.as_pointer() as Ptr<Vec<Value<Vec<i32>>>>).with_mut(|__v: &mut Vec<Value<Vec<i32>>>| {
        __v.push(Rc::new(RefCell::new(
            (0..(10_u64) as usize)
                .map(|_| <i32>::default())
                .collect::<Vec<_>>()
                .clone(),
        )))
    });
    return 0;
}
