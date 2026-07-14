extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct record {
    pub name: Value<Ptr<u8>>,
}
impl Clone for record {
    fn clone(&self) -> Self {
        Self {
            name: Rc::new(RefCell::new((*self.name.borrow()).clone())),
        }
    }
}
impl ByteRepr for record {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.name.borrow()).to_bytes(&mut buf[0..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            name: Rc::new(RefCell::new(<Ptr<u8>>::from_bytes(&buf[0..8]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<Ptr<u8>> = Rc::new(RefCell::new(libcc2rs::strdup_refcount(
        Ptr::from_string_literal(b"hello").clone(),
    )));
    assert!((((!((*d.borrow()).is_null())) as i32) != 0));
    assert!(
        ((({
            let mut __p1 = (*d.borrow()).clone();
            let mut __p2 = Ptr::from_string_literal(b"hello").clone();
            loop {
                let __c1 = __p1.read();
                let __c2 = __p2.read();
                if __c1 != __c2 {
                    break (__c1 as i32) - (__c2 as i32);
                }
                if __c1 == 0 {
                    break 0;
                }
                __p1 += 1;
                __p2 += 1;
            }
        } == 0) as i32)
            != 0)
    );
    libcc2rs::free_refcount(((*d.borrow()).clone() as Ptr<u8>).to_any());
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"world")));
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        (('a' as i32) as u8),
        (('b' as i32) as u8),
        (('c' as i32) as u8),
        (('\0' as i32) as u8),
    ])));
    let d2: Value<Ptr<u8>> = Rc::new(RefCell::new(libcc2rs::strdup_refcount(
        (*p.borrow()).clone(),
    )));
    assert!((((!((*d2.borrow()).is_null())) as i32) != 0));
    assert!(
        ((({
            let mut __p1 = (*d2.borrow()).clone();
            let mut __p2 = (*p.borrow()).clone();
            loop {
                let __c1 = __p1.read();
                let __c2 = __p2.read();
                if __c1 != __c2 {
                    break (__c1 as i32) - (__c2 as i32);
                }
                if __c1 == 0 {
                    break 0;
                }
                __p1 += 1;
                __p2 += 1;
            }
        } == 0) as i32)
            != 0)
    );
    libcc2rs::free_refcount(((*d2.borrow()).clone() as Ptr<u8>).to_any());
    let d3: Value<Ptr<u8>> = Rc::new(RefCell::new(libcc2rs::strdup_refcount(
        (buf.as_pointer() as Ptr<u8>).clone(),
    )));
    assert!((((!((*d3.borrow()).is_null())) as i32) != 0));
    assert!(
        ((({
            let mut __p1 = (*d3.borrow()).clone();
            let mut __p2 = (buf.as_pointer() as Ptr<u8>).clone();
            loop {
                let __c1 = __p1.read();
                let __c2 = __p2.read();
                if __c1 != __c2 {
                    break (__c1 as i32) - (__c2 as i32);
                }
                if __c1 == 0 {
                    break 0;
                }
                __p1 += 1;
                __p2 += 1;
            }
        } == 0) as i32)
            != 0)
    );
    libcc2rs::free_refcount(((*d3.borrow()).clone() as Ptr<u8>).to_any());
    let d4: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::<u8>::null()));
    (*d4.borrow_mut()) = libcc2rs::strdup_refcount((*p.borrow()).clone());
    assert!((((!((*d4.borrow()).is_null())) as i32) != 0));
    assert!(
        ((({
            let mut __p1 = (*d4.borrow()).clone();
            let mut __p2 = (*p.borrow()).clone();
            loop {
                let __c1 = __p1.read();
                let __c2 = __p2.read();
                if __c1 != __c2 {
                    break (__c1 as i32) - (__c2 as i32);
                }
                if __c1 == 0 {
                    break 0;
                }
                __p1 += 1;
                __p2 += 1;
            }
        } == 0) as i32)
            != 0)
    );
    libcc2rs::free_refcount(((*d4.borrow()).clone() as Ptr<u8>).to_any());
    let rec: Value<record> = Rc::new(RefCell::new(record {
        name: Rc::new(RefCell::new(Ptr::<u8>::null())),
    }));
    let r: Value<Ptr<record>> = Rc::new(RefCell::new((rec.as_pointer())));
    (*(*(*r.borrow()).upgrade().deref()).name.borrow_mut()) =
        libcc2rs::strdup_refcount((*p.borrow()).clone());
    assert!((((!((*(*(*r.borrow()).upgrade().deref()).name.borrow()).is_null())) as i32) != 0));
    assert!(
        ((({
            let mut __p1 = (*(*(*r.borrow()).upgrade().deref()).name.borrow()).clone();
            let mut __p2 = (*p.borrow()).clone();
            loop {
                let __c1 = __p1.read();
                let __c2 = __p2.read();
                if __c1 != __c2 {
                    break (__c1 as i32) - (__c2 as i32);
                }
                if __c1 == 0 {
                    break 0;
                }
                __p1 += 1;
                __p2 += 1;
            }
        } == 0) as i32)
            != 0)
    );
    libcc2rs::free_refcount(
        ((*(*(*r.borrow()).upgrade().deref()).name.borrow()).clone() as Ptr<u8>).to_any(),
    );
    return 0;
}
