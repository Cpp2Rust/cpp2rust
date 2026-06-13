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
impl ByteRepr for Point {
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
pub fn agg_0(n: i32) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let buf40: Value<Box<[::core::ffi::c_char]>> = Rc::new(RefCell::new(
        (0..40)
            .map(|_| <::core::ffi::c_char>::default())
            .collect::<Box<[::core::ffi::c_char]>>(),
    ));
    let buf256: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..256).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
    let arr64: Value<Box<[i32]>> = Rc::new(RefCell::new(
        (0..64).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
    ));
    let longs: Value<Box<[i64]>> = Rc::new(RefCell::new(
        (0..33).map(|_| <i64>::default()).collect::<Box<[i64]>>(),
    ));
    let p: Value<Point> = <Value<Point>>::default();
    let ptr: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
    let fp: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(FnPtr::null()));
    let file: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(Ptr::null()));
    let total: Value<i32> = <Value<i32>>::default();
    goto_block!({
        '__entry: {
            *total.borrow_mut() = 0;
            if ((((*n.borrow()) < 0) as i32) != 0) {
                goto!('out);
            }
            (*total.borrow_mut()) = 1;
        }
        'out: {
            return (*total.borrow());
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (((({
            let _n: i32 = -1_i32;
            agg_0(_n)
        }) == 0) as i32)
            != 0)
    );
    assert!((((({ agg_0(1,) }) == 1) as i32) != 0));
    return 0;
}
