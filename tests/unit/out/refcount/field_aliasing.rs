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
    pub x: Value<i32>,
    pub y: Value<i32>,
}
impl Clone for In {
    fn clone(&self) -> Self {
        Self {
            x: Rc::new(RefCell::new((*self.x.borrow()).clone())),
            y: Rc::new(RefCell::new((*self.y.borrow()).clone())),
        }
    }
}
impl ByteRepr for In {
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
#[derive(VaArg, FnPtrArg)]
pub struct S {
    pub in_: Value<In>,
    pub total: Value<i32>,
    pub n: Value<i32>,
    pub arr: Value<Box<[i32]>>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        Self {
            in_: Rc::new(RefCell::new((*self.in_.borrow()).clone())),
            total: Rc::new(RefCell::new((*self.total.borrow()).clone())),
            n: Rc::new(RefCell::new((*self.n.borrow()).clone())),
            arr: Rc::new(RefCell::new((*self.arr.borrow()).clone())),
        }
    }
}
impl Default for S {
    fn default() -> Self {
        S {
            in_: <Value<In>>::default(),
            total: Rc::new(RefCell::new(0_i32)),
            n: Rc::new(RefCell::new(0_i32)),
            arr: Rc::new(RefCell::new((0..4).map(|_| 0_i32).collect::<Box<[i32]>>())),
        }
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        32
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.in_.borrow()).to_bytes(&mut buf[0..8]);
        (*self.total.borrow()).to_bytes(&mut buf[8..12]);
        (*self.n.borrow()).to_bytes(&mut buf[12..16]);
        (*self.arr.borrow()).to_bytes(&mut buf[16..32]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            in_: Rc::new(RefCell::new(<In>::from_bytes(&buf[0..8]))),
            total: Rc::new(RefCell::new(<i32>::from_bytes(&buf[8..12]))),
            n: Rc::new(RefCell::new(<i32>::from_bytes(&buf[12..16]))),
            arr: Rc::new(RefCell::new(<Box<[i32]>>::from_bytes(&buf[16..32]))),
        }
    }
}
#[derive(VaArg, FnPtrArg, Default)]
pub struct Node {
    pub x: Value<i32>,
    pub self_: Value<Ptr<Node>>,
}
impl Clone for Node {
    fn clone(&self) -> Self {
        Self {
            x: Rc::new(RefCell::new((*self.x.borrow()).clone())),
            self_: Rc::new(RefCell::new((*self.self_.borrow()).clone())),
        }
    }
}
impl ByteRepr for Node {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
        (*self.self_.borrow()).to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            self_: Rc::new(RefCell::new(<Ptr<Node>>::from_bytes(&buf[8..16]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p: Value<Ptr<S>> = Rc::new(RefCell::new(
        libcc2rs::calloc_refcount(1_usize, 32usize).reinterpret_cast::<S>(),
    ));
    assert!((((!((*p.borrow()).is_null())) as i32) != 0));
    let q: Value<Ptr<S>> = Rc::new(RefCell::new((*p.borrow()).clone()));
    (*(*(*(*p.borrow()).upgrade().deref()).in_.borrow())
        .x
        .borrow_mut()) = 1;
    (*(*(*(*p.borrow()).upgrade().deref()).in_.borrow())
        .y
        .borrow_mut()) = 2;
    (*(*(*p.borrow()).upgrade().deref()).total.borrow_mut()) = {
        let _lhs = (*(*(*(*q.borrow()).upgrade().deref()).in_.borrow())
            .x
            .borrow());
        _lhs + (*(*(*(*q.borrow()).upgrade().deref()).in_.borrow())
            .y
            .borrow())
    };
    assert!(((((*(*(*q.borrow()).upgrade().deref()).total.borrow()) == 3) as i32) != 0));
    let ip: Value<Ptr<In>> = Rc::new(RefCell::new(
        ((*(*p.borrow()).upgrade().deref()).in_.as_pointer()),
    ));
    (*(*(*ip.borrow()).upgrade().deref()).x.borrow_mut()) =
        ((*(*(*p.borrow()).upgrade().deref()).total.borrow()) + 1);
    assert!(
        (((((((*(*(*(*q.borrow()).upgrade().deref()).in_.borrow())
            .x
            .borrow())
            == 4) as i32)
            != 0)
            && ((((*(*(*(*q.borrow()).upgrade().deref()).in_.borrow())
                .y
                .borrow())
                == 2) as i32)
                != 0)) as i32)
            != 0)
    );
    let __rhs = (*(*(*p.borrow()).upgrade().deref()).total.borrow());
    (*(*(*p.borrow()).upgrade().deref()).arr.borrow_mut())
        [(*(*(*p.borrow()).upgrade().deref()).n.borrow()) as usize] = __rhs;
    (*(*(*p.borrow()).upgrade().deref()).n.borrow_mut()) += 1;
    (*(*(*p.borrow()).upgrade().deref()).arr.borrow_mut())
        [(*(*(*p.borrow()).upgrade().deref()).n.borrow()) as usize] =
        (*(*(*(*q.borrow()).upgrade().deref()).in_.borrow())
            .x
            .borrow());
    assert!(
        ((((((((((*(*(*q.borrow()).upgrade().deref()).arr.borrow())[(0) as usize] == 3) as i32)
            != 0)
            && ((((*(*(*q.borrow()).upgrade().deref()).arr.borrow())[(1) as usize] == 4) as i32)
                != 0)) as i32)
            != 0)
            && ((((*(*(*q.borrow()).upgrade().deref()).n.borrow()) == 1) as i32) != 0))
            as i32)
            != 0)
    );
    libcc2rs::free_refcount((*p.borrow()).to_any());
    let s: Value<Node> = <Value<Node>>::default();
    (*(*s.borrow()).x.borrow_mut()) = 1;
    let __rhs = (s.as_pointer());
    (*(*s.borrow()).self_.borrow_mut()) = __rhs;
    let __rhs = ((*(*s.borrow()).x.borrow()) + 1);
    (*(*(*(*s.borrow()).self_.borrow()).upgrade().deref())
        .x
        .borrow_mut()) = __rhs;
    assert!(((((*(*s.borrow()).x.borrow()) == 2) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
