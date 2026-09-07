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
    pub fn S(init: i32) -> Self {
        let init: Value<i32> = Rc::new(RefCell::new(init));
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*init.borrow()))),
        }));
        let this: Ptr<S> = __this.as_pointer();
        ({ SImpl::mut_method(&this) });
        (*total_0.with(Value::clone).borrow_mut()) += ({ SImpl::const_method(&this) });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub trait SImpl {
    fn const_method(&self) -> i32;
    fn mut_method(&self);
    fn destructor(&self);
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
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        let s: Value<S> = Rc::new(RefCell::new(S::S({ 3 })));
        let _dtor_s = ScopedDestructor::new(&s, |__p| __p.destructor());
        assert!(((*(*s.borrow()).v.borrow()) == 4));
        assert!(((*total_0.with(Value::clone).borrow()) == 8));
    }
    assert!(((*total_0.with(Value::clone).borrow()) == 18));
    return 0;
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
        (*total_0.with(Value::clone).borrow_mut()) += ({ SImpl::const_method(self) });
    }
}
