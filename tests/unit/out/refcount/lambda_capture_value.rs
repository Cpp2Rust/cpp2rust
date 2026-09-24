extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct S {
    pub x: Value<i32>,
    pub y: Value<i32>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
            y: Rc::new(RefCell::new((*self.y.borrow()))),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
        (*self.y.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            y: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let factor: Value<i32> = Rc::new(RefCell::new(3));
    let scale: Value<_> = Rc::new(RefCell::new(
        (|x: i32| {
            let x: Value<i32> = Rc::new(RefCell::new(x));
            return ((*x.borrow()) * (*factor.borrow()));
        }),
    ));
    assert!((({ (*scale.borrow_mut())(4,) }) == 12));
    (*factor.borrow_mut()) = 100;
    assert!((({ (*scale.borrow_mut())(4,) }) == 12));
    let slot: Value<i32> = Rc::new(RefCell::new(7));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((slot.as_pointer())));
    let read_ptr: Value<_> = Rc::new(RefCell::new(
        (|| {
            return ((*p.borrow()).read());
        }),
    ));
    (*slot.borrow_mut()) = 8;
    assert!((({ (*read_ptr.borrow_mut())() }) == 8));
    let s: Value<S> = Rc::new(RefCell::new(S {
        x: Rc::new(RefCell::new(1)),
        y: Rc::new(RefCell::new(2)),
    }));
    let sum: Value<_> = Rc::new(RefCell::new(
        (|| {
            return ((*(*s.borrow()).x.borrow()) + (*(*s.borrow()).y.borrow()));
        }),
    ));
    (*(*s.borrow()).x.borrow_mut()) = 50;
    assert!((({ (*sum.borrow_mut())() }) == 3));
    let base: Value<i32> = Rc::new(RefCell::new(10));
    let shifted: Value<_> = Rc::new(RefCell::new(
        (|x: i32| {
            let x: Value<i32> = Rc::new(RefCell::new(x));
            return ((*x.borrow()) + (*y.borrow()));
        }),
    ));
    assert!((({ (*shifted.borrow_mut())(5,) }) == 16));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
