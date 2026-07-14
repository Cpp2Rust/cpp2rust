extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Layout {
    pub a: Value<u8>,
    pub b: Value<u32>,
    pub c: Value<u16>,
}
impl Clone for Layout {
    fn clone(&self) -> Self {
        let mut this = Self {
            a: Rc::new(RefCell::new((*self.a.borrow()))),
            b: Rc::new(RefCell::new((*self.b.borrow()))),
            c: Rc::new(RefCell::new((*self.c.borrow()))),
        };
        this
    }
}
impl ByteRepr for Layout {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..1]);
        (*self.b.borrow()).to_bytes(&mut buf[4..8]);
        (*self.c.borrow()).to_bytes(&mut buf[8..10]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<u8>::from_bytes(&buf[0..1]))),
            b: Rc::new(RefCell::new(<u32>::from_bytes(&buf[4..8]))),
            c: Rc::new(RefCell::new(<u16>::from_bytes(&buf[8..10]))),
        }
    }
}
#[derive()]
pub struct Frame {
    pub tag: Value<u16>,
    pub body: Value<Box<[u8]>>,
}
impl Clone for Frame {
    fn clone(&self) -> Self {
        let mut this = Self {
            tag: Rc::new(RefCell::new((*self.tag.borrow()))),
            body: Rc::new(RefCell::new((*self.body.borrow()).clone())),
        };
        this
    }
}
impl Default for Frame {
    fn default() -> Self {
        Frame {
            tag: <Value<u16>>::default(),
            body: Rc::new(RefCell::new(
                (0..64).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
            )),
        }
    }
}
impl ByteRepr for Frame {
    fn byte_size() -> usize {
        66
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.tag.borrow()).to_bytes(&mut buf[0..2]);
        (*self.body.borrow()).to_bytes(&mut buf[2..66]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            tag: Rc::new(RefCell::new(<u16>::from_bytes(&buf[0..2]))),
            body: Rc::new(RefCell::new(<Box<[u8]>>::from_bytes(&buf[2..66]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((0_usize == 0_usize));
    assert!((4_usize == 4_usize));
    assert!((8_usize == 8_usize));
    let v: Value<Layout> = Rc::new(RefCell::new(Layout {
        a: Rc::new(RefCell::new(0_u8)),
        b: Rc::new(RefCell::new(<u32>::default())),
        c: Rc::new(RefCell::new(<u16>::default())),
    }));
    (*(*v.borrow()).b.borrow_mut()) = 3735928559_u32;
    let base: Value<Ptr<u8>> = Rc::new(RefCell::new((v.as_pointer()).reinterpret_cast::<u8>()));
    let bp: Value<Ptr<u32>> = Rc::new(RefCell::new(
        ((*base.borrow()).offset((4_usize) as isize)).reinterpret_cast::<u32>(),
    ));
    assert!((((*bp.borrow()).read()) == 3735928559_u32));
    ((*base.borrow()).offset((4_usize) as isize))
        .reinterpret_cast::<u32>()
        .write(305419896_u32);
    assert!(((*(*v.borrow()).b.borrow()) == 305419896_u32));
    let text: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"example-body")));
    let len: Value<usize> = Rc::new(RefCell::new(
        ({
            let mut __p = (*text.borrow()).clone();
            let mut __i: usize = 0;
            while __p.read() != 0 {
                __p += 1;
                __i += 1;
            }
            __i
        })
        .wrapping_add(1_usize),
    ));
    let total: Value<usize> = Rc::new(RefCell::new(
        ((2_usize as u64).wrapping_add(((*len.borrow()) as u64)) as usize),
    ));
    assert!(((*total.borrow()) == (2_usize).wrapping_add((*len.borrow()))));
    return 0;
}
