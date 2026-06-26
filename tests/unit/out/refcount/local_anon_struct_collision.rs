extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn first_0() -> i32 {
    #[derive(Default)]
    pub struct anon_1 {
        pub x: Value<i32>,
        pub y: Value<i32>,
    }
    impl ByteRepr for anon_1 {
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
    };
    let p: Value<anon_1> = <Value<anon_1>>::default();
    (*(*p.borrow()).x.borrow_mut()) = 1;
    (*(*p.borrow()).y.borrow_mut()) = 2;
    return ((*(*p.borrow()).x.borrow()) + (*(*p.borrow()).y.borrow()));
}
pub fn second_2() -> i32 {
    #[derive(Default)]
    pub struct anon_3 {
        pub a: Value<i64>,
        pub b: Value<i64>,
    }
    impl ByteRepr for anon_3 {
        fn byte_size() -> usize {
            16
        }
        fn to_bytes(&self, buf: &mut [u8]) {
            (*self.a.borrow()).to_bytes(&mut buf[0..8]);
            (*self.b.borrow()).to_bytes(&mut buf[8..16]);
        }
        fn from_bytes(buf: &[u8]) -> Self {
            Self {
                a: Rc::new(RefCell::new(<i64>::from_bytes(&buf[0..8]))),
                b: Rc::new(RefCell::new(<i64>::from_bytes(&buf[8..16]))),
            }
        }
    };
    let q: Value<anon_3> = <Value<anon_3>>::default();
    (*(*q.borrow()).a.borrow_mut()) = 10_i64;
    (*(*q.borrow()).b.borrow_mut()) = 20_i64;
    return (((*(*q.borrow()).a.borrow()) + (*(*q.borrow()).b.borrow())) as i32);
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((((({ first_0() }) == 3) as i32) != 0));
    assert!((((({ second_2() }) == 30) as i32) != 0));
    return 0;
}
