extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct record {
    #[offset(0)]
    pub code: u16,
    #[offset(2)]
    #[byte_size(14)]
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
#[derive(ByteRepr)]
#[byte_size(128)]
pub struct anon_0 {
    #[offset(0)]
    #[byte_size(128)]
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(128)]
pub struct inner {
    #[offset(0)]
    #[byte_size(128)]
    pub view: anon_0,
}
#[derive(ByteRepr)]
#[byte_size(128)]
pub struct anon_1 {
    #[offset(0)]
    #[byte_size(128)]
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(144)]
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
    #[byte_size(128)]
    pub body: anon_1,
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
    field!((*ex.borrow_mut()).body.h(), code).write(2_u16);
    elem!(
        (array_field_ptr!((*ex.borrow()).body.h(), pad) as Ptr::<u8>),
        0
    )
    .write((('X' as i32) as u8));
    assert!((((((*ex.borrow()).body.h().with(|__s| __s.code) as i32) == 2) as i32) != 0));
    assert!(
        (((((elem!(
            (array_field_ptr!((*ex.borrow()).body.h(), pad) as Ptr::<u8>),
            0
        )
        .read()) as i32)
            == ('X' as i32)) as i32)
            != 0)
    );
    assert!(
        (((((*(*ex.borrow()).body.nested().upgrade().deref())
            .view
            .h()
            .with(|__s| __s.code) as i32)
            == 2) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
