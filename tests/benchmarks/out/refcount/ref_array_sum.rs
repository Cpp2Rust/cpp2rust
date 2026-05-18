extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn initialize_0(array: Ptr<Option<Value<Box<[i32]>>>>, N: i32) {
    let N: Value<i32> = Rc::new(RefCell::new(N));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        let __rhs = (*i.borrow());
        (*array.upgrade().deref()).as_ref().unwrap().borrow_mut()
            [((*i.borrow()) as u64) as usize] = __rhs;
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn sum_1(array: Ptr<Option<Value<Box<[i32]>>>>, N: i32) -> i64 {
    let array: Value<Ptr<Option<Value<Box<[i32]>>>>> = Rc::new(RefCell::new(array));
    let N: Value<i32> = Rc::new(RefCell::new(N));
    let sum: Value<i64> = Rc::new(RefCell::new(0_i64));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        (*sum.borrow_mut()) += ((*(*array.borrow()).upgrade().deref())
            .as_ref()
            .unwrap()
            .borrow()[((*i.borrow()) as u64) as usize] as i64);
        (*i.borrow_mut()).prefix_inc();
    }
    return (*sum.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let N: Value<i32> = Rc::new(RefCell::new(100000000));
    let out: Value<i64> = Rc::new(RefCell::new(0_i64));
    let k: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*k.borrow()) < 35) {
        let array: Value<Option<Value<Box<[i32]>>>> =
            Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
                (0..((*N.borrow()) as u64))
                    .map(|_| <i32>::default())
                    .collect::<Box<[_]>>(),
            )))));
        ({
            let _array: Ptr<Option<Value<Box<[i32]>>>> = array.as_pointer();
            let _N: i32 = (*N.borrow());
            initialize_0(_array, _N)
        });
        (*out.borrow_mut()) += ({
            let _array: Ptr<Option<Value<Box<[i32]>>>> = (array.as_pointer());
            let _N: i32 = (*N.borrow());
            sum_1(_array, _N)
        });
        (*k.borrow_mut()).prefix_inc();
    }
    return ((*out.borrow()) as i32);
}
