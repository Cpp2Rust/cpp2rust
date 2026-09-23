extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Overload = u32;
pub const Overload_kMutableOverload: Overload = 1;
pub const Overload_kConstOverload: Overload = 2;
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
pub fn g_0(_a0: Ptr<S>) -> Overload {
    return Overload_kMutableOverload;
}
pub fn g_1(_a0: Ptr<S>) -> Overload {
    return Overload_kConstOverload;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(7)),
    }));
    assert!(((({ SImpl::f(&s.as_pointer(),) }) as i32) == (Overload_kMutableOverload as i32)));
    assert!(((({ SImpl::f_const(&s.as_pointer(),) }) as i32) == (Overload_kConstOverload as i32)));
    assert!(((({ g_0(s.as_pointer(),) }) as i32) == (Overload_kMutableOverload as i32)));
    assert!(((({ g_1(s.as_pointer(),) }) as i32) == (Overload_kConstOverload as i32)));
    ({ SImpl::value_ref(&s.as_pointer()) }).write(9);
    assert!(((*(*s.borrow()).v.borrow()) == 9));
    assert!(((({ SImpl::value_ref_const(&s.as_pointer(),) }).read()) == 9));
    let cs: Ptr<S> = s.as_pointer();
    assert!(((({ SImpl::f_const(&cs,) }) as i32) == (Overload_kConstOverload as i32)));
    assert!(((*(*cs.upgrade().deref()).v.borrow()) == 9));
    let p: Value<Ptr<S>> = Rc::new(RefCell::new((s.as_pointer())));
    (*(*(*p.borrow()).upgrade().deref()).v.borrow_mut()) = 11;
    assert!(((*(*s.borrow()).v.borrow()) == 11));
    assert!(((({ SImpl::f(&(*p.borrow()),) }) as i32) == (Overload_kMutableOverload as i32)));
    return 0;
}
pub trait SImpl {
    fn f(&self) -> Overload;
    fn f_const(&self) -> Overload;
    fn value_ref(&self) -> Ptr<i32>;
    fn value_ref_const(&self) -> Ptr<i32>;
}
impl SImpl for Ptr<S> {
    fn f(&self) -> Overload {
        return Overload_kMutableOverload;
    }
    fn f_const(&self) -> Overload {
        return Overload_kConstOverload;
    }
    fn value_ref(&self) -> Ptr<i32> {
        return (*(*self).upgrade().deref()).v.as_pointer();
    }
    fn value_ref_const(&self) -> Ptr<i32> {
        return (*(*self).upgrade().deref()).v.as_pointer();
    }
}
pub fn __cpp2rust_init_globals() {}
