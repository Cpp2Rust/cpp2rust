extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub struct pair {
    pub a: Value<Box<[i32]>>,
    pub b: Value<Box<[i32]>>,
}
impl Default for pair {
    fn default() -> Self {
        pair {
            a: Rc::new(RefCell::new(
                (0..4).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
            )),
            b: Rc::new(RefCell::new(
                (0..4).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
            )),
        }
    }
}
impl ByteRepr for pair {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..16]);
        (*self.b.borrow()).to_bytes(&mut buf[16..32]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<Box<[i32]>>::from_bytes(&buf[0..16]))),
            b: Rc::new(RefCell::new(<Box<[i32]>>::from_bytes(&buf[16..32]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<pair> = <Value<pair>>::default();
    assert!(
        (((((*s.borrow()).a.as_pointer() as Ptr::<i32>).offset((4) as isize)
            == ((*s.borrow()).b.as_pointer() as Ptr::<i32>)) as i32)
            != 0)
    );
    return 0;
}
