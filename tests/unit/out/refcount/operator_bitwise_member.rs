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
    pub v: Value<u32>,
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
            v: Rc::new(RefCell::new(<u32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(12_u32)),
    }));
    let b: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(10_u32)),
    }));
    assert!(((*({ SImpl::operator_bitnot(&a.as_pointer(),) }).v.borrow()) == !12_u32));
    assert!(
        ((*({ SImpl::operator_bitand(&a.as_pointer(), b.as_pointer(),) })
            .v
            .borrow())
            == 8_u32)
    );
    assert!(
        ((*({ SImpl::operator_bitor(&a.as_pointer(), b.as_pointer(),) })
            .v
            .borrow())
            == 14_u32)
    );
    assert!(
        ((*({ SImpl::operator_bitxor(&a.as_pointer(), b.as_pointer(),) })
            .v
            .borrow())
            == 6_u32)
    );
    assert!(((*({ SImpl::operator_shl(&a.as_pointer(), 2,) }).v.borrow()) == 48_u32));
    assert!(((*({ SImpl::operator_shr(&a.as_pointer(), 2,) }).v.borrow()) == 3_u32));
    return 0;
}
pub trait SImpl {
    fn operator_bitnot(&self) -> S;
    fn operator_bitand(&self, o: Ptr<S>) -> S;
    fn operator_bitor(&self, o: Ptr<S>) -> S;
    fn operator_bitxor(&self, o: Ptr<S>) -> S;
    fn operator_shl(&self, n: i32) -> S;
    fn operator_shr(&self, n: i32) -> S;
}
impl SImpl for Ptr<S> {
    fn operator_bitnot(&self) -> S {
        return S {
            v: Rc::new(RefCell::new(!(*(*(*self).upgrade().deref()).v.borrow()))),
        };
    }
    fn operator_bitand(&self, o: Ptr<S>) -> S {
        return S {
            v: Rc::new(RefCell::new({
                let _lhs = (*(*(*self).upgrade().deref()).v.borrow());
                _lhs & (*(*o.upgrade().deref()).v.borrow())
            })),
        };
    }
    fn operator_bitor(&self, o: Ptr<S>) -> S {
        return S {
            v: Rc::new(RefCell::new({
                let _lhs = (*(*(*self).upgrade().deref()).v.borrow());
                _lhs | (*(*o.upgrade().deref()).v.borrow())
            })),
        };
    }
    fn operator_bitxor(&self, o: Ptr<S>) -> S {
        return S {
            v: Rc::new(RefCell::new({
                let _lhs = (*(*(*self).upgrade().deref()).v.borrow());
                _lhs ^ (*(*o.upgrade().deref()).v.borrow())
            })),
        };
    }
    fn operator_shl(&self, n: i32) -> S {
        let n: Value<i32> = Rc::new(RefCell::new(n));
        return S {
            v: Rc::new(RefCell::new(
                ((*(*(*self).upgrade().deref()).v.borrow()) << (*n.borrow())),
            )),
        };
    }
    fn operator_shr(&self, n: i32) -> S {
        let n: Value<i32> = Rc::new(RefCell::new(n));
        return S {
            v: Rc::new(RefCell::new(
                ((*(*(*self).upgrade().deref()).v.borrow()) >> (*n.borrow())),
            )),
        };
    }
}
