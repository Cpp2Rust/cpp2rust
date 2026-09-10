extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Inner {
    pub x: Value<i32>,
}
impl Clone for Inner {
    fn clone(&self) -> Self {
        let __this: Value<Inner> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
        }));
        let this: Ptr<Inner> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Inner {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive()]
pub struct S {
    pub data: Value<Box<[i32]>>,
    pub inner: Value<Inner>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            data: Rc::new(RefCell::new((*self.data.borrow()).clone())),
            inner: Rc::new(RefCell::new((*self.inner.borrow()).clone())),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for S {
    fn default() -> Self {
        S {
            data: Rc::new(RefCell::new(
                (0..3).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
            )),
            inner: <Value<Inner>>::default(),
        }
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.data.borrow()).to_bytes(&mut buf[0..12]);
        (*self.inner.borrow()).to_bytes(&mut buf[12..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            data: Rc::new(RefCell::new(<Box<[i32]>>::from_bytes(&buf[0..12]))),
            inner: Rc::new(RefCell::new(<Inner>::from_bytes(&buf[12..16]))),
        }
    }
}
pub fn operator_deref_0(s: Ptr<S>) -> Ptr<Inner> {
    return (*s.upgrade().deref()).inner.as_pointer();
}
pub fn operator_addr_1(s: Ptr<S>) -> Ptr<i32> {
    return (((*s.upgrade().deref()).data.as_pointer() as Ptr<i32>).offset(0));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S {
        data: Rc::new(RefCell::new(Box::new([1, 2, 3]))),
        inner: Rc::new(RefCell::new(Inner {
            x: Rc::new(RefCell::new(9)),
        })),
    }));
    assert!(
        ((*(*({
            let _s: Ptr<S> = s.as_pointer();
            operator_deref_0(_s)
        })
        .upgrade()
        .deref())
        .x
        .borrow())
            == 9)
    );
    (*(*({
        let _s: Ptr<S> = s.as_pointer();
        operator_deref_0(_s)
    })
    .upgrade()
    .deref())
    .x
    .borrow_mut()) = 10;
    assert!(((*(*(*s.borrow()).inner.borrow()).x.borrow()) == 10));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(
        ({
            let _s: Ptr<S> = s.as_pointer();
            operator_addr_1(_s)
        }),
    ));
    assert!((((*p.borrow()).read()) == 1));
    (*p.borrow()).write(5);
    assert!(((*(*s.borrow()).data.borrow())[(0) as usize] == 5));
    return 0;
}
