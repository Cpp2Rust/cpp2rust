extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct StackArray {
    #[offset(0)]
    #[byte_size(24)]
    pub arr: Value<Box<[Ptr<i32>]>>,
}
impl Clone for StackArray {
    fn clone(&self) -> Self {
        Self {
            arr: Rc::new(RefCell::new((*self.arr.borrow()).clone())),
        }
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
pub fn IncrementAll_0(s: Ptr<StackArray>) {
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 3) {
        {
            let _ptr = ((array_field_ptr!(s, arr) as Ptr<Ptr<i32>>)
                .offset((*i.borrow()) as isize)
                .read())
            .clone();
            _ptr.write(_ptr.read() + 1)
        };
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn main() {
    __cpp2rust_init_globals();
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
    ({ IncrementAll_0(s.as_pointer()) });
    assert!(((*x.borrow()) == 3));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
