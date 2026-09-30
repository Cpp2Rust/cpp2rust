extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Default)]
pub struct S {
    pub a: Value<i32>,
    pub b: Value<i32>,
    pub c: Value<i32>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            a: Rc::new(RefCell::new((*self.a.borrow()))),
            b: Rc::new(RefCell::new((*self.b.borrow()))),
            c: Rc::new(RefCell::new((*self.c.borrow()))),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..4]);
        (*self.b.borrow()).to_bytes(&mut buf[4..8]);
        (*self.c.borrow()).to_bytes(&mut buf[8..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            b: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
            c: Rc::new(RefCell::new(<i32>::from_bytes(&buf[8..12]))),
        }
    }
}
pub fn bump_0(s: Ptr<S>) -> i32 {
    let s: Value<Ptr<S>> = Rc::new(RefCell::new(s));
    (*(*(*s.borrow()).upgrade().deref()).b.borrow_mut()) += 10;
    return (*(*(*s.borrow()).upgrade().deref()).b.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<Ptr<S>> = Rc::new(RefCell::new(
        libcc2rs::calloc_refcount(1_usize, 12usize).reinterpret_cast::<S>(),
    ));
    assert!(!((*s.borrow()).is_null()));
    (*(*(*s.borrow()).upgrade().deref()).b.borrow_mut()) = 1;
    let __rhs = ({ bump_0((*s.borrow()).clone()) });
    (*(*(*s.borrow()).upgrade().deref()).a.borrow_mut()) = __rhs;
    assert!(((*(*(*s.borrow()).upgrade().deref()).a.borrow()) == 11));
    assert!(((*(*(*s.borrow()).upgrade().deref()).b.borrow()) == 11));
    (*(*(*s.borrow()).upgrade().deref()).a.borrow_mut()) = 1;
    (*(*(*s.borrow()).upgrade().deref()).b.borrow_mut()) = 2;
    (*(*(*s.borrow()).upgrade().deref()).c.borrow_mut()) = 0;
    if ({
        let _lhs = (*(*(*s.borrow()).upgrade().deref()).a.borrow());
        _lhs < (*(*(*s.borrow()).upgrade().deref()).b.borrow())
    }) && ((*(*(*s.borrow()).upgrade().deref()).c.borrow_mut()).postfix_inc() == 0)
    {
        (*(*(*s.borrow()).upgrade().deref()).a.borrow_mut()) = 5;
    }
    assert!(
        ((*(*(*s.borrow()).upgrade().deref()).a.borrow()) == 5)
            && ((*(*(*s.borrow()).upgrade().deref()).c.borrow()) == 1)
    );
    if ({
        let _lhs = (*(*(*s.borrow()).upgrade().deref()).a.borrow());
        _lhs < (*(*(*s.borrow()).upgrade().deref()).b.borrow())
    }) && ((*(*(*s.borrow()).upgrade().deref()).c.borrow_mut()).postfix_inc() == 0)
    {
        (*(*(*s.borrow()).upgrade().deref()).a.borrow_mut()) = 6;
    }
    assert!(
        ((*(*(*s.borrow()).upgrade().deref()).a.borrow()) == 5)
            && ((*(*(*s.borrow()).upgrade().deref()).c.borrow()) == 1)
    );
    let x: Value<i32> = Rc::new(RefCell::new({
        let _lhs = (*(*(*s.borrow()).upgrade().deref()).a.borrow());
        _lhs + ({
            (*(*(*s.borrow()).upgrade().deref()).b.borrow_mut()) = 3;
            (*(*(*s.borrow()).upgrade().deref()).b.borrow())
        })
    }));
    assert!(((*x.borrow()) == 8) && ((*(*(*s.borrow()).upgrade().deref()).b.borrow()) == 3));
    let y: Value<i32> = Rc::new(RefCell::new(0));
    (*(*(*s.borrow()).upgrade().deref()).c.borrow_mut()) = ({
        (*y.borrow_mut()) = 99;
        (*y.borrow())
    });
    assert!(((*(*(*s.borrow()).upgrade().deref()).c.borrow()) == 99) && ((*y.borrow()) == 99));
    let __rhs = ({ bump_0((*s.borrow()).clone()) });
    (*(*(*s.borrow()).upgrade().deref()).a.borrow_mut()) += __rhs;
    assert!(
        (((*(*(*s.borrow()).upgrade().deref()).a.borrow()) == 18)
            && ((*(*(*s.borrow()).upgrade().deref()).b.borrow()) == 13))
            && ((*(*(*s.borrow()).upgrade().deref()).c.borrow()) == 99)
    );
    libcc2rs::free_refcount((*s.borrow()).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
