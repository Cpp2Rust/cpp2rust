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
pub fn operator_add_0(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: Rc::new(RefCell::new({
            let _lhs = (*(*a.upgrade().deref()).v.borrow());
            _lhs + (*(*b.upgrade().deref()).v.borrow())
        })),
    };
}
pub fn operator_sub_1(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: Rc::new(RefCell::new({
            let _lhs = (*(*a.upgrade().deref()).v.borrow());
            _lhs - (*(*b.upgrade().deref()).v.borrow())
        })),
    };
}
pub fn operator_mul_2(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: Rc::new(RefCell::new({
            let _lhs = (*(*a.upgrade().deref()).v.borrow());
            _lhs * (*(*b.upgrade().deref()).v.borrow())
        })),
    };
}
pub fn operator_div_3(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: Rc::new(RefCell::new({
            let _lhs = (*(*a.upgrade().deref()).v.borrow());
            _lhs / (*(*b.upgrade().deref()).v.borrow())
        })),
    };
}
pub fn operator_rem_4(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: Rc::new(RefCell::new({
            let _lhs = (*(*a.upgrade().deref()).v.borrow());
            _lhs % (*(*b.upgrade().deref()).v.borrow())
        })),
    };
}
pub fn operator_pos_5(a: Ptr<S>) -> S {
    return S {
        v: Rc::new(RefCell::new((*(*a.upgrade().deref()).v.borrow()))),
    };
}
pub fn operator_neg_6(a: Ptr<S>) -> S {
    return S {
        v: Rc::new(RefCell::new(-(*(*a.upgrade().deref()).v.borrow()))),
    };
}
pub fn operator_inc_7(a: Ptr<S>) -> Ptr<S> {
    (*(*a.upgrade().deref()).v.borrow_mut()).prefix_inc();
    return (a).clone();
}
pub fn operator_post_inc_8(a: Ptr<S>, _: i32) -> S {
    let old: Value<S> = Rc::new(RefCell::new((*a.upgrade().deref()).clone()));
    (*(*a.upgrade().deref()).v.borrow_mut()).prefix_inc();
    return (*old.borrow_mut()).clone();
}
pub fn operator_dec_9(a: Ptr<S>) -> Ptr<S> {
    (*(*a.upgrade().deref()).v.borrow_mut()).prefix_dec();
    return (a).clone();
}
pub fn operator_post_dec_10(a: Ptr<S>, _: i32) -> S {
    let old: Value<S> = Rc::new(RefCell::new((*a.upgrade().deref()).clone()));
    (*(*a.upgrade().deref()).v.borrow_mut()).prefix_dec();
    return (*old.borrow_mut()).clone();
}
pub fn operator_add_11(a: Ptr<S>, b: i32) -> S {
    let b: Value<i32> = Rc::new(RefCell::new(b));
    return S {
        v: Rc::new(RefCell::new({
            let _lhs = (*(*a.upgrade().deref()).v.borrow());
            _lhs + (*b.borrow())
        })),
    };
}
pub fn operator_add_12(a: i32, b: Ptr<S>) -> S {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    return S {
        v: Rc::new(RefCell::new({
            let _lhs = (*a.borrow());
            _lhs + (*(*b.upgrade().deref()).v.borrow())
        })),
    };
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
        ((*({
            let _a: Ptr<S> = a.as_pointer();
            operator_add_0(_a, b.as_pointer())
        })
        .v
        .borrow())
            == 9)
    );
    assert!(
        ((*({
            let _a: Ptr<S> = a.as_pointer();
            operator_sub_1(_a, b.as_pointer())
        })
        .v
        .borrow())
            == 5)
    );
    assert!(
        ((*({
            let _a: Ptr<S> = a.as_pointer();
            operator_mul_2(_a, b.as_pointer())
        })
        .v
        .borrow())
            == 14)
    );
    assert!(
        ((*({
            let _a: Ptr<S> = a.as_pointer();
            operator_div_3(_a, b.as_pointer())
        })
        .v
        .borrow())
            == 3)
    );
    assert!(
        ((*({
            let _a: Ptr<S> = a.as_pointer();
            operator_rem_4(_a, b.as_pointer())
        })
        .v
        .borrow())
            == 1)
    );
    assert!(
        ((*({
            let _a: Ptr<S> = a.as_pointer();
            operator_pos_5(_a)
        })
        .v
        .borrow())
            == 7)
    );
    assert!(
        ((*({
            let _a: Ptr<S> = a.as_pointer();
            operator_neg_6(_a)
        })
        .v
        .borrow())
            == -7_i32)
    );
    assert!(
        ((*(*({
            let _a: Ptr<S> = a.as_pointer();
            operator_inc_7(_a)
        })
        .upgrade()
        .deref())
        .v
        .borrow())
            == 8)
    );
    assert!(
        ((*({
            let _a: Ptr<S> = a.as_pointer();
            operator_post_inc_8(_a, 0)
        })
        .v
        .borrow())
            == 8)
    );
    assert!(((*(*a.borrow()).v.borrow()) == 9));
    assert!(
        ((*(*({
            let _a: Ptr<S> = a.as_pointer();
            operator_dec_9(_a)
        })
        .upgrade()
        .deref())
        .v
        .borrow())
            == 8)
    );
    assert!(
        ((*({
            let _a: Ptr<S> = a.as_pointer();
            operator_post_dec_10(_a, 0)
        })
        .v
        .borrow())
            == 8)
    );
    assert!(((*(*a.borrow()).v.borrow()) == 7));
    assert!(
        ((*({
            let _a: Ptr<S> = a.as_pointer();
            operator_add_11(_a, 1)
        })
        .v
        .borrow())
            == 8)
    );
    assert!(((*({ operator_add_12(1, a.as_pointer(),) }).v.borrow()) == 8));
    return 0;
}
