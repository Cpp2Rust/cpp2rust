extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Default)]
pub struct In {
    pub a: Value<i16>,
    pub b: Value<i16>,
}
impl Clone for In {
    fn clone(&self) -> Self {
        Self {
            a: Rc::new(RefCell::new((*self.a.borrow()).clone())),
            b: Rc::new(RefCell::new((*self.b.borrow()).clone())),
        }
    }
}
impl ByteRepr for In {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..2]);
        (*self.b.borrow()).to_bytes(&mut buf[2..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<i16>::from_bytes(&buf[0..2]))),
            b: Rc::new(RefCell::new(<i16>::from_bytes(&buf[2..4]))),
        }
    }
}
#[derive(VaArg, FnPtrArg, Default)]
pub struct S {
    pub x: Value<i32>,
    pub in_: Value<In>,
    pub z: Value<i32>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        Self {
            x: Rc::new(RefCell::new((*self.x.borrow()).clone())),
            in_: Rc::new(RefCell::new((*self.in_.borrow()).clone())),
            z: Rc::new(RefCell::new((*self.z.borrow()).clone())),
        }
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
        (*self.in_.borrow()).to_bytes(&mut buf[4..8]);
        (*self.z.borrow()).to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            in_: Rc::new(RefCell::new(<In>::from_bytes(&buf[4..8]))),
            z: Rc::new(RefCell::new(<i32>::from_bytes(&buf[8..12]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let src: Value<S> = Rc::new(RefCell::new(S {
        x: Rc::new(RefCell::new(1)),
        in_: Rc::new(RefCell::new(In {
            a: Rc::new(RefCell::new(2_i16)),
            b: Rc::new(RefCell::new(3_i16)),
        })),
        z: Rc::new(RefCell::new(4)),
    }));
    let p: Value<Ptr<S>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount(12usize).reinterpret_cast::<S>(),
    ));
    assert!((((!((*p.borrow()).is_null())) as i32) != 0));
    {
        (((*(*p.borrow()).upgrade().deref()).x.as_pointer()) as Ptr<i32>)
            .to_any()
            .memcpy(&((src.as_pointer()) as Ptr<S>).to_any(), 12usize as usize);
        (((*(*p.borrow()).upgrade().deref()).x.as_pointer()) as Ptr<i32>).to_any()
    };
    assert!(
        (((((((((((((*(*(*p.borrow()).upgrade().deref()).x.borrow()) == 1) as i32) != 0)
            && (((((*(*(*(*p.borrow()).upgrade().deref()).in_.borrow())
                .a
                .borrow()) as i32)
                == 2) as i32)
                != 0)) as i32)
            != 0)
            && (((((*(*(*(*p.borrow()).upgrade().deref()).in_.borrow())
                .b
                .borrow()) as i32)
                == 3) as i32)
                != 0)) as i32)
            != 0)
            && ((((*(*(*p.borrow()).upgrade().deref()).z.borrow()) == 4) as i32) != 0))
            as i32)
            != 0)
    );
    let n: Value<In> = Rc::new(RefCell::new(In {
        a: Rc::new(RefCell::new(5_i16)),
        b: Rc::new(RefCell::new(6_i16)),
    }));
    {
        (((*(*p.borrow()).upgrade().deref()).in_.as_pointer()) as Ptr<In>)
            .to_any()
            .memcpy(&((n.as_pointer()) as Ptr<In>).to_any(), 4usize as usize);
        (((*(*p.borrow()).upgrade().deref()).in_.as_pointer()) as Ptr<In>).to_any()
    };
    assert!(
        (((((((((((((*(*(*p.borrow()).upgrade().deref()).x.borrow()) == 1) as i32) != 0)
            && (((((*(*(*(*p.borrow()).upgrade().deref()).in_.borrow())
                .a
                .borrow()) as i32)
                == 5) as i32)
                != 0)) as i32)
            != 0)
            && (((((*(*(*(*p.borrow()).upgrade().deref()).in_.borrow())
                .b
                .borrow()) as i32)
                == 6) as i32)
                != 0)) as i32)
            != 0)
            && ((((*(*(*p.borrow()).upgrade().deref()).z.borrow()) == 4) as i32) != 0))
            as i32)
            != 0)
    );
    let bz: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ((*(*p.borrow()).upgrade().deref()).z.as_pointer()).reinterpret_cast::<u8>(),
    ));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((((*i.borrow()) < 4) as i32) != 0) {
        (*bz.borrow()).offset((*i.borrow()) as isize).write(1_u8);
        (*i.borrow_mut()).postfix_inc();
    }
    assert!(
        (((((((*(*(*p.borrow()).upgrade().deref()).z.borrow()) == 16843009) as i32) != 0)
            && (((((*(*(*(*p.borrow()).upgrade().deref()).in_.borrow())
                .b
                .borrow()) as i32)
                == 6) as i32)
                != 0)) as i32)
            != 0)
    );
    libcc2rs::free_refcount((*p.borrow()).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
