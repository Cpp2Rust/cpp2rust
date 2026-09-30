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
pub fn operator_add_0(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: {
            let _lhs = a.with(|__s| __s.v);
            _lhs + b.with(|__s| __s.v)
        },
    };
}
pub fn operator_sub_1(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: {
            let _lhs = a.with(|__s| __s.v);
            _lhs - b.with(|__s| __s.v)
        },
    };
}
pub fn operator_mul_2(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: {
            let _lhs = a.with(|__s| __s.v);
            _lhs * b.with(|__s| __s.v)
        },
    };
}
pub fn operator_div_3(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: {
            let _lhs = a.with(|__s| __s.v);
            _lhs / b.with(|__s| __s.v)
        },
    };
}
pub fn operator_rem_4(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: {
            let _lhs = a.with(|__s| __s.v);
            _lhs % b.with(|__s| __s.v)
        },
    };
}
pub fn operator_pos_5(a: Ptr<S>) -> S {
    return S {
        v: a.with(|__s| __s.v),
    };
}
pub fn operator_neg_6(a: Ptr<S>) -> S {
    return S {
        v: -a.with(|__s| __s.v),
    };
}
pub fn operator_inc_7(a: Ptr<S>) -> Ptr<S> {
    field!(a, v).with_mut(|__v| __v.prefix_inc());
    return (a).clone();
}
pub fn operator_post_inc_8(a: Ptr<S>, _a1: i32) -> S {
    let _a1: Value<i32> = Rc::new(RefCell::new(_a1));
    let old: Value<S> = Rc::new(RefCell::new((*a.upgrade().deref()).clone()));
    field!(a, v).with_mut(|__v| __v.prefix_inc());
    return (*old.borrow()).clone();
}
pub fn operator_dec_9(a: Ptr<S>) -> Ptr<S> {
    field!(a, v).with_mut(|__v| __v.prefix_dec());
    return (a).clone();
}
pub fn operator_post_dec_10(a: Ptr<S>, _a1: i32) -> S {
    let _a1: Value<i32> = Rc::new(RefCell::new(_a1));
    let old: Value<S> = Rc::new(RefCell::new((*a.upgrade().deref()).clone()));
    field!(a, v).with_mut(|__v| __v.prefix_dec());
    return (*old.borrow()).clone();
}
pub fn operator_add_11(a: Ptr<S>, b: i32) -> S {
    let b: Value<i32> = Rc::new(RefCell::new(b));
    return S {
        v: {
            let _lhs = a.with(|__s| __s.v);
            _lhs + (*b.borrow())
        },
    };
}
pub fn operator_add_12(a: i32, b: Ptr<S>) -> S {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    return S {
        v: {
            let _lhs = (*a.borrow());
            _lhs + b.with(|__s| __s.v)
        },
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<S> = Rc::new(RefCell::new(S { v: 7 }));
    let b: Value<S> = Rc::new(RefCell::new(S { v: 2 }));
    assert!(
        ({
            ({
                let _a: Ptr<S> = a.as_pointer();
                operator_add_0(_a, b.as_pointer())
            })
            .v
        } == 9)
    );
    assert!(
        ({
            ({
                let _a: Ptr<S> = a.as_pointer();
                operator_sub_1(_a, b.as_pointer())
            })
            .v
        } == 5)
    );
    assert!(
        ({
            ({
                let _a: Ptr<S> = a.as_pointer();
                operator_mul_2(_a, b.as_pointer())
            })
            .v
        } == 14)
    );
    assert!(
        ({
            ({
                let _a: Ptr<S> = a.as_pointer();
                operator_div_3(_a, b.as_pointer())
            })
            .v
        } == 3)
    );
    assert!(
        ({
            ({
                let _a: Ptr<S> = a.as_pointer();
                operator_rem_4(_a, b.as_pointer())
            })
            .v
        } == 1)
    );
    assert!(
        ({
            ({
                let _a: Ptr<S> = a.as_pointer();
                operator_pos_5(_a)
            })
            .v
        } == 7)
    );
    assert!(
        ({
            ({
                let _a: Ptr<S> = a.as_pointer();
                operator_neg_6(_a)
            })
            .v
        } == -7_i32)
    );
    assert!(
        (({
            let _a: Ptr<S> = a.as_pointer();
            operator_inc_7(_a)
        })
        .with(|__s| (__s).v)
            == 8)
    );
    assert!(
        ({
            ({
                let _a: Ptr<S> = a.as_pointer();
                operator_post_inc_8(_a, 0)
            })
            .v
        } == 8)
    );
    assert!(({ (*a.borrow()).v } == 9));
    assert!(
        (({
            let _a: Ptr<S> = a.as_pointer();
            operator_dec_9(_a)
        })
        .with(|__s| (__s).v)
            == 8)
    );
    assert!(
        ({
            ({
                let _a: Ptr<S> = a.as_pointer();
                operator_post_dec_10(_a, 0)
            })
            .v
        } == 8)
    );
    assert!(({ (*a.borrow()).v } == 7));
    assert!(
        ({
            ({
                let _a: Ptr<S> = a.as_pointer();
                operator_add_11(_a, 1)
            })
            .v
        } == 8)
    );
    assert!(({ ({ operator_add_12(1, a.as_pointer(),) }).v } == 8));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
