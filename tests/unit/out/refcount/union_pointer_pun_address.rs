extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct node_a {
    #[offset(0)]
    pub n: i32,
}
impl ByteRepr for node_a {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.n.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            n: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct node_b {
    #[offset(0)]
    pub data: AnyPtr,
    #[offset(8)]
    pub next: Ptr<node_b>,
}
impl ByteRepr for node_b {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.data.to_bytes(&mut buf[0..8]);
        self.next.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            data: <AnyPtr>::from_bytes(&buf[0..8]),
            next: <Ptr<node_b>>::from_bytes(&buf[8..16]),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<node_a> = Rc::new(RefCell::new(node_a { n: 123 }));
    let ptr: Value<anon_0> = <Value<anon_0>>::default();
    (*ptr.borrow_mut()).to_a().write((a.as_pointer()));
    let out: Value<Ptr<node_b>> = Rc::new(RefCell::new(((*ptr.borrow()).to_b().read()).clone()));
    assert!(
        ((({
            let _lhs = (*out.borrow()).to_any();
            _lhs == (a.as_pointer()).to_any()
        }) as i32)
            != 0)
    );
    return 0;
}
pub struct anon_0 {
    __bytes: Value<Box<[u8]>>,
}
impl anon_0 {
    pub fn to_a(&self) -> Ptr<Ptr<node_a>> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn to_b(&self) -> Ptr<Ptr<node_b>> {
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
pub fn __cpp2rust_init_globals() {}
