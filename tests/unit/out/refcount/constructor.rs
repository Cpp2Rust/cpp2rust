extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static total_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Default)]
pub struct S {
    pub v: Value<i32>,
}
impl S {
    pub fn new(init: i32) -> Self {
        let init: Value<i32> = Rc::new(RefCell::new(init));
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*init.borrow()))),
        }));
        let this: Ptr<S> = __this.as_pointer();
        ({ SImpl::mut_method(&this) });
        total_0.with(|rc| *rc.borrow_mut() += ({ SImpl::const_method(&this) }));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for S {
    fn clone(&self) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive()]
pub struct Point {
    pub x: Value<i32>,
    pub y: Value<i32>,
}
impl Point {
    pub fn new_1(x: i32, y: i32) -> Self {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let y: Value<i32> = Rc::new(RefCell::new(y));
        let __this: Value<Point> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*x.borrow()))),
            y: Rc::new(RefCell::new((*y.borrow()))),
        }));
        let this: Ptr<Point> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_2(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<Point> = Rc::new(RefCell::new(Point::new_1({ (*v.borrow()) }, {
            ((*v.borrow()) + 1)
        })));
        let this: Ptr<Point> = __this.as_pointer();
        (*(*this.upgrade().deref()).y.borrow_mut()) *= 10;
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_3() -> Self {
        let __this: Value<Point> = Rc::new(RefCell::new(Point::new_2({ 4 })));
        let this: Ptr<Point> = __this.as_pointer();
        (*(*this.upgrade().deref()).x.borrow_mut()) += 100;
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for Point {
    fn clone(&self) -> Self {
        let __this: Value<Point> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
            y: Rc::new(RefCell::new((*self.y.borrow()))),
        }));
        let this: Ptr<Point> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Point {
    fn default() -> Self {
        { Point::new_3() }
    }
}
impl ByteRepr for Point {
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
    {
        let s: Value<S> = Rc::new(RefCell::new(S::new({ 3 })));
        let _dtor_s = ScopedDestructor::new(&s, |__p| __p.destructor());
        assert!(((*(*s.borrow()).v.borrow()) == 4));
        assert!((total_0.with(|rc| *rc.borrow()) == 8));
    }
    assert!((total_0.with(|rc| *rc.borrow()) == 18));
    let p: Value<Point> = Rc::new(RefCell::new(Point::new_3()));
    assert!(((*(*p.borrow()).x.borrow()) == 104));
    assert!(((*(*p.borrow()).y.borrow()) == 50));
    let q: Value<Point> = Rc::new(RefCell::new(Point::new_2({ 7 })));
    assert!(((*(*q.borrow()).x.borrow()) == 7));
    assert!(((*(*q.borrow()).y.borrow()) == 80));
    return 0;
}
pub trait SImpl {
    fn const_method(&self) -> i32;
    fn mut_method(&self);
    fn destructor(&self);
}
impl SImpl for Ptr<S> {
    fn const_method(&self) -> i32 {
        return ((*(*(*self).upgrade().deref()).v.borrow()) * 2);
    }
    fn mut_method(&self) {
        (*(*(*self).upgrade().deref()).v.borrow_mut()) += 1;
    }
    fn destructor(&self) {
        ({ SImpl::mut_method(self) });
        total_0.with(|rc| *rc.borrow_mut() += ({ SImpl::const_method(self) }));
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = total_0.with(|_| ());
}
