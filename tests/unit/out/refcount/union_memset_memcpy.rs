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
#[byte_size(16)]
pub struct shape_b {
    #[offset(0)]
    pub code: u16,
    #[offset(2)]
    pub lo: u16,
    #[offset(4)]
    pub hi: u32,
    #[offset(8)]
    #[byte_size(8)]
    pub fill: Value<Box<[i8]>>,
}
impl Default for shape_b {
    fn default() -> Self {
        shape_b {
            code: 0_u16,
            lo: 0_u16,
            hi: 0_u32,
            fill: Rc::new(RefCell::new((0..8).map(|_| 0_i8).collect::<Box<[i8]>>())),
        }
    }
}
#[derive(VaArg, FnPtrArg, ByteRepr, DeepClone)]
#[byte_size(256)]
pub struct anon_0 {
    #[offset(0)]
    #[byte_size(256)]
    __bytes: Value<Box<[u8]>>,
}
impl anon_0 {
    pub fn a(this: Ptr<Self>) -> Ptr<shape_a> {
        this.reinterpret_cast()
    }
    pub fn b(this: Ptr<Self>) -> Ptr<shape_b> {
        this.reinterpret_cast()
    }
    pub fn raw_(this: Ptr<Self>) -> Ptr<i8> {
        this.reinterpret_cast()
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 256]))),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(256)]
pub struct Container {
    #[offset(0)]
    #[byte_size(256)]
    pub view: anon_0,
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
            .memset((0) as u8, 256usize as usize);
        ((c.as_pointer()) as Ptr<Container>).to_any()
    };
    assert!(
        ((((anon_0::a(field_ptr!(c.as_pointer(), view)).with(|__s| __s.code) as i32) == 0) as i32)
            != 0)
    );
    assert!(
        ((((anon_0::b(field_ptr!(c.as_pointer(), view)).with(|__s| __s.lo) as i32) == 0) as i32)
            != 0)
    );
    assert!(
        (((((elem!(
            (anon_0::raw_(field_ptr!(c.as_pointer(), view)).reinterpret_cast::<i8>() as Ptr::<i8>),
            0
        )
        .read()) as i32)
            == 0) as i32)
            != 0)
    );
    assert!(
        (((((elem!(
            (anon_0::raw_(field_ptr!(c.as_pointer(), view)).reinterpret_cast::<i8>() as Ptr::<i8>),
            255
        )
        .read()) as i32)
            == 0) as i32)
            != 0)
    );
    let src: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        0_u8,
    ])));
    (*src.borrow_mut())[(0) as usize] = 2_u8;
    (*src.borrow_mut())[(2) as usize] = 80_u8;
    (*src.borrow_mut())[(3) as usize] = 0_u8;
    (*src.borrow_mut())[(4) as usize] = 127_u8;
    (*src.borrow_mut())[(5) as usize] = 0_u8;
    (*src.borrow_mut())[(6) as usize] = 0_u8;
    (*src.borrow_mut())[(7) as usize] = 1_u8;
    let mut len: usize = 16_usize;
    assert!((((len <= ::std::mem::size_of::<[i8; 256]>()) as i32) != 0));
    {
        ((anon_0::raw_(field_ptr!(c.as_pointer(), view)).reinterpret_cast::<i8>()) as Ptr<i8>)
            .to_any()
            .memcpy(
                &((src.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                len as usize,
            );
        ((anon_0::raw_(field_ptr!(c.as_pointer(), view)).reinterpret_cast::<i8>()) as Ptr<i8>)
            .to_any()
    };
    assert!(
        ((((anon_0::b(field_ptr!(c.as_pointer(), view)).with(|__s| __s.code) as i32) == 2) as i32)
            != 0)
    );
    assert!(
        (((((elem!(
            ((field_ptr!(anon_0::b(field_ptr!(c.as_pointer(), view)), lo))
                .reinterpret_cast::<u8>()),
            0
        )
        .read()) as i32)
            == 80) as i32)
            != 0)
    );
    {
        ((c.as_pointer()) as Ptr<Container>)
            .to_any()
            .memset((0) as u8, 256usize as usize);
        ((c.as_pointer()) as Ptr<Container>).to_any()
    };
    assert!(
        ((((anon_0::b(field_ptr!(c.as_pointer(), view)).with(|__s| __s.code) as i32) == 0) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
