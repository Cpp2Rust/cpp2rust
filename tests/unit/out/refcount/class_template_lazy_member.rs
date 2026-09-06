extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Point {
    pub x: Value<i32>,
}
impl Clone for Point {
    fn clone(&self) -> Self {
        let __this: Value<Point> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
        }));
        let this: Ptr<Point> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Point {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct Box_int_ {
    pub val: Value<i32>,
}
pub trait Box_int_Impl {
    fn twice(&self) -> i32;
}
impl Clone for Box_int_ {
    fn clone(&self) -> Self {
        let __this: Value<Box_int_> = Rc::new(RefCell::new(Self {
            val: Rc::new(RefCell::new((*self.val.borrow()))),
        }));
        let this: Ptr<Box_int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Box_int_ {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.val.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            val: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct Box_Point_ {
    pub val: Value<Point>,
}
pub trait Box_Point_Impl {
    fn get(&self) -> Point;
}
impl Clone for Box_Point_ {
    fn clone(&self) -> Self {
        let __this: Value<Box_Point_> = Rc::new(RefCell::new(Self {
            val: Rc::new(RefCell::new((*self.val.borrow()).clone())),
        }));
        let this: Ptr<Box_Point_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Box_Point_ {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.val.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            val: Rc::new(RefCell::new(<Point>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let i: Value<Box_int_> = Rc::new(RefCell::new(Box_int_ {
        val: Rc::new(RefCell::new(3)),
    }));
    assert!((({ Box_int_Impl::twice(&i.as_pointer(),) }) == 6));
    let p: Value<Box_Point_> = Rc::new(RefCell::new(Box_Point_ {
        val: Rc::new(RefCell::new(Point {
            x: Rc::new(RefCell::new(4)),
        })),
    }));
    assert!(((*({ Box_Point_Impl::get(&p.as_pointer(),) }).x.borrow()) == 4));
    return 0;
}
impl Box_Point_Impl for Ptr<Box_Point_> {
    fn get(&self) -> Point {
        return (*(*(*self).upgrade().deref()).val.borrow()).clone();
    }
}
impl Box_int_Impl for Ptr<Box_int_> {
    fn twice(&self) -> i32 {
        return ((*(*(*self).upgrade().deref()).val.borrow())
            + (*(*(*self).upgrade().deref()).val.borrow()));
    }
}
