extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn f_0(list: Vec<i32>) {
    let list: Value<Vec<i32>> = Rc::new(RefCell::new(list));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let i1: Value<i32> = Rc::new(RefCell::new(3));
    let i2: Value<i32> = Rc::new(RefCell::new(<i32>::default()));
    let carr1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
    let carr2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        1,
        <i32>::default(),
        <i32>::default(),
    ])));
    let arr: Value<Vec<i32>> = Rc::new(RefCell::new(vec![1, 2, 3]));
    let vec_: Value<Vec<i32>> = Rc::new(RefCell::new(vec![1, 2, 3]));
    ({
        let _list: Vec<i32> = vec![1, 2, 3, 4];
        f_0(_list)
    });
    return 0;
}
