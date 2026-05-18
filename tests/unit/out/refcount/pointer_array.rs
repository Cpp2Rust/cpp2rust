extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub struct StackArray {
    pub arr: Value<Box<[Ptr<i32>]>>,
}
impl Clone for StackArray {
    fn clone(&self) -> Self {
        let mut this = Self {
            arr: Rc::new(RefCell::new((*self.arr.borrow()).clone())),
        };
        this
    }
}
impl Default for StackArray {
    fn default() -> Self {
        StackArray {
            arr: Rc::new(RefCell::new(
                (0..3)
                    .map(|_| Ptr::<i32>::null())
                    .collect::<Box<[Ptr<i32>]>>(),
            )),
        }
    }
}
impl ByteRepr for StackArray {}
pub fn IncrementAll_0(s: Ptr<StackArray>) {
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 3) {
        {
            let __ptr = (*(*s.upgrade().deref()).arr.borrow())[(*i.borrow()) as usize].clone();
            let __tmp = __ptr.read() + 1;
            __ptr.write(__tmp)
        };
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(0));
    let s: Value<StackArray> = Rc::new(RefCell::new(StackArray {
        arr: Rc::new(RefCell::new(Box::new([
            (x.as_pointer()),
            (x.as_pointer()),
            (x.as_pointer()),
        ]))),
    }));
    ({
        let _s: Ptr<StackArray> = s.as_pointer();
        IncrementAll_0(_s)
    });
    return (*x.borrow());
}
