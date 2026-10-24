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
        operator_lt_0(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for S {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_1(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_eq_1(a: Ptr<S>, b: Ptr<S>) -> bool {
    return ({ a.with(|__s| __s.v) } == { b.with(|__s| __s.v) });
}
pub fn operator_ne_2(a: Ptr<S>, b: Ptr<S>) -> bool {
    return ({ a.with(|__s| __s.v) } != { b.with(|__s| __s.v) });
}
pub fn operator_lt_0(a: Ptr<S>, b: Ptr<S>) -> bool {
    return ({ a.with(|__s| __s.v) } < { b.with(|__s| __s.v) });
}
pub fn operator_gt_3(a: Ptr<S>, b: Ptr<S>) -> bool {
    return ({ a.with(|__s| __s.v) } > { b.with(|__s| __s.v) });
}
pub fn operator_le_4(a: Ptr<S>, b: Ptr<S>) -> bool {
    return ({ a.with(|__s| __s.v) } <= { b.with(|__s| __s.v) });
}
pub fn operator_ge_5(a: Ptr<S>, b: Ptr<S>) -> bool {
    return ({ a.with(|__s| __s.v) } >= { b.with(|__s| __s.v) });
}
pub fn operator_lt_6(a: Ptr<S>, mut b: i32) -> bool {
    return ({ a.with(|__s| __s.v) } < { b });
}
pub fn operator_lt_7(mut a: i32, b: Ptr<S>) -> bool {
    return ({ a } < { b.with(|__s| __s.v) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct V {
    #[offset(0)]
    pub v: i32,
}
impl CppLess for V {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_8(__lhs.read(), __rhs.read())
    }
}
impl CppEq for V {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_9(__lhs.read(), __rhs.read())
    }
}
pub fn operator_eq_9(mut a: V, mut b: V) -> bool {
    return (a.v == b.v);
}
pub fn operator_ne_10(mut a: V, mut b: V) -> bool {
    return (a.v != b.v);
}
pub fn operator_lt_8(mut a: V, mut b: V) -> bool {
    return (a.v < b.v);
}
pub fn operator_gt_11(mut a: V, mut b: V) -> bool {
    return (a.v > b.v);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut x: V = V { v: 1 };
    let mut y: V = V { v: 2 };
    let mut z: V = V { v: 1 };
    assert!(
        ({
            let _a: V = (x).clone();
            operator_eq_9(_a, (z).clone())
        })
    );
    assert!(
        ({
            let _a: V = (x).clone();
            operator_ne_10(_a, (y).clone())
        })
    );
    assert!(
        ({
            let _a: V = (x).clone();
            operator_lt_8(_a, (y).clone())
        })
    );
    assert!(
        ({
            let _a: V = (y).clone();
            operator_gt_11(_a, (x).clone())
        })
    );
    assert!(
        !({
            let _a: V = (y).clone();
            operator_lt_8(_a, (x).clone())
        })
    );
    let a: Value<S> = Rc::new(RefCell::new(S { v: 1 }));
    let b: Value<S> = Rc::new(RefCell::new(S { v: 2 }));
    let c: Value<S> = Rc::new(RefCell::new(S { v: 1 }));
    assert!(
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_eq_1(_a, c.as_pointer())
        })
    );
    assert!(
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_ne_2(_a, b.as_pointer())
        })
    );
    assert!(
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_lt_0(_a, b.as_pointer())
        })
    );
    assert!(
        ({
            let _a: Ptr<S> = b.as_pointer();
            operator_gt_3(_a, a.as_pointer())
        })
    );
    assert!(
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_le_4(_a, c.as_pointer())
        })
    );
    assert!(
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_ge_5(_a, c.as_pointer())
        })
    );
    assert!(
        !({
            let _a: Ptr<S> = b.as_pointer();
            operator_lt_0(_a, a.as_pointer())
        })
    );
    assert!(
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_lt_6(_a, 5)
        })
    );
    assert!(({ operator_lt_7(0, a.as_pointer(),) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
