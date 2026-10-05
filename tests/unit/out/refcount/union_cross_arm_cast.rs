extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct shape_a {
    #[offset(0)]
    pub code: u16,
    #[offset(2)]
    #[byte_size(14)]
    pub pad: Value<Box<[i8]>>,
}
impl Default for shape_a {
    fn default() -> Self {
        shape_a {
            code: 0_u16,
            pad: Rc::new(RefCell::new((0..14).map(|_| 0_i8).collect::<Box<[i8]>>())),
        }
    }
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(28)]
pub struct shape_b {
    #[offset(0)]
    pub code: u16,
    #[offset(2)]
    pub lo: u16,
    #[offset(4)]
    pub mid: u32,
    #[offset(8)]
    #[byte_size(16)]
    pub fill: Value<Box<[u8]>>,
    #[offset(24)]
    pub tail: u32,
}
impl Default for shape_b {
    fn default() -> Self {
        shape_b {
            code: 0_u16,
            lo: 0_u16,
            mid: 0_u32,
            fill: Rc::new(RefCell::new((0..16).map(|_| 0_u8).collect::<Box<[u8]>>())),
            tail: 0_u32,
        }
    }
}
#[derive(ByteRepr, DeepClone)]
#[byte_size(64)]
pub struct anon_0 {
    #[offset(0)]
    #[byte_size(64)]
    __bytes: Value<Box<[u8]>>,
}
impl anon_0 {
    pub fn a(&self) -> Ptr<shape_a> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn b(&self) -> Ptr<shape_b> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn raw_(&self) -> Ptr<i8> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 64]))),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(68)]
pub struct Container {
    #[offset(0)]
    pub len: u32,
    #[offset(4)]
    #[byte_size(64)]
    pub u: anon_0,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let c: Value<Container> = <Value<Container>>::default();
    {
        ((c.as_pointer()) as Ptr<Container>)
            .to_any()
            .memset((0) as u8, 68usize as usize);
        ((c.as_pointer()) as Ptr<Container>).to_any()
    };
    field!((*c.borrow_mut()).u.a(), code).write(10_u16);
    (*c.borrow_mut()).len = (28usize as u32);
    field!(
        (((*c.borrow()).u.a()).to_any().reinterpret_cast::<shape_b>()),
        tail
    )
    .write(3735928559_u32);
    assert!(((((*c.borrow()).u.b().with(|__s| __s.tail) == 3735928559_u32) as i32) != 0));
    assert!((((((*c.borrow()).u.b().with(|__s| __s.code) as i32) == 10) as i32) != 0));
    field!((*c.borrow_mut()).u.b(), lo).write(8080_u16);
    assert!(
        (((((elem!(
            ((((*c.borrow()).u.raw_().reinterpret_cast::<i8>()) as Ptr<i8>)
                .reinterpret_cast::<u8>()),
            2
        )
        .read()) as i32)
            == 144) as i32)
            != 0)
    );
    assert!(
        (((((elem!(
            ((((*c.borrow()).u.raw_().reinterpret_cast::<i8>()) as Ptr<i8>)
                .reinterpret_cast::<u8>()),
            3
        )
        .read()) as i32)
            == 31) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
