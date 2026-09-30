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
    pub a: i32,
    #[offset(4)]
    pub b: i32,
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.a.to_bytes(&mut buf[0..4]);
        self.b.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: <i32>::from_bytes(&buf[0..4]),
            b: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s1: Value<S> = Rc::new(RefCell::new(S { a: 1, b: 2 }));
    let s2: Value<S> = Rc::new(RefCell::new(S { a: 1, b: 2 }));
    let s3: Value<S> = Rc::new(RefCell::new(S { a: 1, b: 3 }));
    assert!(
        ({
            let _x: Ptr<S> = s1.as_pointer();
            operator_eq_0(_x, s2.as_pointer())
        })
    );
    assert!(
        ({
            let _x: Ptr<S> = s1.as_pointer();
            operator_ne_1(_x, s3.as_pointer())
        })
    );
    assert!(
        !({
            let _x: Ptr<S> = s1.as_pointer();
            operator_eq_0(_x, s3.as_pointer())
        })
    );
    assert!(
        ({
            let _x: Ptr<S> = s1.as_pointer();
            operator_lt_2(_x, s3.as_pointer())
        })
    );
    assert!(
        !({
            let _x: Ptr<S> = s3.as_pointer();
            operator_lt_2(_x, s1.as_pointer())
        })
    );
    assert!((({ compare_3(s1.as_pointer(), s3.as_pointer(),) }) == -1_i32));
    assert!((({ compare_3(s3.as_pointer(), s1.as_pointer(),) }) == 1));
    assert!((({ compare_3(s1.as_pointer(), s2.as_pointer(),) }) == 0));
    return 0;
}
pub fn operator_eq_0(x: Ptr<S>, y: Ptr<S>) -> bool {
    return ({
        let _lhs = x.with(|__s| __s.a);
        _lhs == y.with(|__s| __s.a)
    }) && ({
        let _lhs = x.with(|__s| __s.b);
        _lhs == y.with(|__s| __s.b)
    });
}
pub fn operator_ne_1(x: Ptr<S>, y: Ptr<S>) -> bool {
    return !({
        let _x: Ptr<S> = (x).clone();
        let _y: Ptr<S> = (y).clone();
        operator_eq_0(_x, _y)
    });
}
pub fn operator_lt_2(x: Ptr<S>, y: Ptr<S>) -> bool {
    return ({
        let _lhs = x.with(|__s| __s.a);
        _lhs < y.with(|__s| __s.a)
    }) || (({
        let _lhs = x.with(|__s| __s.a);
        _lhs == y.with(|__s| __s.a)
    }) && ({
        let _lhs = x.with(|__s| __s.b);
        _lhs < y.with(|__s| __s.b)
    }));
}
pub fn compare_3(x: Ptr<S>, y: Ptr<S>) -> i32 {
    if ({
        let _x: Ptr<S> = (x).clone();
        let _y: Ptr<S> = (y).clone();
        operator_lt_2(_x, _y)
    }) {
        return -1_i32;
    }
    if ({
        let _x: Ptr<S> = (y).clone();
        let _y: Ptr<S> = (x).clone();
        operator_lt_2(_x, _y)
    }) {
        return 1;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
