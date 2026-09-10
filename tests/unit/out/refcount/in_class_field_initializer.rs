extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub struct Inner {
    pub x: Value<i32>,
    pub y: Value<i32>,
}
impl Clone for Inner {
    fn clone(&self) -> Self {
        let mut this = Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
            y: Rc::new(RefCell::new((*self.y.borrow()))),
        };
        this
    }
}
impl Default for Inner {
    fn default() -> Self {
        Inner {
            x: Rc::new(RefCell::new(3)),
            y: Rc::new(RefCell::new(4)),
        }
    }
}
impl ByteRepr for Inner {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
        (*self.y.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            y: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
#[derive()]
pub struct S {
    pub a: Value<i32>,
    pub b: Value<u8>,
    pub c: Value<Inner>,
    pub d: Value<Inner>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        let mut this = Self {
            a: Rc::new(RefCell::new((*self.a.borrow()))),
            b: Rc::new(RefCell::new((*self.b.borrow()))),
            c: Rc::new(RefCell::new((*self.c.borrow()).clone())),
            d: Rc::new(RefCell::new((*self.d.borrow()).clone())),
        };
        this
    }
}
impl Default for S {
    fn default() -> Self {
        S {
            a: Rc::new(RefCell::new(1)),
            b: Rc::new(RefCell::new(2_u8)),
            c: Rc::new(RefCell::new(Inner {
                x: Rc::new(RefCell::new(3)),
                y: Rc::new(RefCell::new(4)),
            })),
            d: <Value<Inner>>::default(),
        }
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..4]);
        (*self.b.borrow()).to_bytes(&mut buf[4..5]);
        (*self.c.borrow()).to_bytes(&mut buf[8..16]);
        (*self.d.borrow()).to_bytes(&mut buf[16..24]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            b: Rc::new(RefCell::new(<u8>::from_bytes(&buf[4..5]))),
            c: Rc::new(RefCell::new(<Inner>::from_bytes(&buf[8..16]))),
            d: Rc::new(RefCell::new(<Inner>::from_bytes(&buf[16..24]))),
        }
    }
}
#[derive()]
pub struct Boxed_int_ {
    pub v: Value<i32>,
    pub tag: Value<i32>,
}
impl Boxed_int_ {
    pub fn Boxed_int_(x: i32, t: i32) -> Self {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let t: Value<i32> = Rc::new(RefCell::new(t));
        let mut this = Self {
            v: Rc::new(RefCell::new((*x.borrow()))),
            tag: Rc::new(RefCell::new((*t.borrow()))),
        };
        this
    }
}
impl Clone for Boxed_int_ {
    fn clone(&self) -> Self {
        let mut this = Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
            tag: Rc::new(RefCell::new((*self.tag.borrow()))),
        };
        this
    }
}
impl Default for Boxed_int_ {
    fn default() -> Self {
        Boxed_int_ {
            v: <Value<i32>>::default(),
            tag: <Value<i32>>::default(),
        }
    }
}
impl ByteRepr for Boxed_int_ {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
        (*self.tag.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            tag: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(<S>::default()));
    assert!(((*(*s.borrow()).a.borrow()) == 1));
    assert!((((*(*s.borrow()).b.borrow()) as i32) == 2));
    assert!(((*(*(*s.borrow()).c.borrow()).x.borrow()) == 3));
    assert!(((*(*(*s.borrow()).c.borrow()).y.borrow()) == 4));
    assert!(((*(*(*s.borrow()).d.borrow()).x.borrow()) == 3));
    assert!(((*(*(*s.borrow()).d.borrow()).y.borrow()) == 4));
    let boxed: Value<Boxed_int_> = Rc::new(RefCell::new(Boxed_int_::Boxed_int_({ 5 }, { 9 })));
    assert!(((*(*boxed.borrow()).v.borrow()) == 5));
    assert!(((*(*boxed.borrow()).tag.borrow()) == 9));
    return 0;
}
