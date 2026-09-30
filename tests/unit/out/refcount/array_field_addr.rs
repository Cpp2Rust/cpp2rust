extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, VaArg, FnPtrArg)]
pub struct S {
    #[offset(0)]
    pub before: i32,
    #[offset(4)]
    pub mask: Value<Box<[u8]>>,
    #[offset(8)]
    pub after: i32,
}
impl Clone for S {
    fn clone(&self) -> Self {
        Self {
            before: self.before.clone(),
            mask: Rc::new(RefCell::new((*self.mask.borrow()).clone())),
            after: self.after.clone(),
        }
    }
}
impl Default for S {
    fn default() -> Self {
        S {
            before: 0_i32,
            mask: Rc::new(RefCell::new((0..4).map(|_| 0_u8).collect::<Box<[u8]>>())),
            after: 0_i32,
        }
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.before.to_bytes(&mut buf[0..4]);
        (*self.mask.borrow()).to_bytes(&mut buf[4..8]);
        self.after.to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            before: <i32>::from_bytes(&buf[0..4]),
            mask: Rc::new(RefCell::new(<Box<[u8]>>::from_bytes(&buf[4..8]))),
            after: <i32>::from_bytes(&buf[8..12]),
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
    field!((*s.borrow()), before).write(1);
    {
        ((array_field_ptr!((*s.borrow()), mask) as Ptr<u8>) as Ptr<u8>)
            .to_any()
            .memset((5) as u8, ::std::mem::size_of::<[u8; 4]>() as usize);
        ((array_field_ptr!((*s.borrow()), mask) as Ptr<u8>) as Ptr<u8>).to_any()
    };
    field!((*s.borrow()), after).write(2);
    ((array_field_ptr!((*s.borrow()), mask)) as Ptr<u8>).write(7_u8);
    let out: Value<Box<[u8]>> = Rc::new(RefCell::new((0..4).map(|_| 0_u8).collect::<Box<[u8]>>()));
    {
        ((out.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any().memcpy(
            &((array_field_ptr!((*s.borrow()), mask)) as Ptr<u8>).to_any(),
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
        (((((((*s.borrow()).with(|__s| __s.before) == 1) as i32) != 0)
            && ((((*s.borrow()).with(|__s| __s.after) == 2) as i32) != 0)) as i32)
            != 0)
    );
    libcc2rs::free_refcount((*s.borrow()).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
