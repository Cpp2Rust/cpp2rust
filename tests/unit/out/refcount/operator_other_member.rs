extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Static {}
impl Static {
    pub fn operator_call(mut a: i32, mut b: i32) -> i32 {
        return (a * b);
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub v: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S { v: 3 }));
    let t: Value<S> = Rc::new(RefCell::new(S { v: 4 }));
    assert!((({ SImpl::operator_call_1(&s.as_pointer(),) }) == 3));
    assert!((({ SImpl::operator_call_2(&s.as_pointer(), 1,) }) == 4));
    assert!((({ SImpl::operator_call_3(&s.as_pointer(), 1, 2,) }) == 6));
    assert!(({ ({ SImpl::operator_comma(&s.as_pointer(), t.as_pointer(),) }).v } == 34));
    let mut i: i32 = ({ SImpl::to_i32(&s.as_pointer()) });
    assert!((i == 3));
    assert!(((({ SImpl::to_i32(&s.as_pointer(),) }) + 1) == 4));
    if ({ SImpl::to_bool(&s.as_pointer()) }) {
        assert!(({ SImpl::to_bool(&s.as_pointer(),) }));
    } else {
        assert!(false);
    }
    let z: Value<S> = Rc::new(RefCell::new(S { v: 0 }));
    assert!(({ SImpl::to_bool(&s.as_pointer(),) }));
    assert!(!({ SImpl::to_bool(&z.as_pointer(),) }));
    assert!(({ SImpl::to_bool(&s.as_pointer(),) }) && (!({ SImpl::to_bool(&z.as_pointer(),) })));
    let st: Value<Static> = Rc::new(RefCell::new(<Static>::default()));
    assert!((({ Static::operator_call(6, 7,) }) == 42));
    assert!((({ SImpl::operator_call_1(&Rc::new(RefCell::new(S { v: 5 })).as_pointer(),) }) == 5));
    assert!(
        (({ SImpl::operator_call_3(&Rc::new(RefCell::new(S { v: 5 })).as_pointer(), 1, 1,) }) == 7)
    );
    return 0;
}
pub trait SImpl {
    fn operator_call_1(&self) -> i32;
    fn operator_call_2(&self, a: i32) -> i32;
    fn operator_call_3(&self, a: i32, b: i32) -> i32;
    fn operator_comma(&self, o: Ptr<S>) -> S;
    fn to_i32(&self) -> i32;
    fn to_bool(&self) -> bool;
}
impl SImpl for Ptr<S> {
    fn operator_call_1(&self) -> i32 {
        return (*self).with(|__s| __s.v);
    }
    fn operator_call_2(&self, mut a: i32) -> i32 {
        return ((*self).with(|__s| __s.v) + a);
    }
    fn operator_call_3(&self, mut a: i32, mut b: i32) -> i32 {
        return (((*self).with(|__s| __s.v) + a) + b);
    }
    fn operator_comma(&self, o: Ptr<S>) -> S {
        return S {
            v: ({ ((*self).with(|__s| __s.v) * 10) } + { o.with(|__s| __s.v) }),
        };
    }
    fn to_i32(&self) -> i32 {
        return (*self).with(|__s| __s.v);
    }
    fn to_bool(&self) -> bool {
        return ((*self).with(|__s| __s.v) != 0);
    }
}
pub fn __cpp2rust_init_globals() {}
