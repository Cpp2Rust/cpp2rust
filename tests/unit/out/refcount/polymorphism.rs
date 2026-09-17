extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub trait Animal {
    fn bark(&self) -> bool;
}
#[derive(Clone, Default)]
pub struct Dog {}
impl Animal for Dog {
    fn bark(&self) -> bool {
        return true;
    }
}
impl ByteRepr for Dog {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Clone, Default)]
pub struct Cat {}
impl Animal for Cat {
    fn bark(&self) -> bool {
        return false;
    }
}
impl ByteRepr for Cat {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let dog: Value<Dog> = Rc::new(RefCell::new(<Dog>::default()));
    let animal: Value<PtrDyn<dyn Animal>> = Rc::new(RefCell::new(
        ((dog.as_pointer()).to_strong() as Value<dyn Animal>).as_pointer_dyn(),
    ));
    let eat1: Value<bool> = Rc::new(RefCell::new(
        ({ (*(*animal.borrow()).upgrade().deref()).bark() }),
    ));
    let cat: Value<Cat> = Rc::new(RefCell::new(<Cat>::default()));
    (*animal.borrow_mut()) = ((cat.as_pointer()).to_strong() as Value<dyn Animal>).as_pointer_dyn();
    let eat2: Value<bool> = Rc::new(RefCell::new(
        ({ (*(*animal.borrow()).upgrade().deref()).bark() }),
    ));
    assert!((*eat1.borrow()) && (!(*eat2.borrow())));
    return 0;
}
pub trait CatImpl {
    fn meow(&self) -> bool;
}
impl CatImpl for Ptr<Cat> {
    fn meow(&self) -> bool {
        return true;
    }
}
pub fn __cpp2rust_init_globals() {}
