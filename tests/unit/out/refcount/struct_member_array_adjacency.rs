extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, VaArg, FnPtrArg)]
pub struct pair {
    #[offset(0)]
    pub a: Value<Box<[i32]>>,
    #[offset(16)]
    pub b: Value<Box<[i32]>>,
}
impl Clone for pair {
    fn clone(&self) -> Self {
        Self {
            a: Rc::new(RefCell::new((*self.a.borrow()).clone())),
            b: Rc::new(RefCell::new((*self.b.borrow()).clone())),
        }
    }
}
impl Default for pair {
    fn default() -> Self {
        pair {
            a: Rc::new(RefCell::new((0..4).map(|_| 0_i32).collect::<Box<[i32]>>())),
            b: Rc::new(RefCell::new((0..4).map(|_| 0_i32).collect::<Box<[i32]>>())),
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
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<pair> = <Value<pair>>::default();
    assert!(
        (((({ (*s.borrow()).a.clone() }.as_pointer() as Ptr::<i32>).offset((4) as isize)
            == ({ (*s.borrow()).b.clone() }.as_pointer() as Ptr::<i32>)) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
