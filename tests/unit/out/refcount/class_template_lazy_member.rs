extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Point {
    #[offset(0)]
    pub x: i32,
}
impl ByteRepr for Point {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Box_int_ {
    #[offset(0)]
    pub val: i32,
}
impl ByteRepr for Box_int_ {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.val.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            val: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Box_Point_ {
    #[offset(0)]
    pub val: Point,
}
impl ByteRepr for Box_Point_ {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.val.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            val: <Point>::from_bytes(&buf[0..4]),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let i: Value<Box_int_> = Rc::new(RefCell::new(Box_int_ { val: 3 }));
    assert!((({ Box_int_Impl::twice(&i.as_pointer(),) }) == 6));
    let p: Value<Box_Point_> = Rc::new(RefCell::new(Box_Point_ {
        val: Point { x: 4 },
    }));
    assert!(({ ({ Box_Point_Impl::get(&p.as_pointer(),) }).x } == 4));
    return 0;
}
pub trait Box_Point_Impl {
    fn get(&self) -> Point;
    fn twice(&self) -> Point {
        unimplemented!()
    }
}
impl Box_Point_Impl for Ptr<Box_Point_> {
    fn get(&self) -> Point {
        return ((*(*self).upgrade().deref()).val).clone();
    }
}
pub trait Box_int_Impl {
    fn get(&self) -> i32 {
        unimplemented!()
    }
    fn twice(&self) -> i32;
}
impl Box_int_Impl for Ptr<Box_int_> {
    fn twice(&self) -> i32 {
        return ((*self).with(|__s: &Box_int_| __s.val) + (*self).with(|__s: &Box_int_| __s.val));
    }
}
pub fn __cpp2rust_init_globals() {}
