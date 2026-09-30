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
    pub keep: Value<i32>,
    pub a: Value<i32>,
    pub b: Value<i64>,
    pub c: Value<Box<[u8]>>,
    pub last: Value<i32>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        Self {
            keep: Rc::new(RefCell::new((*self.keep.borrow()).clone())),
            a: Rc::new(RefCell::new((*self.a.borrow()).clone())),
            b: Rc::new(RefCell::new((*self.b.borrow()).clone())),
            c: Rc::new(RefCell::new((*self.c.borrow()).clone())),
            last: Rc::new(RefCell::new((*self.last.borrow()).clone())),
        }
    }
}
impl Default for S {
    fn default() -> Self {
        S {
            keep: Rc::new(RefCell::new(0_i32)),
            a: Rc::new(RefCell::new(0_i32)),
            b: Rc::new(RefCell::new(0_i64)),
            c: Rc::new(RefCell::new((0..5).map(|_| 0_u8).collect::<Box<[u8]>>())),
            last: Rc::new(RefCell::new(0_i32)),
        }
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.keep.borrow()).to_bytes(&mut buf[0..4]);
        (*self.a.borrow()).to_bytes(&mut buf[4..8]);
        (*self.b.borrow()).to_bytes(&mut buf[8..16]);
        (*self.c.borrow()).to_bytes(&mut buf[16..21]);
        (*self.last.borrow()).to_bytes(&mut buf[24..28]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            keep: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            a: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
            b: Rc::new(RefCell::new(<i64>::from_bytes(&buf[8..16]))),
            c: Rc::new(RefCell::new(<Box<[u8]>>::from_bytes(&buf[16..21]))),
            last: Rc::new(RefCell::new(<i32>::from_bytes(&buf[24..28]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p: Value<Ptr<S>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount(32usize).reinterpret_cast::<S>(),
    ));
    assert!((((!((*p.borrow()).is_null())) as i32) != 0));
    {
        (*p.borrow()).to_any().memset((255) as u8, 32usize as usize);
        (*p.borrow()).to_any()
    };
    (*(*(*p.borrow()).upgrade().deref()).keep.borrow_mut()) = 7;
    {
        (((*(*p.borrow()).upgrade().deref()).a.as_pointer()) as Ptr<i32>)
            .to_any()
            .memset(
                (0) as u8,
                (32usize as usize).wrapping_sub((4_usize as usize)) as usize,
            );
        (((*(*p.borrow()).upgrade().deref()).a.as_pointer()) as Ptr<i32>).to_any()
    };
    assert!(((((*(*(*p.borrow()).upgrade().deref()).keep.borrow()) == 7) as i32) != 0));
    assert!(
        (((((((((((((*(*(*p.borrow()).upgrade().deref()).a.borrow()) == 0) as i32) != 0)
            && ((((*(*(*p.borrow()).upgrade().deref()).b.borrow()) == 0_i64) as i32) != 0))
            as i32)
            != 0)
            && (((((*(*(*p.borrow()).upgrade().deref()).c.borrow())[(4) as usize] as i32) == 0)
                as i32)
                != 0)) as i32)
            != 0)
            && ((((*(*(*p.borrow()).upgrade().deref()).last.borrow()) == 0) as i32) != 0))
            as i32)
            != 0)
    );
    (*(*(*p.borrow()).upgrade().deref()).a.borrow_mut()) = 1;
    (*(*(*p.borrow()).upgrade().deref()).b.borrow_mut()) = 2_i64;
    (*(*(*p.borrow()).upgrade().deref()).c.borrow_mut())[(0) as usize] = (('x' as i32) as u8);
    (*(*(*p.borrow()).upgrade().deref()).last.borrow_mut()) = 3;
    {
        (((*(*p.borrow()).upgrade().deref()).b.as_pointer()) as Ptr<i64>)
            .to_any()
            .memset(
                (0) as u8,
                (24_usize as usize).wrapping_sub((8_usize as usize)) as usize,
            );
        (((*(*p.borrow()).upgrade().deref()).b.as_pointer()) as Ptr<i64>).to_any()
    };
    assert!(
        (((((((*(*(*p.borrow()).upgrade().deref()).keep.borrow()) == 7) as i32) != 0)
            && ((((*(*(*p.borrow()).upgrade().deref()).a.borrow()) == 1) as i32) != 0))
            as i32)
            != 0)
    );
    assert!(
        (((((((*(*(*p.borrow()).upgrade().deref()).b.borrow()) == 0_i64) as i32) != 0)
            && (((((*(*(*p.borrow()).upgrade().deref()).c.borrow())[(0) as usize] as i32) == 0)
                as i32)
                != 0)) as i32)
            != 0)
    );
    assert!(((((*(*(*p.borrow()).upgrade().deref()).last.borrow()) == 3) as i32) != 0));
    libcc2rs::free_refcount((*p.borrow()).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
