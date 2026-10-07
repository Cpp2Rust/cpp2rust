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
    (*u.borrow()).i().write(42);
    assert!((((*u.borrow()).i().read()) == 42));
    (*u.borrow()).f().write(3.140000105E+0);
    assert!((((*u.borrow()).f().read()) == 3.140000105E+0));
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new((0..4).map(|_| 0_u8).collect::<Box<[u8]>>()));
    {
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
            .to_any()
            .memset((0) as u8, ::std::mem::size_of::<[u8; 4]>() as usize);
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any()
    };
    let mut ru: Ptr<basic> = (buf.as_pointer() as Ptr<u8>).reinterpret_cast::<basic>();
    let mut pi: Ptr<i32> = ((*ru.upgrade().deref()).i()).clone();
    let mut pf: Ptr<f32> = ((*ru.upgrade().deref()).f()).clone();
    (*ru.upgrade().deref()).i().write(7);
    assert!(((pi.read()) == 7));
    pi.write(1065353216);
    assert!((((*ru.upgrade().deref()).i().read()) == 1065353216));
    assert!(((pf.read()) == 1.0E+0));
    assert!((((*buf.borrow())[(3) as usize] as i32) == 63));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
