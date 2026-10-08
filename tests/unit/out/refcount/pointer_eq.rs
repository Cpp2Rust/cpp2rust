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
    let x: Value<i32> = Rc::new(RefCell::new(5));
    let mut p1: Ptr<i32> = (x.as_pointer());
    let mut p2: Ptr<i32> = (x.as_pointer());
    assert!(({ (p1).clone() } == { (p2).clone() }));
    let y: Value<i32> = Rc::new(RefCell::new(5));
    let mut p3: Ptr<i32> = (y.as_pointer());
    assert!(({ (p1).clone() } != { (p3).clone() }));
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    let mut p: Ptr<i32> = (arr.as_pointer() as Ptr<i32>);
    assert!(({ (p).clone() } == { (arr.as_pointer() as Ptr::<i32>) }));
    assert!(({ p.offset((1) as isize) } == { ((arr.as_pointer() as Ptr<i32>).offset(1)) }));
    assert!(({ p.offset((2) as isize) } == { ((arr.as_pointer() as Ptr<i32>).offset(2)) }));
    let val: Value<i32> = Rc::new(RefCell::new(42));
    let mut orig: Ptr<i32> = (val.as_pointer());
    let mut as_bytes: Ptr<u8> = orig.reinterpret_cast::<u8>();
    let mut back: Ptr<i32> = as_bytes.reinterpret_cast::<i32>();
    assert!(({ (orig).clone() } == { (back).clone() }));
    let mut arr_bytes: Ptr<u8> = (arr.as_pointer() as Ptr<i32>).reinterpret_cast::<u8>();
    let mut arr_back: Ptr<i32> = arr_bytes.reinterpret_cast::<i32>();
    assert!(({ (arr_back).clone() } == { (arr.as_pointer() as Ptr::<i32>) }));
    assert!(({ arr_back.offset((1) as isize) } == { ((arr.as_pointer() as Ptr<i32>).offset(1)) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
