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
pub trait ItemImpl {
    fn foo(&self, other: Ptr<Item>);
}
impl Clone for Item {
    fn clone(&self) -> Self {
        let __this: Value<Item> = Rc::new(RefCell::new(Self {
            value: Rc::new(RefCell::new((*self.value.borrow()))),
        }));
        let this: Ptr<Item> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Item {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.value.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            value: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Ptr<Item>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..2_usize)
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
        ItemImpl::foo(&(*arr.borrow()).offset((0) as isize), _other)
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
    assert!(((*result.borrow()) == 11));
    return 0;
}
impl ItemImpl for Ptr<Item> {
    fn foo(&self, other: Ptr<Item>) {
        let other: Value<Ptr<Item>> = Rc::new(RefCell::new(other));
        (*(*(*other.borrow()).upgrade().deref()).value.borrow_mut()) = 10;
    }
}
