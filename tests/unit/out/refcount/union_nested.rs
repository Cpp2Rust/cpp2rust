extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub struct record {
    pub code: Value<u16>,
    pub pad: Value<Box<[u8]>>,
}
impl Default for record {
    fn default() -> Self {
        record {
            code: <Value<u16>>::default(),
            pad: Rc::new(RefCell::new(
                (0..14).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
            )),
        }
    }
}
impl ByteRepr for record {
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
pub struct anon_0 {
    __bytes: Value<Box<[u8]>>,
}
impl anon_0 {
    pub fn h(&self) -> Ptr<record> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn raw_(&self) -> Ptr<Box<[u8]>> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
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
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 128]))),
        }
    }
}
impl ByteRepr for anon_0 {
    fn byte_size() -> usize {
        128
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
pub struct inner {
    pub view: Value<anon_0>,
}
impl ByteRepr for inner {
    fn byte_size() -> usize {
        128
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.view.borrow()).to_bytes(&mut buf[0..128]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            view: Rc::new(RefCell::new(<anon_0>::from_bytes(&buf[0..128]))),
        }
    }
}
pub struct anon_1 {
    __bytes: Value<Box<[u8]>>,
}
impl anon_1 {
    pub fn h(&self) -> Ptr<record> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn nested(&self) -> Ptr<inner> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
}
impl Clone for anon_1 {
    fn clone(&self) -> Self {
        anon_1 {
            __bytes: Rc::new(RefCell::new(self.__bytes.borrow().clone())),
        }
    }
}
impl Default for anon_1 {
    fn default() -> Self {
        anon_1 {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 128]))),
        }
    }
}
impl ByteRepr for anon_1 {
    fn byte_size() -> usize {
        128
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        buf.copy_from_slice(&self.__bytes.borrow());
    }
    fn from_bytes(buf: &[u8]) -> Self {
        anon_1 {
            __bytes: Rc::new(RefCell::new(Box::from(buf))),
        }
    }
}
#[derive(Default)]
pub struct Outer {
    pub kind: Value<i32>,
    pub level: Value<i32>,
    pub variant: Value<i32>,
    pub len: Value<u32>,
    pub body: Value<anon_1>,
}
impl ByteRepr for Outer {
    fn byte_size() -> usize {
        144
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.kind.borrow()).to_bytes(&mut buf[0..4]);
        (*self.level.borrow()).to_bytes(&mut buf[4..8]);
        (*self.variant.borrow()).to_bytes(&mut buf[8..12]);
        (*self.len.borrow()).to_bytes(&mut buf[12..16]);
        (*self.body.borrow()).to_bytes(&mut buf[16..144]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            kind: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            level: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
            variant: Rc::new(RefCell::new(<i32>::from_bytes(&buf[8..12]))),
            len: Rc::new(RefCell::new(<u32>::from_bytes(&buf[12..16]))),
            body: Rc::new(RefCell::new(<anon_1>::from_bytes(&buf[16..144]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let ex: Value<Outer> = <Value<Outer>>::default();
    {
        ((ex.as_pointer()) as Ptr<Outer>)
            .to_any()
            .memset((0) as u8, 144usize as usize);
        ((ex.as_pointer()) as Ptr<Outer>).to_any().clone()
    };
    (*(*ex.borrow()).kind.borrow_mut()) = 2;
    (*(*ex.borrow()).level.borrow_mut()) = 1;
    (*(*ex.borrow()).variant.borrow_mut()) = 6;
    (*(*ex.borrow()).len.borrow_mut()) = (16usize as u32);
    (*(*(*(*ex.borrow()).body.borrow()).h().upgrade().deref())
        .code
        .borrow_mut()) = 2_u16;
    (*(*(*(*ex.borrow()).body.borrow()).h().upgrade().deref())
        .pad
        .borrow_mut())[(0) as usize] = (('X' as i32) as u8);
    assert!(
        (((((*(*(*(*ex.borrow()).body.borrow()).h().upgrade().deref())
            .code
            .borrow()) as i32)
            == 2) as i32)
            != 0)
    );
    assert!(
        (((((*(*(*(*ex.borrow()).body.borrow()).h().upgrade().deref())
            .pad
            .borrow())[(0) as usize] as i32)
            == ('X' as i32)) as i32)
            != 0)
    );
    assert!(
        (((((*(*(*(*(*(*ex.borrow()).body.borrow()).nested().upgrade().deref())
            .view
            .borrow())
        .h()
        .upgrade()
        .deref())
        .code
        .borrow()) as i32)
            == 2) as i32)
            != 0)
    );
    return 0;
}
