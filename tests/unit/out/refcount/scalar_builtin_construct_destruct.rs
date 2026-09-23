extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Pod {
    pub v: Value<i32>,
}
impl Clone for Pod {
    fn clone(&self) -> Self {
        let __this: Value<Pod> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<Pod> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Pod {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn zero_0() -> Ptr<i32> {
    return Ptr::<i32>::null();
}
pub fn zero_1() -> i64 {
    return 0_i64;
}
pub fn destroy_2(p: Ptr<i32>) {
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p));
}
pub fn destroy_3(p: Ptr<Pod>) {
    let p: Value<Ptr<Pod>> = Rc::new(RefCell::new(p));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let i: Value<i32> = Rc::new(RefCell::new(0_i32));
    let d: Value<f64> = Rc::new(RefCell::new(0.0_f64));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(({ zero_0() })));
    assert!(((*i.borrow()) == 0));
    assert!(((*d.borrow()) == 0.0E+0));
    assert!((*p.borrow()).is_null());
    assert!((({ zero_1() }) == 0_i64));
    let x: Value<i32> = Rc::new(RefCell::new(5));
    ({ destroy_2((x.as_pointer())) });
    assert!(((*x.borrow()) == 5));
    let pod: Value<Pod> = Rc::new(RefCell::new(Pod {
        v: Rc::new(RefCell::new(7)),
    }));
    ({ destroy_3((pod.as_pointer())) });
    assert!(((*(*pod.borrow()).v.borrow()) == 7));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
