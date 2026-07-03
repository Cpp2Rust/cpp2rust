extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Point {
    pub x: Value<i32>,
    pub y: Value<i32>,
}
impl Clone for Point {
    fn clone(&self) -> Self {
        let mut this = Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
            y: Rc::new(RefCell::new((*self.y.borrow()))),
        };
        this
    }
}
impl ByteRepr for Point {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
        (*self.y.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            y: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
pub fn set_0(ref_: Ptr<i32>, val: i32) {
    let ref_: Value<Ptr<i32>> = Rc::new(RefCell::new(ref_));
    let val: Value<i32> = Rc::new(RefCell::new(val));
    (*ref_.borrow()).clone().write((*val.borrow()));
}
pub fn read_1(ref_: Ptr<i32>) -> i32 {
    let ref_: Value<Ptr<i32>> = Rc::new(RefCell::new(ref_));
    let r: Ptr<i32> = (*ref_.borrow()).clone();
    return (r.read());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let i1: Value<i32> = Rc::new(RefCell::new(10));
    let ref_1: Value<Ptr<i32>> = Rc::new(RefCell::new(i1.as_pointer()));
    (*ref_1.borrow()).clone().write(20);
    let i2: Ptr<i32> = (*ref_1.borrow()).clone();
    {
        let _ptr = i2.clone();
        _ptr.write(_ptr.read() + 5)
    };
    write!(libcc2rs::cout(), "{:}\n", (*i1.borrow()),);
    let i3: Value<i32> = Rc::new(RefCell::new(1));
    let i4: Value<i32> = Rc::new(RefCell::new(2));
    let ref_3: Value<Ptr<i32>> = Rc::new(RefCell::new(i3.as_pointer()));
    let ref_4: Value<Ptr<i32>> = Rc::new(RefCell::new(i4.as_pointer()));
    let __rhs = ((*ref_4.borrow()).clone().read());
    (*ref_3.borrow()).clone().write(__rhs);
    write!(
        libcc2rs::cout(),
        "{:} {:}\n",
        (*i3.borrow()),
        (*i4.borrow()),
    );
    ({ set_0((*ref_1.borrow()).clone(), 99) });
    write!(
        libcc2rs::cout(),
        "{:} {:}\n",
        (*i1.borrow()),
        ({ read_1((*ref_1.borrow()).clone(),) }),
    );
    let point: Value<Point> = Rc::new(RefCell::new(Point {
        x: Rc::new(RefCell::new(3)),
        y: Rc::new(RefCell::new(4)),
    }));
    let point_ref: Value<Ptr<Point>> = Rc::new(RefCell::new(point.as_pointer()));
    (*(*(*point_ref.borrow()).clone().upgrade().deref())
        .x
        .borrow_mut()) = 30;
    (*(*(*point_ref.borrow()).clone().upgrade().deref())
        .y
        .borrow_mut()) = 40;
    write!(
        libcc2rs::cout(),
        "{:} {:}\n",
        (*(*point.borrow()).x.borrow()),
        (*(*point.borrow()).y.borrow()),
    );
    return 0;
}
