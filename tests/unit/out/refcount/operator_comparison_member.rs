extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct S {
    #[offset(0)]
    pub v: i32,
}
impl std::cmp::Ord for S {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if SImpl::operator_lt_3(
                &Rc::new(RefCell::new(S { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(S { v: other.v.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if SImpl::operator_lt_3(
                &Rc::new(RefCell::new(S { v: other.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(S { v: self.v.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for S {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for S {
    fn eq(&self, other: &Self) -> bool {
        {
            SImpl::operator_eq(
                &Rc::new(RefCell::new(S { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(S { v: other.v.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for S {}
impl ByteRepr for S {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <i32>::from_bytes(&buf[0..4]),
        }
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
        return {
            let _lhs = (*self).with(|__s: &S| __s.v);
            _lhs == o.with(|__s: &S| __s.v)
        };
    }
    fn operator_ne(&self, o: Ptr<S>) -> bool {
        return {
            let _lhs = (*self).with(|__s: &S| __s.v);
            _lhs != o.with(|__s: &S| __s.v)
        };
    }
    fn operator_lt_3(&self, o: Ptr<S>) -> bool {
        return {
            let _lhs = (*self).with(|__s: &S| __s.v);
            _lhs < o.with(|__s: &S| __s.v)
        };
    }
    fn operator_gt(&self, o: Ptr<S>) -> bool {
        return {
            let _lhs = (*self).with(|__s: &S| __s.v);
            _lhs > o.with(|__s: &S| __s.v)
        };
    }
    fn operator_le(&self, o: Ptr<S>) -> bool {
        return {
            let _lhs = (*self).with(|__s: &S| __s.v);
            _lhs <= o.with(|__s: &S| __s.v)
        };
    }
    fn operator_ge(&self, o: Ptr<S>) -> bool {
        return {
            let _lhs = (*self).with(|__s: &S| __s.v);
            _lhs >= o.with(|__s: &S| __s.v)
        };
    }
    fn operator_lt_7(&self, o: i32) -> bool {
        let o: Value<i32> = Rc::new(RefCell::new(o));
        return ((*self).with(|__s: &S| __s.v) < (*o.borrow()));
    }
}
pub fn __cpp2rust_init_globals() {}
