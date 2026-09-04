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
        let mut this = Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
        };
        this
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
impl Box_int_ {
    pub fn twice(&self) -> i32 {
        return ((*self.val.borrow()) + (*self.val.borrow()));
    }
}
impl Clone for Box_int_ {
    fn clone(&self) -> Self {
        let mut this = Self {
            val: Rc::new(RefCell::new((*self.val.borrow()))),
        };
        this
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
impl Box_Point_ {
    pub fn get(&self) -> Point {
        return (*self.val.borrow()).clone();
    }
}
impl Clone for Box_Point_ {
    fn clone(&self) -> Self {
        let mut this = Self {
            val: Rc::new(RefCell::new((*self.val.borrow()).clone())),
        };
        this
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
    assert!((({ (*i.borrow()).twice() }) == 6));
    let p: Value<Box_Point_> = Rc::new(RefCell::new(Box_Point_ {
        val: Rc::new(RefCell::new(Point {
            x: Rc::new(RefCell::new(4)),
        })),
    }));
    assert!(((*({ (*p.borrow()).get() }).x.borrow()) == 4));
    return 0;
}
