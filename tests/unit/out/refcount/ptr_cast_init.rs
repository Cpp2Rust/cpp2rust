extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct header {
    #[offset(0)]
    pub tag: i32,
    #[offset(4)]
    pub size: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct view {
    #[offset(0)]
    pub tag: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let text: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"hi\0")));
    let mut cp: Ptr<i8> = (text.as_pointer() as Ptr<i8>);
    let mut u: Ptr<u8> = cp.reinterpret_cast::<u8>();
    assert!((((((elem!(u, 0).read()) as i32) == ('h' as i32)) as i32) != 0));
    assert!((((((elem!(u, 1).read()) as i32) == ('i' as i32)) as i32) != 0));
    let h: Value<header> = Rc::new(RefCell::new(header { tag: 7, size: 32 }));
    let mut hp: Ptr<header> = (h.as_pointer());
    let mut v: Ptr<view> = hp.reinterpret_cast::<view>();
    assert!((((v.with(|__s| __s.tag) == 7) as i32) != 0));
    let data: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"hi\0")));
    let mut vp: AnyPtr = ((data.as_pointer() as Ptr<i8>) as Ptr<i8>).to_any();
    let mut n: i32 = 2;
    let mut sel: Ptr<i8> = if (((n < 100) as i32) != 0) {
        (vp).clone()
    } else {
        (AnyPtr::default())
    }
    .reinterpret_cast::<i8>();
    assert!((((!((sel).is_null())) as i32) != 0));
    assert!((((((elem!(sel, 0).read()) as i32) == ('h' as i32)) as i32) != 0));
    n = 200;
    sel = if (((n < 100) as i32) != 0) {
        (vp).clone()
    } else {
        (AnyPtr::default())
    }
    .reinterpret_cast::<i8>();
    assert!(((((sel).is_null()) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
