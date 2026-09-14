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
    let a: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(7)),
    }));
    let b: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(2)),
    }));
    assert!(
        ((*({ SImpl::operator_add_pconstS_const(&a.as_pointer(), b.as_pointer(),) })
            .v
            .borrow())
            == 9)
    );
    assert!(
        ((*({ SImpl::operator_sub_pconstS_const(&a.as_pointer(), b.as_pointer(),) })
            .v
            .borrow())
            == 5)
    );
    assert!(
        ((*({ SImpl::operator_mul(&a.as_pointer(), b.as_pointer(),) })
            .v
            .borrow())
            == 14)
    );
    assert!(
        ((*({ SImpl::operator_div(&a.as_pointer(), b.as_pointer(),) })
            .v
            .borrow())
            == 3)
    );
    assert!(
        ((*({ SImpl::operator_rem(&a.as_pointer(), b.as_pointer(),) })
            .v
            .borrow())
            == 1)
    );
    assert!(((*({ SImpl::operator_pos_const(&a.as_pointer(),) }).v.borrow()) == 7));
    assert!(((*({ SImpl::operator_neg_const(&a.as_pointer(),) }).v.borrow()) == -7_i32));
    assert!(
        ((*(*({ SImpl::operator_inc(&a.as_pointer(),) })
            .upgrade()
            .deref())
        .v
        .borrow())
            == 8)
    );
    assert!(
        ((*({ SImpl::operator_post_inc_i32(&a.as_pointer(), 0,) })
            .v
            .borrow())
            == 8)
    );
    assert!(((*(*a.borrow()).v.borrow()) == 9));
    assert!(
        ((*(*({ SImpl::operator_dec(&a.as_pointer(),) })
            .upgrade()
            .deref())
        .v
        .borrow())
            == 8)
    );
    assert!(
        ((*({ SImpl::operator_post_dec_i32(&a.as_pointer(), 0,) })
            .v
            .borrow())
            == 8)
    );
    assert!(((*(*a.borrow()).v.borrow()) == 7));
    assert!(
        ((*(*({ SImpl::operator_inc(&({ SImpl::operator_inc(&a.as_pointer(),) }),) })
            .upgrade()
            .deref())
        .v
        .borrow())
            == 9)
    );
    return 0;
}
pub trait SImpl {
    fn operator_add_pconstS_const(&self, o: Ptr<S>) -> S;
    fn operator_sub_pconstS_const(&self, o: Ptr<S>) -> S;
    fn operator_mul(&self, o: Ptr<S>) -> S;
    fn operator_div(&self, o: Ptr<S>) -> S;
    fn operator_rem(&self, o: Ptr<S>) -> S;
    fn operator_pos_const(&self) -> S;
    fn operator_neg_const(&self) -> S;
    fn operator_inc(&self) -> Ptr<S>;
    fn operator_post_inc_i32(&self, _a0: i32) -> S;
    fn operator_dec(&self) -> Ptr<S>;
    fn operator_post_dec_i32(&self, _a0: i32) -> S;
}
impl SImpl for Ptr<S> {
    fn operator_add_pconstS_const(&self, o: Ptr<S>) -> S {
        return S {
            v: Rc::new(RefCell::new({
                let _lhs = (*(*(*self).upgrade().deref()).v.borrow());
                _lhs + (*(*o.upgrade().deref()).v.borrow())
            })),
        };
    }
    fn operator_sub_pconstS_const(&self, o: Ptr<S>) -> S {
        return S {
            v: Rc::new(RefCell::new({
                let _lhs = (*(*(*self).upgrade().deref()).v.borrow());
                _lhs - (*(*o.upgrade().deref()).v.borrow())
            })),
        };
    }
    fn operator_mul(&self, o: Ptr<S>) -> S {
        return S {
            v: Rc::new(RefCell::new({
                let _lhs = (*(*(*self).upgrade().deref()).v.borrow());
                _lhs * (*(*o.upgrade().deref()).v.borrow())
            })),
        };
    }
    fn operator_div(&self, o: Ptr<S>) -> S {
        return S {
            v: Rc::new(RefCell::new({
                let _lhs = (*(*(*self).upgrade().deref()).v.borrow());
                _lhs / (*(*o.upgrade().deref()).v.borrow())
            })),
        };
    }
    fn operator_rem(&self, o: Ptr<S>) -> S {
        return S {
            v: Rc::new(RefCell::new({
                let _lhs = (*(*(*self).upgrade().deref()).v.borrow());
                _lhs % (*(*o.upgrade().deref()).v.borrow())
            })),
        };
    }
    fn operator_pos_const(&self) -> S {
        return S {
            v: Rc::new(RefCell::new((*(*(*self).upgrade().deref()).v.borrow()))),
        };
    }
    fn operator_neg_const(&self) -> S {
        return S {
            v: Rc::new(RefCell::new(-(*(*(*self).upgrade().deref()).v.borrow()))),
        };
    }
    fn operator_inc(&self) -> Ptr<S> {
        (*(*(*self).upgrade().deref()).v.borrow_mut()).prefix_inc();
        return (*self).clone();
    }
    fn operator_post_inc_i32(&self, _a0: i32) -> S {
        let _a0: Value<i32> = Rc::new(RefCell::new(_a0));
        let old: Value<S> = Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        (*(*(*self).upgrade().deref()).v.borrow_mut()).prefix_inc();
        return (*old.borrow()).clone();
    }
    fn operator_dec(&self) -> Ptr<S> {
        (*(*(*self).upgrade().deref()).v.borrow_mut()).prefix_dec();
        return (*self).clone();
    }
    fn operator_post_dec_i32(&self, _a0: i32) -> S {
        let _a0: Value<i32> = Rc::new(RefCell::new(_a0));
        let old: Value<S> = Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        (*(*(*self).upgrade().deref()).v.borrow_mut()).prefix_dec();
        return (*old.borrow()).clone();
    }
}
