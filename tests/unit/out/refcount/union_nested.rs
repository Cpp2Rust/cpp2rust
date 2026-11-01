extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct record {
    #[offset(0)]
    pub code: u16,
    #[offset(2)]
    #[byte_size(14)]
    pub pad: Box<[i8]>,
}
impl Default for record {
    fn default() -> Self {
        {
            record {
                code: 0_u16,
                pad: (0..14).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(128)]
pub struct anon_0 {
    #[offset(0)]
    #[byte_size(128)]
    __bytes: Box<[u8]>,
}
impl anon_0 {
    pub fn h(this: Ptr<Self>) -> Ptr<record> {
        this.reinterpret_cast()
    }
    pub fn raw_(this: Ptr<Self>) -> Ptr<i8> {
        this.reinterpret_cast()
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Box::from([0u8; 128]),
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
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(128)]
pub struct anon_1 {
    #[offset(0)]
    #[byte_size(128)]
    __bytes: Box<[u8]>,
}
impl anon_1 {
    pub fn h(this: Ptr<Self>) -> Ptr<record> {
        this.reinterpret_cast()
    }
    pub fn nested(this: Ptr<Self>) -> Ptr<inner> {
        this.reinterpret_cast()
    }
}
impl Default for anon_1 {
    fn default() -> Self {
        anon_1 {
            __bytes: Box::from([0u8; 128]),
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
    match ({ ((ex.as_pointer()) as Ptr<Outer>).to_any() },) {
        (mut __tmp_0,) => {
            __tmp_0.memset((0) as u8, 144usize as usize);
            (__tmp_0).clone()
        }
    };
    (*ex.borrow_mut()).kind = 2;
    (*ex.borrow_mut()).level = 1;
    (*ex.borrow_mut()).variant = 6;
    (*ex.borrow_mut()).len = (16usize as u32);
    field!(anon_1::h(field_ptr!(ex.as_pointer(), body)), code).write(2_u16);
    array_elem!(anon_1::h(field_ptr!(ex.as_pointer(), body)), pad, 0).write((('X' as i32) as i8));
    assert!(
        ((((anon_1::h(field_ptr!(ex.as_pointer(), body)).with(|__s| __s.code) as i32) == 2)
            as i32)
            != 0)
    );
    assert!(
        (((((array_elem!(anon_1::h(field_ptr!(ex.as_pointer(), body)), pad, 0).read()) as i32)
            == ('X' as i32)) as i32)
            != 0)
    );
    assert!(
        ((((anon_0::h(field_ptr!(
            anon_1::nested(field_ptr!(ex.as_pointer(), body)),
            view
        ))
        .with(|__s| __s.code) as i32)
            == 2) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
