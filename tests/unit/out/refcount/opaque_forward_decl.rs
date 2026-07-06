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
impl Clone for container {
    fn clone(&self) -> Self {
        Self {
            p: Rc::new(RefCell::new((*self.p.borrow()).clone())),
            x: Rc::new(RefCell::new((*self.x.borrow()).clone())),
        }
    }
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
    (*(*c.borrow()).p.borrow()).clone();
    return ((*(*c.borrow()).x.borrow()) - 42);
}
pub struct opaque;
