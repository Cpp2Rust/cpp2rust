extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub struct anon_0 {
    __bytes: Value<Box<[u8]>>,
}
impl anon_0 {
    pub fn bytes(&self) -> Ptr<u8> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn aligner(&self) -> Ptr<AnyPtr> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
}
impl Clone for anon_0 {
    fn clone(&self) -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(self.__bytes.borrow().clone())),
        }
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 8]))),
        }
    }
}
impl ByteRepr for anon_0 {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        buf.copy_from_slice(&self.__bytes.borrow());
    }
    fn from_bytes(buf: &[u8]) -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(Box::from(buf))),
        }
    }
}
#[derive(Default)]
pub struct node {
    pub next: Value<Ptr<node>>,
    pub x: Value<anon_0>,
}
impl Clone for node {
    fn clone(&self) -> Self {
        Self {
            next: Rc::new(RefCell::new((*self.next.borrow()).clone())),
            x: Rc::new(RefCell::new((*self.x.borrow()).clone())),
        }
    }
}
impl ByteRepr for node {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.next.borrow()).to_bytes(&mut buf[0..8]);
        (*self.x.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            next: Rc::new(RefCell::new(<Ptr<node>>::from_bytes(&buf[0..8]))),
            x: Rc::new(RefCell::new(<anon_0>::from_bytes(&buf[8..16]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let n: Value<node> = <Value<node>>::default();
    (*(*n.borrow()).next.borrow_mut()) = Ptr::<node>::null();
    ((*(*n.borrow()).x.borrow()).bytes().reinterpret_cast::<u8>() as Ptr<u8>)
        .offset((0) as isize)
        .write(171_u8);
    assert!(
        (((((((*(*n.borrow()).x.borrow()).bytes().reinterpret_cast::<u8>() as Ptr::<u8>)
            .offset((0) as isize)
            .read()) as i32)
            == 171) as i32)
            != 0)
    );
    return 0;
}
