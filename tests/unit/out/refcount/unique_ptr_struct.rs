extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl ByteRepr for Point {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x.to_bytes(&mut buf[0..4]);
        self.y.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: <i32>::from_bytes(&buf[0..4]),
            y: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
pub fn sum_0(p: Point) -> i32 {
    let p: Value<Point> = Rc::new(RefCell::new(p));
    return ({ (*p.borrow()).x } + { (*p.borrow()).y });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p: Value<Option<Value<Point>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Point {
            x: 3,
            y: 4,
        })))));
    (*(*p.borrow()).as_ref().unwrap().borrow_mut()).x += 10;
    let __rhs = ({ (*(*p.borrow()).as_ref().unwrap().borrow()).x } + {
        (*(*p.borrow()).as_ref().unwrap().borrow()).y
    });
    (*(*p.borrow()).as_ref().unwrap().borrow_mut()).y = __rhs;
    let s: Value<i32> = Rc::new(RefCell::new(
        ({ sum_0((*(*p.borrow()).as_ref().unwrap().borrow()).clone()) }),
    ));
    assert!(((*s.borrow()) == 30));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
