extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Pair {
    pub x: Value<i32>,
    pub y: Value<i32>,
}
impl Clone for Pair {
    fn clone(&self) -> Self {
        let __this: Value<Pair> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
            y: Rc::new(RefCell::new((*self.y.borrow()))),
        }));
        let this: Ptr<Pair> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Pair {
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
#[derive(Default)]
pub struct Triple {
    pub a: Value<i32>,
    pub b: Value<i32>,
    pub p: Value<Pair>,
}
impl Clone for Triple {
    fn clone(&self) -> Self {
        let __this: Value<Triple> = Rc::new(RefCell::new(Self {
            a: Rc::new(RefCell::new((*self.a.borrow()))),
            b: Rc::new(RefCell::new((*self.b.borrow()))),
            p: Rc::new(RefCell::new((*self.p.borrow()).clone())),
        }));
        let this: Ptr<Triple> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Triple {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..4]);
        (*self.b.borrow()).to_bytes(&mut buf[4..8]);
        (*self.p.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            b: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
            p: Rc::new(RefCell::new(<Pair>::from_bytes(&buf[8..16]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p: Value<Ptr<Pair>> = Rc::new(RefCell::new(Ptr::alloc(Pair {
        x: Rc::new(RefCell::new(1)),
        y: Rc::new(RefCell::new(2)),
    })));
    let out: Value<i32> = Rc::new(RefCell::new({
        let _lhs = (*(*(*p.borrow()).upgrade().deref()).x.borrow());
        _lhs + (*(*(*p.borrow()).upgrade().deref()).y.borrow())
    }));
    (*p.borrow()).delete();
    assert!(((*out.borrow()) == 3));
    let t: Value<Triple> = Rc::new(RefCell::new(Triple {
        a: Rc::new(RefCell::new(1)),
        b: Rc::new(RefCell::new(<i32>::default())),
        p: Rc::new(RefCell::new(Pair {
            x: Rc::new(RefCell::new(<i32>::default())),
            y: Rc::new(RefCell::new(<i32>::default())),
        })),
    }));
    assert!(((*(*t.borrow()).a.borrow()) == 1));
    assert!(((*(*t.borrow()).b.borrow()) == 0));
    assert!(
        ((*(*(*t.borrow()).p.borrow()).x.borrow()) == 0)
            && ((*(*(*t.borrow()).p.borrow()).y.borrow()) == 0)
    );
    let q: Value<Ptr<Triple>> = Rc::new(RefCell::new(Ptr::alloc(Triple {
        a: Rc::new(RefCell::new(2)),
        b: Rc::new(RefCell::new(3)),
        p: Rc::new(RefCell::new(Pair {
            x: Rc::new(RefCell::new(<i32>::default())),
            y: Rc::new(RefCell::new(<i32>::default())),
        })),
    })));
    assert!(((*(*(*q.borrow()).upgrade().deref()).a.borrow()) == 2));
    assert!(((*(*(*q.borrow()).upgrade().deref()).b.borrow()) == 3));
    assert!(
        ((*(*(*(*q.borrow()).upgrade().deref()).p.borrow()).x.borrow()) == 0)
            && ((*(*(*(*q.borrow()).upgrade().deref()).p.borrow()).y.borrow()) == 0)
    );
    (*q.borrow()).delete();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
