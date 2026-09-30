extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg)]
pub struct S {
    pub before: Value<i32>,
    pub mask: Value<Box<[u8]>>,
    pub after: Value<i32>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        Self {
            before: Rc::new(RefCell::new((*self.before.borrow()).clone())),
            mask: Rc::new(RefCell::new((*self.mask.borrow()).clone())),
            after: Rc::new(RefCell::new((*self.after.borrow()).clone())),
        }
    }
}
impl Default for S {
    fn default() -> Self {
        S {
            before: Rc::new(RefCell::new(0_i32)),
            mask: Rc::new(RefCell::new((0..4).map(|_| 0_u8).collect::<Box<[u8]>>())),
            after: Rc::new(RefCell::new(0_i32)),
        }
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.before.borrow()).to_bytes(&mut buf[0..4]);
        (*self.mask.borrow()).to_bytes(&mut buf[4..8]);
        (*self.after.borrow()).to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            before: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            mask: Rc::new(RefCell::new(<Box<[u8]>>::from_bytes(&buf[4..8]))),
            after: Rc::new(RefCell::new(<i32>::from_bytes(&buf[8..12]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<Ptr<S>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount(12usize).reinterpret_cast::<S>(),
    ));
    assert!((((!((*s.borrow()).is_null())) as i32) != 0));
    (*(*(*s.borrow()).upgrade().deref()).before.borrow_mut()) = 1;
    {
        (((*(*s.borrow()).upgrade().deref()).mask.as_pointer() as Ptr<u8>) as Ptr<u8>)
            .to_any()
            .memset((5) as u8, ::std::mem::size_of::<[u8; 4]>() as usize);
        (((*(*s.borrow()).upgrade().deref()).mask.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any()
    };
    (*(*(*s.borrow()).upgrade().deref()).after.borrow_mut()) = 2;
    ((*(*s.borrow()).upgrade().deref()).mask.as_pointer())
        .reinterpret_cast::<u8>()
        .write(7_u8);
    let out: Value<Box<[u8]>> = Rc::new(RefCell::new((0..4).map(|_| 0_u8).collect::<Box<[u8]>>()));
    {
        ((out.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().memcpy(
            &(((*(*s.borrow()).upgrade().deref()).mask.as_pointer()) as Ptr<u8>).to_any(),
            ::std::mem::size_of::<[u8; 4]>() as usize,
        );
        ((out.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any()
    };
    assert!(
        ((((((((*out.borrow())[(0) as usize] as i32) == 7) as i32) != 0)
            && (((((*out.borrow())[(3) as usize] as i32) == 5) as i32) != 0)) as i32)
            != 0)
    );
    assert!(
        (((((((*(*(*s.borrow()).upgrade().deref()).before.borrow()) == 1) as i32) != 0)
            && ((((*(*(*s.borrow()).upgrade().deref()).after.borrow()) == 2) as i32) != 0))
            as i32)
            != 0)
    );
    libcc2rs::free_refcount((*s.borrow()).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
