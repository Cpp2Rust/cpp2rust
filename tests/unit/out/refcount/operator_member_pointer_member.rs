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
    assert!(((({ SImpl::operator_index_i32(&s.as_pointer(), 1,) }).read()) == 2));
    ({ SImpl::operator_index_i32(&s.as_pointer(), 1) }).write(20);
    assert!(((({ SImpl::operator_index_i32(&s.as_pointer(), 1,) }).read()) == 20));
    let cs: Ptr<S> = s.as_pointer();
    assert!(((({ SImpl::operator_index_i32_const(&cs, 2,) }).read()) == 3));
    assert!(
        ((*(*({ SImpl::operator_deref(&s.as_pointer(),) })
            .upgrade()
            .deref())
        .x
        .borrow())
            == 9)
    );
    (*(*({ SImpl::operator_deref(&s.as_pointer()) })
        .upgrade()
        .deref())
    .x
    .borrow_mut()) = 10;
    assert!(
        ((*(*({ SImpl::operator_arrow(&s.as_pointer(),) })
            .upgrade()
            .deref())
        .x
        .borrow())
            == 10)
    );
    (*(*({ SImpl::operator_arrow(&s.as_pointer()) })
        .upgrade()
        .deref())
    .x
    .borrow_mut()) = 11;
    assert!(((*(*(*s.borrow()).inner.borrow()).x.borrow()) == 11));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(({ SImpl::operator_addr(&s.as_pointer()) })));
    assert!((((*p.borrow()).read()) == 1));
    (*p.borrow()).write(5);
    assert!(((*(*s.borrow()).data.borrow())[(0) as usize] == 5));
    return 0;
}
pub trait SImpl {
    fn operator_index_i32(&self, i: i32) -> Ptr<i32>;
    fn operator_index_i32_const(&self, i: i32) -> Ptr<i32>;
    fn operator_deref(&self) -> Ptr<Inner>;
    fn operator_arrow(&self) -> Ptr<Inner>;
    fn operator_addr(&self) -> Ptr<i32>;
}
impl SImpl for Ptr<S> {
    fn operator_index_i32(&self, i: i32) -> Ptr<i32> {
        let i: Value<i32> = Rc::new(RefCell::new(i));
        return ((*(*self).upgrade().deref()).data.as_pointer() as Ptr<i32>).offset((*i.borrow()));
    }
    fn operator_index_i32_const(&self, i: i32) -> Ptr<i32> {
        let i: Value<i32> = Rc::new(RefCell::new(i));
        return ((*(*self).upgrade().deref()).data.as_pointer() as Ptr<i32>).offset((*i.borrow()));
    }
    fn operator_deref(&self) -> Ptr<Inner> {
        return (*(*self).upgrade().deref()).inner.as_pointer();
    }
    fn operator_arrow(&self) -> Ptr<Inner> {
        return ((*(*self).upgrade().deref()).inner.as_pointer());
    }
    fn operator_addr(&self) -> Ptr<i32> {
        return (((*(*self).upgrade().deref()).data.as_pointer() as Ptr<i32>).offset(0));
    }
}
