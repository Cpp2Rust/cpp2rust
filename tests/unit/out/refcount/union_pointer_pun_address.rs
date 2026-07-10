extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct node_a {
    pub n: Value<i32>,
}
impl Clone for node_a {
    fn clone(&self) -> Self {
        Self {
            n: Rc::new(RefCell::new((*self.n.borrow()).clone())),
        }
    }
}
impl ByteRepr for node_a {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.n.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            n: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct node_b {
    pub data: Value<AnyPtr>,
    pub next: Value<Ptr<node_b>>,
}
impl Clone for node_b {
    fn clone(&self) -> Self {
        Self {
            data: Rc::new(RefCell::new((*self.data.borrow()).clone())),
            next: Rc::new(RefCell::new((*self.next.borrow()).clone())),
        }
    }
}
impl ByteRepr for node_b {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.data.borrow()).to_bytes(&mut buf[0..8]);
        (*self.next.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            data: Rc::new(RefCell::new(<AnyPtr>::from_bytes(&buf[0..8]))),
            next: Rc::new(RefCell::new(<Ptr<node_b>>::from_bytes(&buf[8..16]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<node_a> = Rc::new(RefCell::new(node_a {
        n: Rc::new(RefCell::new(123)),
    }));
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
    };
    let ptr: Value<anon_0> = <Value<anon_0>>::default();
    (*ptr.borrow_mut()).to_a().write((a.as_pointer()));
    let out: Value<Ptr<node_b>> = Rc::new(RefCell::new(((*ptr.borrow()).to_b().read()).clone()));
    assert!(
        ((({
            let _lhs = (*out.borrow()).clone().to_any();
            _lhs == (a.as_pointer()).to_any()
        }) as i32)
            != 0)
    );
    return 0;
}
