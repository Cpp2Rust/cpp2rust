extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct In {
    #[offset(0)]
    pub a: i16,
    #[offset(2)]
    pub b: i16,
}
impl ByteRepr for In {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.a.to_bytes(&mut buf[0..2]);
        self.b.to_bytes(&mut buf[2..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: <i16>::from_bytes(&buf[0..2]),
            b: <i16>::from_bytes(&buf[2..4]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct S {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub in_: In,
    #[offset(8)]
    pub z: i32,
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x.to_bytes(&mut buf[0..4]);
        self.in_.to_bytes(&mut buf[4..8]);
        self.z.to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: <i32>::from_bytes(&buf[0..4]),
            in_: <In>::from_bytes(&buf[4..8]),
            z: <i32>::from_bytes(&buf[8..12]),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let src: Value<S> = Rc::new(RefCell::new(S {
        x: 1,
        in_: In { a: 2_i16, b: 3_i16 },
        z: 4,
    }));
    let p: Value<Ptr<S>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount(12usize).reinterpret_cast::<S>(),
    ));
    assert!((((!((*p.borrow()).is_null())) as i32) != 0));
    {
        ((field_ptr!((*p.borrow()), x)) as Ptr<i32>)
            .to_any()
            .memcpy(&((src.as_pointer()) as Ptr<S>).to_any(), 12usize as usize);
        ((field_ptr!((*p.borrow()), x)) as Ptr<i32>).to_any()
    };
    assert!(
        (((((((((((((*p.borrow()).with(|__s| __s.x) == 1) as i32) != 0)
            && (((((*p.borrow()).with(|__s| __s.in_.a) as i32) == 2) as i32) != 0))
            as i32)
            != 0)
            && (((((*p.borrow()).with(|__s| __s.in_.b) as i32) == 3) as i32) != 0))
            as i32)
            != 0)
            && ((((*p.borrow()).with(|__s| __s.z) == 4) as i32) != 0)) as i32)
            != 0)
    );
    let n: Value<In> = Rc::new(RefCell::new(In { a: 5_i16, b: 6_i16 }));
    {
        ((field_ptr!((*p.borrow()), in_)) as Ptr<In>)
            .to_any()
            .memcpy(&((n.as_pointer()) as Ptr<In>).to_any(), 4usize as usize);
        ((field_ptr!((*p.borrow()), in_)) as Ptr<In>).to_any()
    };
    assert!(
        (((((((((((((*p.borrow()).with(|__s| __s.x) == 1) as i32) != 0)
            && (((((*p.borrow()).with(|__s| __s.in_.a) as i32) == 5) as i32) != 0))
            as i32)
            != 0)
            && (((((*p.borrow()).with(|__s| __s.in_.b) as i32) == 6) as i32) != 0))
            as i32)
            != 0)
            && ((((*p.borrow()).with(|__s| __s.z) == 4) as i32) != 0)) as i32)
            != 0)
    );
    let bz: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (field_ptr!((*p.borrow()), z)).reinterpret_cast::<u8>(),
    ));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((((*i.borrow()) < 4) as i32) != 0) {
        (*bz.borrow()).offset((*i.borrow()) as isize).write(1_u8);
        (*i.borrow_mut()).postfix_inc();
    }
    assert!(
        (((((((*p.borrow()).with(|__s| __s.z) == 16843009) as i32) != 0)
            && (((((*p.borrow()).with(|__s| __s.in_.b) as i32) == 6) as i32) != 0))
            as i32)
            != 0)
    );
    libcc2rs::free_refcount((*p.borrow()).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
