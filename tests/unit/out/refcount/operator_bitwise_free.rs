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
pub fn operator_bitnot_0(a: Ptr<S>) -> S {
    return S {
        v: Rc::new(RefCell::new(!(*(*a.upgrade().deref()).v.borrow()))),
    };
}
pub fn operator_bitand_1(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: Rc::new(RefCell::new({
            let _lhs = (*(*a.upgrade().deref()).v.borrow());
            _lhs & (*(*b.upgrade().deref()).v.borrow())
        })),
    };
}
pub fn operator_bitor_2(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: Rc::new(RefCell::new({
            let _lhs = (*(*a.upgrade().deref()).v.borrow());
            _lhs | (*(*b.upgrade().deref()).v.borrow())
        })),
    };
}
pub fn operator_bitxor_3(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: Rc::new(RefCell::new({
            let _lhs = (*(*a.upgrade().deref()).v.borrow());
            _lhs ^ (*(*b.upgrade().deref()).v.borrow())
        })),
    };
}
pub fn operator_shl_4(a: Ptr<S>, n: i32) -> S {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    return S {
        v: Rc::new(RefCell::new({
            let _lhs = (*(*a.upgrade().deref()).v.borrow());
            _lhs << (*n.borrow())
        })),
    };
}
pub fn operator_shr_5(a: Ptr<S>, n: i32) -> S {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    return S {
        v: Rc::new(RefCell::new({
            let _lhs = (*(*a.upgrade().deref()).v.borrow());
            _lhs >> (*n.borrow())
        })),
    };
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
    assert!(
        ((*({
            let _a: Ptr<S> = a.as_pointer();
            operator_bitnot_0(_a)
        })
        .v
        .borrow())
            == !12_u32)
    );
    assert!(
        ((*({
            let _a: Ptr<S> = a.as_pointer();
            operator_bitand_1(_a, b.as_pointer())
        })
        .v
        .borrow())
            == 8_u32)
    );
    assert!(
        ((*({
            let _a: Ptr<S> = a.as_pointer();
            operator_bitor_2(_a, b.as_pointer())
        })
        .v
        .borrow())
            == 14_u32)
    );
    assert!(
        ((*({
            let _a: Ptr<S> = a.as_pointer();
            operator_bitxor_3(_a, b.as_pointer())
        })
        .v
        .borrow())
            == 6_u32)
    );
    assert!(
        ((*({
            let _a: Ptr<S> = a.as_pointer();
            operator_shl_4(_a, 2)
        })
        .v
        .borrow())
            == 48_u32)
    );
    assert!(
        ((*({
            let _a: Ptr<S> = a.as_pointer();
            operator_shr_5(_a, 2)
        })
        .v
        .borrow())
            == 3_u32)
    );
    return 0;
}
