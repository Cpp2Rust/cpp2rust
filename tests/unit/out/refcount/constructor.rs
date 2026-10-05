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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub v: i32,
}
impl S {
    pub fn new(mut init: i32) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self { v: init }));
        let this: Ptr<S> = __this.as_pointer();
        ({ SImpl::mut_method(&this) });
        total_0.with(|rc| *rc.borrow_mut() += ({ SImpl::const_method(&this) }));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl Point {
    pub fn new_1(mut x: i32, mut y: i32) -> Self {
        let __this: Value<Point> = Rc::new(RefCell::new(Self { x: x, y: y }));
        let this: Ptr<Point> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_2(mut v: i32) -> Self {
        let __this: Value<Point> = Rc::new(RefCell::new(Point::new_1({ v }, { (v + 1) })));
        let this: Ptr<Point> = __this.as_pointer();
        {
            let _ptr = field!(this, y);
            _ptr.write(_ptr.read() * 10)
        };
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_3() -> Self {
        let __this: Value<Point> = Rc::new(RefCell::new(Point::new_2({ 4 })));
        let this: Ptr<Point> = __this.as_pointer();
        {
            let _ptr = field!(this, x);
            _ptr.write(_ptr.read() + 100)
        };
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Point {
    fn default() -> Self {
        { Point::new_3() }
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
        assert!(({ (*s.borrow()).v } == 4));
        assert!((total_0.with(|rc| *rc.borrow()) == 8));
    }
    assert!((total_0.with(|rc| *rc.borrow()) == 18));
    let p: Value<Point> = Rc::new(RefCell::new(Point::new_3()));
    assert!(({ (*p.borrow()).x } == 104));
    assert!(({ (*p.borrow()).y } == 50));
    let q: Value<Point> = Rc::new(RefCell::new(Point::new_2({ 7 })));
    assert!(({ (*q.borrow()).x } == 7));
    assert!(({ (*q.borrow()).y } == 80));
    return 0;
}
pub trait SImpl {
    fn const_method(&self) -> i32;
    fn mut_method(&self);
    fn destructor(&self);
}
impl SImpl for Ptr<S> {
    fn const_method(&self) -> i32 {
        return ((*self).with(|__s| __s.v) * 2);
    }
    fn mut_method(&self) {
        {
            let _ptr = field!((*self), v);
            _ptr.write(_ptr.read() + 1)
        };
    }
    fn destructor(&self) {
        ({ SImpl::mut_method(self) });
        total_0.with(|rc| *rc.borrow_mut() += ({ SImpl::const_method(self) }));
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = total_0.with(|_| ());
}
