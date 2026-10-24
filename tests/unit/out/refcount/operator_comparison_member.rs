extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub v: i32,
}
impl CppLess for S {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        SImpl::operator_lt_3(__lhs, __rhs.clone())
    }
}
impl CppEq for S {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        SImpl::operator_eq(__lhs, __rhs.clone())
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<S> = Rc::new(RefCell::new(S { v: 1 }));
    let b: Value<S> = Rc::new(RefCell::new(S { v: 2 }));
    let c: Value<S> = Rc::new(RefCell::new(S { v: 1 }));
    assert!(({ SImpl::operator_eq(&a.as_pointer(), c.as_pointer(),) }));
    assert!(({ SImpl::operator_ne(&a.as_pointer(), b.as_pointer(),) }));
    assert!(({ SImpl::operator_lt_3(&a.as_pointer(), b.as_pointer(),) }));
    assert!(({ SImpl::operator_gt(&b.as_pointer(), a.as_pointer(),) }));
    assert!(({ SImpl::operator_le(&a.as_pointer(), c.as_pointer(),) }));
    assert!(({ SImpl::operator_ge(&a.as_pointer(), c.as_pointer(),) }));
    assert!(!({ SImpl::operator_lt_3(&b.as_pointer(), a.as_pointer(),) }));
    assert!(({ SImpl::operator_lt_7(&a.as_pointer(), 5,) }));
    return 0;
}
pub trait SImpl {
    fn operator_eq(&self, o: Ptr<S>) -> bool;
    fn operator_ne(&self, o: Ptr<S>) -> bool;
    fn operator_lt_3(&self, o: Ptr<S>) -> bool;
    fn operator_gt(&self, o: Ptr<S>) -> bool;
    fn operator_le(&self, o: Ptr<S>) -> bool;
    fn operator_ge(&self, o: Ptr<S>) -> bool;
    fn operator_lt_7(&self, o: i32) -> bool;
}
impl SImpl for Ptr<S> {
    fn operator_eq(&self, o: Ptr<S>) -> bool {
        return ({ (*self).with(|__s| __s.v) } == { o.with(|__s| __s.v) });
    }
    fn operator_ne(&self, o: Ptr<S>) -> bool {
        return ({ (*self).with(|__s| __s.v) } != { o.with(|__s| __s.v) });
    }
    fn operator_lt_3(&self, o: Ptr<S>) -> bool {
        return ({ (*self).with(|__s| __s.v) } < { o.with(|__s| __s.v) });
    }
    fn operator_gt(&self, o: Ptr<S>) -> bool {
        return ({ (*self).with(|__s| __s.v) } > { o.with(|__s| __s.v) });
    }
    fn operator_le(&self, o: Ptr<S>) -> bool {
        return ({ (*self).with(|__s| __s.v) } <= { o.with(|__s| __s.v) });
    }
    fn operator_ge(&self, o: Ptr<S>) -> bool {
        return ({ (*self).with(|__s| __s.v) } >= { o.with(|__s| __s.v) });
    }
    fn operator_lt_7(&self, mut o: i32) -> bool {
        return ((*self).with(|__s| __s.v) < o);
    }
}
pub fn __cpp2rust_init_globals() {}
