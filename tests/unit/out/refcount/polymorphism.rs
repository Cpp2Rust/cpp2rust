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
#[derive(Clone, ByteRepr, Default)]
pub struct Dog {}
#[derive(Clone, ByteRepr, Default)]
pub struct Cat {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let dog: Value<Dog> = Rc::new(RefCell::new(Dog {}));
    let animal: Value<PtrDyn<dyn Animal>> =
        Rc::new(RefCell::new((dog.as_pointer()).to_dyn::<dyn Animal>(|w| w)));
    let eat1: Value<bool> = Rc::new(RefCell::new(
        ({ (*(*animal.borrow()).upgrade().deref()).bark() }),
    ));
    let cat: Value<Cat> = Rc::new(RefCell::new(Cat {}));
    (*animal.borrow_mut()) = (cat.as_pointer()).to_dyn::<dyn Animal>(|w| w);
    let eat2: Value<bool> = Rc::new(RefCell::new(
        ({ (*(*animal.borrow()).upgrade().deref()).bark() }),
    ));
    assert!((*eat1.borrow()) && (!(*eat2.borrow())));
    return 0;
}
impl Animal for Cat {
    fn bark(&self) -> bool {
        return false;
    }
}
impl Animal for Dog {
    fn bark(&self) -> bool {
        return true;
    }
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
