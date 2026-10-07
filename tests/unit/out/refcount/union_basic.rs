extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, ByteRepr, DeepClone)]
#[byte_size(4)]
pub struct basic {
    #[offset(0)]
    #[byte_size(4)]
    __bytes: Value<Box<[u8]>>,
}
impl basic {
    pub fn i(&self) -> Ptr<i32> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn f(&self) -> Ptr<f32> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
}
impl Default for basic {
    fn default() -> Self {
        basic {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 4]))),
        }
    }
}
#[derive(VaArg, FnPtrArg, ByteRepr, DeepClone)]
#[byte_size(1)]
pub struct empty {
    #[offset(0)]
    #[byte_size(1)]
    __bytes: Value<Box<[u8]>>,
}
impl empty {}
impl Default for empty {
    fn default() -> Self {
        empty {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 1]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let u: Value<basic> = Rc::new(RefCell::new(<basic>::default()));
    let e: Value<empty> = Rc::new(RefCell::new(<empty>::default()));
    &(*e.borrow_mut());
    (*u.borrow_mut()).i().write(42);
    assert!((((*u.borrow()).i().read()) == 42));
    (*u.borrow_mut()).f().write(3.140000105E+0);
    assert!((((*u.borrow()).f().read()) == 3.140000105E+0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
