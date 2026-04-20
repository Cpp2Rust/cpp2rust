extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn All_0(arr: Ptr<Option<Value<Box<[i32]>>>>, N: i32, element: i32) {
    let N: Value<i32> = Rc::new(RefCell::new(N));
    let element: Value<i32> = Rc::new(RefCell::new(element));
    let all: Value<Option<Value<Box<[i32]>>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
        (0..((*N.borrow()) as u64))
            .map(|_| <i32>::default())
            .collect::<Box<[_]>>(),
    )))));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        (*all.borrow()).as_ref().unwrap().borrow_mut()[((*i.borrow()) as u64) as usize] =
            (*element.borrow());
        (*i.borrow_mut()).prefix_inc();
    }
    let __rhs = (*all.borrow_mut()).take();
    arr.write(__rhs);
}
pub fn Consume_1(arr: Option<Value<Box<[i32]>>>, N: i32) -> i32 {
    let arr: Value<Option<Value<Box<[i32]>>>> = Rc::new(RefCell::new(arr));
    let N: Value<i32> = Rc::new(RefCell::new(N));
    let sum: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(-1_i32));
    'loop_: while ((*i.borrow_mut()).prefix_inc() < (*N.borrow())) {
        (*sum.borrow_mut()) +=
            (*arr.borrow()).as_ref().unwrap().borrow()[((*i.borrow()) as u64) as usize];
    }
    return (*sum.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let N: Value<i32> = Rc::new(RefCell::new(10));
    let arr: Value<Option<Value<Box<[i32]>>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
        (0..((*N.borrow()) as u64))
            .map(|_| <i32>::default())
            .collect::<Box<[_]>>(),
    )))));
    ({
        let _arr: Ptr<Option<Value<Box<[i32]>>>> = arr.as_pointer();
        let _N: i32 = (*N.borrow());
        let _element: i32 = 1;
        All_0(_arr, _N, _element)
    });
    return ({
        let _arr: Option<Value<Box<[i32]>>> = (*arr.borrow_mut()).take();
        let _N: i32 = (*N.borrow());
        Consume_1(_arr, _N)
    });
}
