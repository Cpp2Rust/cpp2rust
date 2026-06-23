extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone)]
pub struct basic {
    __store: libcc2rs::UnionStorage,
}
impl basic {
    pub fn i(&self) -> Ptr<i32> {
        self.__store.reinterpret(0)
    }
    pub fn f(&self) -> Ptr<f32> {
        self.__store.reinterpret(0)
    }
}
impl Default for basic {
    fn default() -> Self {
        basic {
            __store: libcc2rs::UnionStorage::new(4),
        }
    }
}
impl ByteRepr for basic {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let u: Value<basic> = <Value<basic>>::default();
    (*u.borrow_mut()).i().write(42);
    assert!((((((*u.borrow()).i().read()) == 42) as i32) != 0));
    (*u.borrow_mut()).f().write(3.140000105E+0);
    assert!((((((*u.borrow()).f().read()) == 3.140000105E+0) as i32) != 0));
    return 0;
}
