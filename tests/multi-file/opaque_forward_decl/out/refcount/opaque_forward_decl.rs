extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct container {
    pub p: Value<Ptr<opaque>>,
    pub x: Value<i32>,
}
impl ByteRepr for container {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.p.borrow()).to_bytes(&mut buf[0..8]);
        (*self.x.borrow()).to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            p: Rc::new(RefCell::new(<Ptr<opaque>>::from_bytes(&buf[0..8]))),
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[8..12]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let c: Value<container> = Rc::new(RefCell::new(container {
        p: Rc::new(RefCell::new(Ptr::<opaque>::null())),
        x: Rc::new(RefCell::new(42)),
    }));
    ({ touch_0((c.as_pointer())) });
    assert!(((((*(*c.borrow()).x.borrow()) == 42) as i32) != 0));
    assert!(((((*(*c.borrow()).p.borrow()).is_null()) as i32) != 0));
    return 0;
}
pub fn touch_0(c: Ptr<container>) {
    let c: Value<Ptr<container>> = Rc::new(RefCell::new(c));
    (*(*(*c.borrow()).upgrade().deref()).p.borrow()).clone();
}
pub struct opaque;
impl ByteRepr for opaque {}
