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
#[derive(Default)]
pub struct Dog {}
impl Clone for Dog {
    fn clone(&self) -> Self {
        let mut this = Self {};
        this
    }
}
impl ByteRepr for Dog {}
impl Animal for Dog {
    fn bark(&self) -> bool {
        return true;
    }
}
#[derive(Default)]
pub struct Cat {}
impl Cat {
    fn meow(&self) -> bool {
        return true;
    }
}
impl Clone for Cat {
    fn clone(&self) -> Self {
        let mut this = Self {};
        this
    }
}
impl ByteRepr for Cat {}
impl Animal for Cat {
    fn bark(&self) -> bool {
        return false;
    }
}
pub fn main() {
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
    return (((*eat1.borrow()) && !(*eat2.borrow())) as i32);
}
