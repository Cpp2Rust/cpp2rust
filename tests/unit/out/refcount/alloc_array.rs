extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn All_0(arr: Ptr<Option<Value<Box<[i32]>>>>, mut N: i32, mut element: i32) {
    let all: Value<Option<Value<Box<[i32]>>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
        (0..(N as usize))
            .map(|_| <i32>::default())
            .collect::<Box<[_]>>(),
    )))));
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        (*all.borrow()).as_ref().unwrap().borrow_mut()[(i as usize) as usize] = element;
        i.prefix_inc();
    }
    ((arr).clone() as Ptr<Option<Value<Box<[i32]>>>>).write((*all.borrow_mut()).take());
}
pub fn Consume_1(mut arr: Option<Value<Box<[i32]>>>, mut N: i32) -> i32 {
    let mut sum: i32 = 0;
    let mut i: i32 = -1_i32;
    'loop_: while (i.prefix_inc() < N) {
        sum += arr.as_ref().unwrap().borrow()[(i as usize) as usize];
    }
    return sum;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut N: i32 = 10;
    let arr: Value<Option<Value<Box<[i32]>>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
        (0..(N as usize))
            .map(|_| <i32>::default())
            .collect::<Box<[_]>>(),
    )))));
    ({ All_0(arr.as_pointer(), N, 1) });
    assert!((({ Consume_1((*arr.borrow_mut()).take(), N,) }) == 10));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
