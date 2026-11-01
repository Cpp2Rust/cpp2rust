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
pub struct point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let src: Value<point> = Rc::new(RefCell::new(point { x: 3, y: 7 }));
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new((0..8).map(|_| 0_u8).collect::<Box<[u8]>>()));
    match ({ ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any() },) {
        (mut __tmp_0,) => {
            __tmp_0.memcpy(
                &((src.as_pointer()) as Ptr<point>).to_any(),
                ::std::mem::size_of::<[u8; 8]>() as usize,
            );
            (__tmp_0).clone()
        }
    };
    let dst: Value<point> = <Value<point>>::default();
    match ({ ((dst.as_pointer()) as Ptr<point>).to_any() },) {
        (mut __tmp_1,) => {
            __tmp_1.memcpy(
                &((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
                8usize as usize,
            );
            (__tmp_1).clone()
        }
    };
    assert!(((({ (*dst.borrow()).x } == 3) as i32) != 0));
    assert!(((({ (*dst.borrow()).y } == 7) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
