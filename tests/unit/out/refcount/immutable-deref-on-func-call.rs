extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Item {
    #[offset(0)]
    pub value: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Ptr<Item>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..2_usize)
            .map(|_| <Item>::default())
            .collect::<Box<[Item]>>(),
    )));
    field!((*arr.borrow()).offset((0) as isize), value).write(1);
    field!((*arr.borrow()).offset((1) as isize), value).write(2);
    ({
        let _other: Ptr<Item> = ((*arr.borrow()).offset((1) as isize));
        ItemImpl::foo(&(*arr.borrow()).offset((0) as isize), _other)
    });
    let result: Value<i32> = Rc::new(RefCell::new(
        ({ (*(*arr.borrow()).offset((0) as isize).upgrade().deref()).value } + {
            (*(*arr.borrow()).offset((1) as isize).upgrade().deref()).value
        }),
    ));
    (*arr.borrow()).delete();
    assert!(((*result.borrow()) == 11));
    return 0;
}
pub trait ItemImpl {
    fn foo(&self, other: Ptr<Item>);
}
impl ItemImpl for Ptr<Item> {
    fn foo(&self, other: Ptr<Item>) {
        let other: Value<Ptr<Item>> = Rc::new(RefCell::new(other));
        field!((*other.borrow()), value).write(10);
    }
}
pub fn __cpp2rust_init_globals() {}
