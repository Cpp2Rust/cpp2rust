extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Item {
    pub value: Value<i32>,
}
impl Item {
    pub fn foo(&self, other: Ptr<Item>) {
        let other: Value<Ptr<Item>> = Rc::new(RefCell::new(other));
        (*(*(*other.borrow()).upgrade().deref()).value.borrow_mut()) = 10;
    }
}
impl Clone for Item {
    fn clone(&self) -> Self {
        let mut this = Self {
            value: Rc::new(RefCell::new((*self.value.borrow()))),
        };
        this
    }
}
impl ByteRepr for Item {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Ptr<Item>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..2_u64)
            .map(|_| <Item>::default())
            .collect::<Box<[Item]>>(),
    )));
    (*(*(*arr.borrow()).offset((0) as isize).upgrade().deref())
        .value
        .borrow_mut()) = 1;
    (*(*(*arr.borrow()).offset((1) as isize).upgrade().deref())
        .value
        .borrow_mut()) = 2;
    ({
        let _other: Ptr<Item> = ((*arr.borrow()).offset((1) as isize));
        (*(*arr.borrow()).offset((0) as isize).upgrade().deref()).foo(_other)
    });
    let result: Value<i32> = Rc::new(RefCell::new(
        ((*(*(*arr.borrow()).offset((0) as isize).upgrade().deref())
            .value
            .borrow())
            + (*(*(*arr.borrow()).offset((1) as isize).upgrade().deref())
                .value
                .borrow())),
    ));
    (*arr.borrow()).delete_array();
    return (*result.borrow());
}
