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
pub fn set_0(mut ref_: Ptr<i32>, mut val: i32) {
    ref_.write(val);
}
pub fn read_1(mut ref_: Ptr<i32>) -> i32 {
    let r: Ptr<i32> = (ref_).clone();
    return (r.read());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let i1: Value<i32> = Rc::new(RefCell::new(10));
    let mut ref_1: Ptr<i32> = i1.as_pointer();
    ref_1.write(20);
    let i2: Ptr<i32> = (ref_1).clone();
    {
        i2.with_mut(|__v| *__v = *__v + 5)
    };
    write!(libcc2rs::cout(), "{:}\n", (*i1.borrow()),);
    let i3: Value<i32> = Rc::new(RefCell::new(1));
    let i4: Value<i32> = Rc::new(RefCell::new(2));
    let mut ref_3: Ptr<i32> = i3.as_pointer();
    let mut ref_4: Ptr<i32> = i4.as_pointer();
    let __rhs = (ref_4.read());
    ref_3.write(__rhs);
    write!(
        libcc2rs::cout(),
        "{:} {:}\n",
        (*i3.borrow()),
        (*i4.borrow()),
    );
    ({ set_0((ref_1).clone(), 99) });
    write!(
        libcc2rs::cout(),
        "{:} {:}\n",
        (*i1.borrow()),
        ({ read_1((ref_1).clone(),) }),
    );
    let point: Value<Point> = Rc::new(RefCell::new(Point { x: 3, y: 4 }));
    let mut point_ref: Ptr<Point> = point.as_pointer();
    field!(point_ref, x).write(30);
    field!(point_ref, y).write(40);
    write!(libcc2rs::cout(), "{:} {:}\n", { (*point.borrow()).x }, {
        (*point.borrow()).y
    },);
    return 0;
}
pub fn __cpp2rust_init_globals() {}
