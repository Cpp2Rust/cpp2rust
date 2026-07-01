extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub struct shape_a {
    pub code: Value<u16>,
    pub pad: Value<Box<[u8]>>,
}
impl Default for shape_a {
    fn default() -> Self {
        shape_a {
            code: <Value<u16>>::default(),
            pad: Rc::new(RefCell::new(
                (0..14).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
            )),
        }
    }
}
impl ByteRepr for shape_a {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.code.borrow()).to_bytes(&mut buf[0..2]);
        (*self.pad.borrow()).to_bytes(&mut buf[2..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            code: Rc::new(RefCell::new(<u16>::from_bytes(&buf[0..2]))),
            pad: Rc::new(RefCell::new(<Box<[u8]>>::from_bytes(&buf[2..16]))),
        }
    }
}
#[derive()]
pub struct shape_b {
    pub code: Value<u16>,
    pub lo: Value<u16>,
    pub mid: Value<u32>,
    pub fill: Value<Box<[u8]>>,
    pub tail: Value<u32>,
}
impl Default for shape_b {
    fn default() -> Self {
        shape_b {
            code: <Value<u16>>::default(),
            lo: <Value<u16>>::default(),
            mid: <Value<u32>>::default(),
            fill: Rc::new(RefCell::new(
                (0..16).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
            )),
            tail: <Value<u32>>::default(),
        }
    }
}
impl ByteRepr for shape_b {
    fn byte_size() -> usize {
        28
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.code.borrow()).to_bytes(&mut buf[0..2]);
        (*self.lo.borrow()).to_bytes(&mut buf[2..4]);
        (*self.mid.borrow()).to_bytes(&mut buf[4..8]);
        (*self.fill.borrow()).to_bytes(&mut buf[8..24]);
        (*self.tail.borrow()).to_bytes(&mut buf[24..28]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            code: Rc::new(RefCell::new(<u16>::from_bytes(&buf[0..2]))),
            lo: Rc::new(RefCell::new(<u16>::from_bytes(&buf[2..4]))),
            mid: Rc::new(RefCell::new(<u32>::from_bytes(&buf[4..8]))),
            fill: Rc::new(RefCell::new(<Box<[u8]>>::from_bytes(&buf[8..24]))),
            tail: Rc::new(RefCell::new(<u32>::from_bytes(&buf[24..28]))),
        }
    }
}
pub struct anon_0 {
    __bytes: Value<Box<[u8]>>,
}
impl anon_0 {
    pub fn a(&self) -> Ptr<shape_a> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn b(&self) -> Ptr<shape_b> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn raw_(&self) -> Ptr<Box<[u8]>> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_sized(64)
    }
}
impl Clone for anon_0 {
    fn clone(&self) -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(self.__bytes.borrow().clone())),
        }
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 64]))),
        }
    }
}
impl ByteRepr for anon_0 {
    fn byte_size() -> usize {
        64
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        buf.copy_from_slice(&self.__bytes.borrow());
    }
    fn from_bytes(buf: &[u8]) -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(Box::from(buf))),
        }
    }
}
#[derive(Default)]
pub struct Container {
    pub len: Value<u32>,
    pub u: Value<anon_0>,
}
impl ByteRepr for Container {
    fn byte_size() -> usize {
        68
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.len.borrow()).to_bytes(&mut buf[0..4]);
        (*self.u.borrow()).to_bytes(&mut buf[4..68]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            len: Rc::new(RefCell::new(<u32>::from_bytes(&buf[0..4]))),
            u: Rc::new(RefCell::new(<anon_0>::from_bytes(&buf[4..68]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let c: Value<Container> = <Value<Container>>::default();
    {
        ((c.as_pointer()) as Ptr<Container>)
            .to_any()
            .memset((0) as u8, 68usize as usize);
        ((c.as_pointer()) as Ptr<Container>).to_any().clone()
    };
    (*(*(*(*c.borrow()).u.borrow()).a().upgrade().deref())
        .code
        .borrow_mut()) = 10_u16;
    (*(*c.borrow()).len.borrow_mut()) = (28usize as u32);
    (*(*(((*(*c.borrow()).u.borrow()).a())
        .clone()
        .to_any()
        .cast::<shape_b>()
        .expect("ub:wrong type"))
    .upgrade()
    .deref())
    .tail
    .borrow_mut()) = 3735928559_u32;
    assert!(
        ((((*(*(*(*c.borrow()).u.borrow()).b().upgrade().deref())
            .tail
            .borrow())
            == 3735928559_u32) as i32)
            != 0)
    );
    assert!(
        (((((*(*(*(*c.borrow()).u.borrow()).b().upgrade().deref())
            .code
            .borrow()) as i32)
            == 10) as i32)
            != 0)
    );
    (*(*(*(*c.borrow()).u.borrow()).b().upgrade().deref())
        .lo
        .borrow_mut()) = 8080_u16;
    assert!(
        ((((((((*(*c.borrow()).u.borrow()).raw_().reinterpret_cast::<u8>())
            .reinterpret_cast::<u8>())
        .offset((2) as isize)
        .read()) as i32)
            == 144) as i32)
            != 0)
    );
    assert!(
        ((((((((*(*c.borrow()).u.borrow()).raw_().reinterpret_cast::<u8>())
            .reinterpret_cast::<u8>())
        .offset((3) as isize)
        .read()) as i32)
            == 31) as i32)
            != 0)
    );
    return 0;
}
