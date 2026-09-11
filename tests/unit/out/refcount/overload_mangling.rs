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
    pub base: Value<i32>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            base: Rc::new(RefCell::new((*self.base.borrow()))),
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
        (*self.base.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            base: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct Box {
    pub v: Value<i32>,
}
impl Clone for Box {
    fn clone(&self) -> Self {
        let __this: Value<Box> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<Box> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Box {
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
    let s: Value<S> = Rc::new(RefCell::new(S {
        base: Rc::new(RefCell::new(100)),
    }));
    assert!((({ SImpl::width_i32__char_const(&s.as_pointer(), 3,) }) == 103));
    assert!((({ SImpl::width_i32__int_const(&s.as_pointer(), 3,) }) == 112));
    assert!((({ SImpl::scale_i32__2_const(&s.as_pointer(), 5,) }) == 110));
    assert!((({ SImpl::scale_i32__3_const(&s.as_pointer(), 5,) }) == 115));
    assert!((({ SImpl::count_i32_const(&s.as_pointer(), 1,) }) == 101));
    assert!((({ SImpl::count_i32__int_long_const(&s.as_pointer(), 1,) }) == 103));
    assert!((({ SImpl::plain_i32_const(&s.as_pointer(), 1,) }) == 101));
    assert!((({ SImpl::plain_i64_const(&s.as_pointer(), 1_i64,) }) == 102));
    let b: Value<Box> = Rc::new(RefCell::new(Box {
        v: Rc::new(RefCell::new(4)),
    }));
    assert!(((*(*b.borrow()).v.borrow()) == 4));
    return 0;
}
pub trait SImpl {
    fn plain_i32_const(&self, x: i32) -> i32;
    fn plain_i64_const(&self, x: i64) -> i32;
    fn width_i32__char_const(&self, x: i32) -> i32;
    fn width_i32__int_const(&self, x: i32) -> i32;
    fn scale_i32__2_const(&self, x: i32) -> i32;
    fn scale_i32__3_const(&self, x: i32) -> i32;
    fn count_i32_const(&self, x: i32) -> i32;
    fn count_i32__int_long_const(&self, x: i32) -> i32;
}
impl SImpl for Ptr<S> {
    fn plain_i32_const(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*(*(*self).upgrade().deref()).base.borrow()) + (*x.borrow()));
    }
    fn plain_i64_const(&self, x: i64) -> i32 {
        let x: Value<i64> = Rc::new(RefCell::new(x));
        return (((*(*(*self).upgrade().deref()).base.borrow()) + ((*x.borrow()) as i32)) + 1);
    }
    fn width_i32__char_const(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*(*(*self).upgrade().deref()).base.borrow())
            + ((*x.borrow()) * (::std::mem::size_of::<u8>() as i32)));
    }
    fn width_i32__int_const(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*(*(*self).upgrade().deref()).base.borrow())
            + ((*x.borrow()) * (::std::mem::size_of::<i32>() as i32)));
    }
    fn scale_i32__2_const(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*(*(*self).upgrade().deref()).base.borrow()) + ((*x.borrow()) * 2));
    }
    fn scale_i32__3_const(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*(*(*self).upgrade().deref()).base.borrow()) + ((*x.borrow()) * 3));
    }
    fn count_i32_const(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return (((*(*(*self).upgrade().deref()).base.borrow()) + (*x.borrow())) + (0 as i32));
    }
    fn count_i32__int_long_const(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return (((*(*(*self).upgrade().deref()).base.borrow()) + (*x.borrow())) + (2 as i32));
    }
}
