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
    pub hi: Value<u32>,
    pub fill: Value<Box<[u8]>>,
}
impl Default for shape_b {
    fn default() -> Self {
        shape_b {
            code: <Value<u16>>::default(),
            lo: <Value<u16>>::default(),
            hi: <Value<u32>>::default(),
            fill: Rc::new(RefCell::new(
                (0..8).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
            )),
        }
    }
}
impl ByteRepr for shape_b {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.code.borrow()).to_bytes(&mut buf[0..2]);
        (*self.lo.borrow()).to_bytes(&mut buf[2..4]);
        (*self.hi.borrow()).to_bytes(&mut buf[4..8]);
        (*self.fill.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            code: Rc::new(RefCell::new(<u16>::from_bytes(&buf[0..2]))),
            lo: Rc::new(RefCell::new(<u16>::from_bytes(&buf[2..4]))),
            hi: Rc::new(RefCell::new(<u32>::from_bytes(&buf[4..8]))),
            fill: Rc::new(RefCell::new(<Box<[u8]>>::from_bytes(&buf[8..16]))),
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
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_sized(256)
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
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 256]))),
        }
    }
}
impl ByteRepr for anon_0 {
    fn byte_size() -> usize {
        256
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
    pub view: Value<anon_0>,
}
impl ByteRepr for Container {
    fn byte_size() -> usize {
        256
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.view.borrow()).to_bytes(&mut buf[0..256]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            view: Rc::new(RefCell::new(<anon_0>::from_bytes(&buf[0..256]))),
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
            .memset((0) as u8, 256usize as usize);
        ((c.as_pointer()) as Ptr<Container>).to_any().clone()
    };
    assert!(
        (((((*(*(*(*c.borrow()).view.borrow()).a().upgrade().deref())
            .code
            .borrow()) as i32)
            == 0) as i32)
            != 0)
    );
    assert!(
        (((((*(*(*(*c.borrow()).view.borrow()).b().upgrade().deref())
            .lo
            .borrow()) as i32)
            == 0) as i32)
            != 0)
    );
    assert!(
        ((((((*(*c.borrow()).view.borrow()).raw_().read())[(0) as usize] as i32) == 0) as i32)
            != 0)
    );
    assert!(
        ((((((*(*c.borrow()).view.borrow()).raw_().read())[(255) as usize] as i32) == 0) as i32)
            != 0)
    );
    let src: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8,
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
        <u8>::default(),
    ])));
    (*src.borrow_mut())[(0) as usize] = 2_u8;
    (*src.borrow_mut())[(2) as usize] = 80_u8;
    (*src.borrow_mut())[(3) as usize] = 0_u8;
    (*src.borrow_mut())[(4) as usize] = 127_u8;
    (*src.borrow_mut())[(5) as usize] = 0_u8;
    (*src.borrow_mut())[(6) as usize] = 0_u8;
    (*src.borrow_mut())[(7) as usize] = 1_u8;
    let len: Value<usize> = Rc::new(RefCell::new(16_usize));
    assert!(((((*len.borrow()) <= ::std::mem::size_of::<[u8; 256]>()) as i32) != 0));
    {
        (((*(*c.borrow()).view.borrow())
            .raw_()
            .reinterpret_cast::<u8>())
        .clone() as Ptr<u8>)
            .to_any()
            .memcpy(
                &((src.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                (*len.borrow()) as usize,
            );
        (((*(*c.borrow()).view.borrow())
            .raw_()
            .reinterpret_cast::<u8>())
        .clone() as Ptr<u8>)
            .to_any()
            .clone()
    };
    assert!(
        (((((*(*(*(*c.borrow()).view.borrow()).b().upgrade().deref())
            .code
            .borrow()) as i32)
            == 2) as i32)
            != 0)
    );
    assert!(
        ((((((((*(*(*c.borrow()).view.borrow()).b().upgrade().deref())
            .lo
            .as_pointer())
        .reinterpret_cast::<u8>())
        .offset((0) as isize)
        .read()) as i32)
            == 80) as i32)
            != 0)
    );
    {
        ((c.as_pointer()) as Ptr<Container>)
            .to_any()
            .memset((0) as u8, 256usize as usize);
        ((c.as_pointer()) as Ptr<Container>).to_any().clone()
    };
    assert!(
        (((((*(*(*(*c.borrow()).view.borrow()).b().upgrade().deref())
            .code
            .borrow()) as i32)
            == 0) as i32)
            != 0)
    );
    return 0;
}
