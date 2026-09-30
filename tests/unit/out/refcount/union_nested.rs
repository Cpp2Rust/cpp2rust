extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, VaArg, FnPtrArg)]
pub struct record {
    #[offset(0)]
    pub code: u16,
    #[offset(2)]
    pub pad: Value<Box<[u8]>>,
}
impl Clone for record {
    fn clone(&self) -> Self {
        Self {
            code: self.code.clone(),
            pad: Rc::new(RefCell::new((*self.pad.borrow()).clone())),
        }
    }
}
impl Default for record {
    fn default() -> Self {
        record {
            code: 0_u16,
            pad: Rc::new(RefCell::new((0..14).map(|_| 0_u8).collect::<Box<[u8]>>())),
        }
    }
}
impl ByteRepr for record {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.code.to_bytes(&mut buf[0..2]);
        (*self.pad.borrow()).to_bytes(&mut buf[2..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            code: <u16>::from_bytes(&buf[0..2]),
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
    pub fn raw_(&self) -> Ptr<u8> {
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
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct inner {
    #[offset(0)]
    pub view: anon_0,
}
impl ByteRepr for inner {
    fn byte_size() -> usize {
        128
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.view.to_bytes(&mut buf[0..128]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            view: <anon_0>::from_bytes(&buf[0..128]),
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
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Outer {
    #[offset(0)]
    pub kind: i32,
    #[offset(4)]
    pub level: i32,
    #[offset(8)]
    pub variant: i32,
    #[offset(12)]
    pub len: u32,
    #[offset(16)]
    pub body: anon_1,
}
impl ByteRepr for Outer {
    fn byte_size() -> usize {
        144
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.kind.to_bytes(&mut buf[0..4]);
        self.level.to_bytes(&mut buf[4..8]);
        self.variant.to_bytes(&mut buf[8..12]);
        self.len.to_bytes(&mut buf[12..16]);
        self.body.to_bytes(&mut buf[16..144]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            kind: <i32>::from_bytes(&buf[0..4]),
            level: <i32>::from_bytes(&buf[4..8]),
            variant: <i32>::from_bytes(&buf[8..12]),
            len: <u32>::from_bytes(&buf[12..16]),
            body: <anon_1>::from_bytes(&buf[16..144]),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let ex: Value<Outer> = <Value<Outer>>::default();
    {
        ((ex.as_pointer()) as Ptr<Outer>)
            .to_any()
            .memset((0) as u8, 144usize as usize);
        ((ex.as_pointer()) as Ptr<Outer>).to_any()
    };
    (*ex.borrow_mut()).kind = 2;
    (*ex.borrow_mut()).level = 1;
    (*ex.borrow_mut()).variant = 6;
    (*ex.borrow_mut()).len = (16usize as u32);
    (*ex.borrow())
        .body
        .h()
        .with_mut(|__s: &mut record| __s.code = 2_u16);
    (*ex.borrow())
        .body
        .h()
        .with(|__s: &record| (*__s.pad.borrow_mut())[(0) as usize] = (('X' as i32) as u8));
    assert!((((((*ex.borrow()).body.h().with(|__s: &record| __s.code) as i32) == 2) as i32) != 0));
    assert!(
        (((((*ex.borrow())
            .body
            .h()
            .with(|__s: &record| (*__s.pad.borrow())[(0) as usize]) as i32)
            == ('X' as i32)) as i32)
            != 0)
    );
    assert!(
        (((((*(*ex.borrow()).body.nested().upgrade().deref())
            .view
            .h()
            .with(|__s: &record| __s.code) as i32)
            == 2) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
