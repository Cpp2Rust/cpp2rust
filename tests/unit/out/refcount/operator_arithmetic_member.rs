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
    let a: Value<S> = Rc::new(RefCell::new(S { v: 7 }));
    let b: Value<S> = Rc::new(RefCell::new(S { v: 2 }));
    assert!(({ ({ SImpl::operator_add_1(&a.as_pointer(), b.as_pointer(),) }).v } == 9));
    assert!(({ ({ SImpl::operator_sub_2(&a.as_pointer(), b.as_pointer(),) }).v } == 5));
    assert!(({ ({ SImpl::operator_mul(&a.as_pointer(), b.as_pointer(),) }).v } == 14));
    assert!(({ ({ SImpl::operator_div(&a.as_pointer(), b.as_pointer(),) }).v } == 3));
    assert!(({ ({ SImpl::operator_rem(&a.as_pointer(), b.as_pointer(),) }).v } == 1));
    assert!(({ ({ SImpl::operator_pos_6(&a.as_pointer(),) }).v } == 7));
    assert!(({ ({ SImpl::operator_neg_7(&a.as_pointer(),) }).v } == -7_i32));
    assert!((({ SImpl::operator_inc_8(&a.as_pointer(),) }).with(|__s: &S| __s.v) == 8));
    assert!(({ ({ SImpl::operator_post_inc_9(&a.as_pointer(), 0,) }).v } == 8));
    assert!(({ (*a.borrow()).v } == 9));
    assert!((({ SImpl::operator_dec_10(&a.as_pointer(),) }).with(|__s: &S| __s.v) == 8));
    assert!(({ ({ SImpl::operator_post_dec_11(&a.as_pointer(), 0,) }).v } == 8));
    assert!(({ (*a.borrow()).v } == 7));
    assert!(
        (({ SImpl::operator_inc_8(&({ SImpl::operator_inc_8(&a.as_pointer(),) }),) })
            .with(|__s: &S| __s.v)
            == 9)
    );
    assert!(
        ({
            ({
                let _o: Value<S> = Rc::new(RefCell::new(S { v: 4 }));
                SImpl::operator_add_1(
                    &Rc::new(RefCell::new(S { v: 3 })).as_pointer(),
                    _o.as_pointer(),
                )
            })
            .v
        } == 7)
    );
    return 0;
}
pub trait SImpl {
    fn operator_add_1(&self, o: Ptr<S>) -> S;
    fn operator_sub_2(&self, o: Ptr<S>) -> S;
    fn operator_mul(&self, o: Ptr<S>) -> S;
    fn operator_div(&self, o: Ptr<S>) -> S;
    fn operator_rem(&self, o: Ptr<S>) -> S;
    fn operator_pos_6(&self) -> S;
    fn operator_neg_7(&self) -> S;
    fn operator_inc_8(&self) -> Ptr<S>;
    fn operator_post_inc_9(&self, _a0: i32) -> S;
    fn operator_dec_10(&self) -> Ptr<S>;
    fn operator_post_dec_11(&self, _a0: i32) -> S;
}
impl SImpl for Ptr<S> {
    fn operator_add_1(&self, o: Ptr<S>) -> S {
        return S {
            v: {
                let _lhs = (*self).with(|__s: &S| __s.v);
                _lhs + o.with(|__s: &S| __s.v)
            },
        };
    }
    fn operator_sub_2(&self, o: Ptr<S>) -> S {
        return S {
            v: {
                let _lhs = (*self).with(|__s: &S| __s.v);
                _lhs - o.with(|__s: &S| __s.v)
            },
        };
    }
    fn operator_mul(&self, o: Ptr<S>) -> S {
        return S {
            v: {
                let _lhs = (*self).with(|__s: &S| __s.v);
                _lhs * o.with(|__s: &S| __s.v)
            },
        };
    }
    fn operator_div(&self, o: Ptr<S>) -> S {
        return S {
            v: {
                let _lhs = (*self).with(|__s: &S| __s.v);
                _lhs / o.with(|__s: &S| __s.v)
            },
        };
    }
    fn operator_rem(&self, o: Ptr<S>) -> S {
        return S {
            v: {
                let _lhs = (*self).with(|__s: &S| __s.v);
                _lhs % o.with(|__s: &S| __s.v)
            },
        };
    }
    fn operator_pos_6(&self) -> S {
        return S {
            v: { (*self).with(|__s: &S| __s.v) },
        };
    }
    fn operator_neg_7(&self) -> S {
        return S {
            v: { -(*self).with(|__s: &S| __s.v) },
        };
    }
    fn operator_inc_8(&self) -> Ptr<S> {
        (*self).with_mut(|__s: &mut S| __s.v.prefix_inc());
        return (*self).clone();
    }
    fn operator_post_inc_9(&self, _a0: i32) -> S {
        let _a0: Value<i32> = Rc::new(RefCell::new(_a0));
        let old: Value<S> = Rc::new(RefCell::new((*self).read()));
        (*self).with_mut(|__s: &mut S| __s.v.prefix_inc());
        return (*old.borrow()).clone();
    }
    fn operator_dec_10(&self) -> Ptr<S> {
        (*self).with_mut(|__s: &mut S| __s.v.prefix_dec());
        return (*self).clone();
    }
    fn operator_post_dec_11(&self, _a0: i32) -> S {
        let _a0: Value<i32> = Rc::new(RefCell::new(_a0));
        let old: Value<S> = Rc::new(RefCell::new((*self).read()));
        (*self).with_mut(|__s: &mut S| __s.v.prefix_dec());
        return (*old.borrow()).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
