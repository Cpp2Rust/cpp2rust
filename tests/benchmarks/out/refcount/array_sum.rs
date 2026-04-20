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
    let N: Value<i32> = Rc::new(RefCell::new(100000000));
    let sum: Value<i64> = Rc::new(RefCell::new(0_i64));
    let k: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*k.borrow()) < 35) {
        let array: Value<Option<Value<Box<[i32]>>>> =
            Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
                (0..((*N.borrow()) as u64))
                    .map(|_| <i32>::default())
                    .collect::<Box<[_]>>(),
            )))));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*N.borrow())) {
            let __rhs = (*i.borrow());
            (*array.borrow()).as_ref().unwrap().borrow_mut()[((*i.borrow()) as u64) as usize] =
                __rhs;
            (*i.borrow_mut()).prefix_inc();
        }
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*N.borrow())) {
            (*sum.borrow_mut()) += ((*array.borrow()).as_ref().unwrap().borrow()
                [((*i.borrow()) as u64) as usize] as i64);
            (*i.borrow_mut()).prefix_inc();
        }
        (*k.borrow_mut()).prefix_inc();
    }
    return ((*sum.borrow()) as i32);
}
