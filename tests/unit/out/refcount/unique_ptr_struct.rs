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
pub struct Point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
pub fn sum_0(mut p: Point) -> i32 {
    return (p.x + p.y);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut p: Option<Value<Point>> = Some(Rc::new(RefCell::new(Point { x: 3, y: 4 })));
    {
        field!(p.as_pointer(), x).with_mut(|__v| *__v = *__v + 10)
    };
    let __rhs = (p.as_pointer().with(|__s| __s.x) + p.as_pointer().with(|__s| __s.y));
    field!(p.as_pointer(), y).write(__rhs);
    let mut s: i32 = ({ sum_0((*p.as_pointer().upgrade().deref()).clone()) });
    assert!((s == 30));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
