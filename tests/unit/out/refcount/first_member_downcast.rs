extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Default)]
pub struct base {
    pub kind: Value<i32>,
}
impl Clone for base {
    fn clone(&self) -> Self {
        Self {
            kind: Rc::new(RefCell::new((*self.kind.borrow()).clone())),
        }
    }
}
impl ByteRepr for base {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.kind.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            kind: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(VaArg, FnPtrArg, Default)]
pub struct derived {
    pub head: Value<base>,
    pub value: Value<usize>,
}
impl Clone for derived {
    fn clone(&self) -> Self {
        Self {
            head: Rc::new(RefCell::new((*self.head.borrow()).clone())),
            value: Rc::new(RefCell::new((*self.value.borrow()).clone())),
        }
    }
}
impl ByteRepr for derived {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.head.borrow()).to_bytes(&mut buf[0..4]);
        (*self.value.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            head: Rc::new(RefCell::new(<base>::from_bytes(&buf[0..4]))),
            value: Rc::new(RefCell::new(<usize>::from_bytes(&buf[8..16]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<Ptr<derived>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount(16usize).reinterpret_cast::<derived>(),
    ));
    assert!((((!((*d.borrow()).is_null())) as i32) != 0));
    (*(*(*(*d.borrow()).upgrade().deref()).head.borrow())
        .kind
        .borrow_mut()) = 3;
    (*(*(*d.borrow()).upgrade().deref()).value.borrow_mut()) = 7_usize;
    let b: Value<Ptr<base>> = Rc::new(RefCell::new(
        ((*(*d.borrow()).upgrade().deref()).head.as_pointer()),
    ));
    let back: Value<Ptr<derived>> =
        Rc::new(RefCell::new((*b.borrow()).reinterpret_cast::<derived>()));
    assert!(
        ((({
            let _lhs = (*back.borrow()).clone();
            _lhs == (*d.borrow()).clone()
        }) as i32)
            != 0)
    );
    assert!(((((*(*(*back.borrow()).upgrade().deref()).value.borrow()) == 7_usize) as i32) != 0));
    assert!(
        ((((*(*(*(*back.borrow()).upgrade().deref()).head.borrow())
            .kind
            .borrow())
            == 3) as i32)
            != 0)
    );
    (*(*(*back.borrow()).upgrade().deref()).value.borrow_mut()) = 8_usize;
    assert!(((((*(*(*d.borrow()).upgrade().deref()).value.borrow()) == 8_usize) as i32) != 0));
    (*(*(*b.borrow()).upgrade().deref()).kind.borrow_mut()) = 4;
    assert!(
        ((((*(*(*(*d.borrow()).upgrade().deref()).head.borrow())
            .kind
            .borrow())
            == 4) as i32)
            != 0)
    );
    libcc2rs::free_refcount((*back.borrow()).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
