extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct S {
    #[offset(0)]
    pub keep: i32,
    #[offset(4)]
    pub a: i32,
    #[offset(8)]
    pub b: i64,
    #[offset(16)]
    #[byte_size(5)]
    pub c: Value<Box<[u8]>>,
    #[offset(24)]
    pub last: i32,
}
impl Clone for S {
    fn clone(&self) -> Self {
        Self {
            keep: self.keep.clone(),
            a: self.a.clone(),
            b: self.b.clone(),
            c: Rc::new(RefCell::new((*self.c.borrow()).clone())),
            last: self.last.clone(),
        }
    }
}
impl Default for S {
    fn default() -> Self {
        S {
            keep: 0_i32,
            a: 0_i32,
            b: 0_i64,
            c: Rc::new(RefCell::new((0..5).map(|_| 0_u8).collect::<Box<[u8]>>())),
            last: 0_i32,
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
    field!((*p.borrow()), keep).write(7);
    {
        ((field_ptr!((*p.borrow()), a)) as Ptr<i32>)
            .to_any()
            .memset(
                (0) as u8,
                (32usize as usize).wrapping_sub((4_usize as usize)) as usize,
            );
        ((field_ptr!((*p.borrow()), a)) as Ptr<i32>).to_any()
    };
    assert!(((((*p.borrow()).with(|__s| __s.keep) == 7) as i32) != 0));
    assert!(
        (((((((((((((*p.borrow()).with(|__s| __s.a) == 0) as i32) != 0)
            && ((((*p.borrow()).with(|__s| __s.b) == 0_i64) as i32) != 0)) as i32)
            != 0)
            && ((((((array_field_ptr!((*p.borrow()), c) as Ptr::<u8>)
                .offset((4) as isize)
                .read()) as i32)
                == 0) as i32)
                != 0)) as i32)
            != 0)
            && ((((*p.borrow()).with(|__s| __s.last) == 0) as i32) != 0)) as i32)
            != 0)
    );
    field!((*p.borrow()), a).write(1);
    field!((*p.borrow()), b).write(2_i64);
    (array_field_ptr!((*p.borrow()), c) as Ptr<u8>)
        .offset((0) as isize)
        .write((('x' as i32) as u8));
    field!((*p.borrow()), last).write(3);
    {
        ((field_ptr!((*p.borrow()), b)) as Ptr<i64>)
            .to_any()
            .memset(
                (0) as u8,
                (24_usize as usize).wrapping_sub((8_usize as usize)) as usize,
            );
        ((field_ptr!((*p.borrow()), b)) as Ptr<i64>).to_any()
    };
    assert!(
        (((((((*p.borrow()).with(|__s| __s.keep) == 7) as i32) != 0)
            && ((((*p.borrow()).with(|__s| __s.a) == 1) as i32) != 0)) as i32)
            != 0)
    );
    assert!(
        (((((((*p.borrow()).with(|__s| __s.b) == 0_i64) as i32) != 0)
            && ((((((array_field_ptr!((*p.borrow()), c) as Ptr::<u8>)
                .offset((0) as isize)
                .read()) as i32)
                == 0) as i32)
                != 0)) as i32)
            != 0)
    );
    assert!(((((*p.borrow()).with(|__s| __s.last) == 3) as i32) != 0));
    libcc2rs::free_refcount((*p.borrow()).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
