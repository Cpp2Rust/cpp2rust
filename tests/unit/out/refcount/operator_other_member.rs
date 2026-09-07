extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Static {}
impl Static {
    pub fn operator_call(a: i32, b: i32) -> i32 {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        let b: Value<i32> = Rc::new(RefCell::new(b));
        return ((*a.borrow()) * (*b.borrow()));
    }
}
impl Clone for Static {
    fn clone(&self) -> Self {
        let __this: Value<Static> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<Static> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Static {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct S {
    pub v: Value<i32>,
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
    let s: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(3)),
    }));
    let t: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(4)),
    }));
    assert!((({ SImpl::operator_call_const(&s.as_pointer(),) }) == 3));
    assert!((({ SImpl::operator_call_i32_const(&s.as_pointer(), 1,) }) == 4));
    assert!((({ SImpl::operator_call_i32_i32_const(&s.as_pointer(), 1, 2,) }) == 6));
    assert!(
        ((*({ SImpl::operator_comma(&s.as_pointer(), t.as_pointer(),) })
            .v
            .borrow())
            == 34)
    );
    let i: Value<i32> = Rc::new(RefCell::new(({ SImpl::operator_int(&s.as_pointer()) })));
    assert!(((*i.borrow()) == 3));
    assert!(((({ SImpl::operator_int(&s.as_pointer(),) }) + 1) == 4));
    if ({ SImpl::operator__Bool(&s.as_pointer()) }) {
        assert!(({ SImpl::operator__Bool(&s.as_pointer(),) }));
    } else {
        assert!(false);
    }
    let z: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(0)),
    }));
    assert!(!({ SImpl::operator__Bool(&z.as_pointer(),) }));
    let st: Value<Static> = Rc::new(RefCell::new(<Static>::default()));
    assert!((({ Static::operator_call(6, 7,) }) == 42));
    return 0;
}
pub trait SImpl {
    fn operator_call_const(&self) -> i32;
    fn operator_call_i32_const(&self, a: i32) -> i32;
    fn operator_call_i32_i32_const(&self, a: i32, b: i32) -> i32;
    fn operator_comma(&self, o: Ptr<S>) -> S;
    fn operator_int(&self) -> i32;
    fn operator__Bool(&self) -> bool;
}
impl SImpl for Ptr<S> {
    fn operator_call_const(&self) -> i32 {
        return (*(*(*self).upgrade().deref()).v.borrow());
    }
    fn operator_call_i32_const(&self, a: i32) -> i32 {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        return ((*(*(*self).upgrade().deref()).v.borrow()) + (*a.borrow()));
    }
    fn operator_call_i32_i32_const(&self, a: i32, b: i32) -> i32 {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        let b: Value<i32> = Rc::new(RefCell::new(b));
        return (((*(*(*self).upgrade().deref()).v.borrow()) + (*a.borrow())) + (*b.borrow()));
    }
    fn operator_comma(&self, o: Ptr<S>) -> S {
        return S {
            v: Rc::new(RefCell::new({
                let _lhs = ((*(*(*self).upgrade().deref()).v.borrow()) * 10);
                _lhs + (*(*o.upgrade().deref()).v.borrow())
            })),
        };
    }
    fn operator_int(&self) -> i32 {
        return (*(*(*self).upgrade().deref()).v.borrow());
    }
    fn operator__Bool(&self) -> bool {
        return ((*(*(*self).upgrade().deref()).v.borrow()) != 0);
    }
}
