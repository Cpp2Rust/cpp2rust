extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct Layout {
    #[offset(0)]
    pub a: u8,
    #[offset(4)]
    pub b: u32,
    #[offset(8)]
    pub c: u16,
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(66)]
pub struct Frame {
    #[offset(0)]
    pub tag: u16,
    #[offset(2)]
    #[byte_size(64)]
    pub body: Value<Box<[u8]>>,
}
impl Default for Frame {
    fn default() -> Self {
        Frame {
            tag: 0_u16,
            body: Rc::new(RefCell::new((0..64).map(|_| 0_u8).collect::<Box<[u8]>>())),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((0_usize == 0_usize));
    assert!((4_usize == 4_usize));
    assert!((8_usize == 8_usize));
    let v: Value<Layout> = Rc::new(RefCell::new(Layout {
        a: 0_u8,
        b: 0_u32,
        c: 0_u16,
    }));
    (*v.borrow_mut()).b = 3735928559_u32;
    let base: Value<Ptr<u8>> = Rc::new(RefCell::new((v.as_pointer()).reinterpret_cast::<u8>()));
    let bp: Value<Ptr<u32>> = Rc::new(RefCell::new(
        ((*base.borrow()).offset((4_usize) as isize)).reinterpret_cast::<u32>(),
    ));
    assert!((((*bp.borrow()).read()) == 3735928559_u32));
    ((*base.borrow()).offset((4_usize) as isize))
        .reinterpret_cast::<u32>()
        .write(305419896_u32);
    assert!(({ (*v.borrow()).b } == 305419896_u32));
    let text: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::<u8>::from_string_literal(
        b"example-body",
    )));
    let len: Value<usize> = Rc::new(RefCell::new(
        ((*text.borrow()).to_c_string_iterator().count()).wrapping_add(1_usize),
    ));
    let total: Value<usize> = Rc::new(RefCell::new(
        ((2_usize as u64).wrapping_add(((*len.borrow()) as u64)) as usize),
    ));
    assert!(((*total.borrow()) == (2_usize).wrapping_add((*len.borrow()))));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
