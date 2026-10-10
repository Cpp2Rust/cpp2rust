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
impl std::cmp::Ord for S {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_0(
                Rc::new(RefCell::new(S { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(S { v: other.v.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_0(
                Rc::new(RefCell::new(S { v: other.v.clone() })).as_pointer(),
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
            operator_eq_1(
                Rc::new(RefCell::new(S { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(S { v: other.v.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for S {}
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
impl std::cmp::Ord for V {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_8(self.clone(), other.clone()) {
                std::cmp::Ordering::Less
            } else if operator_lt_8(other.clone(), self.clone()) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for V {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for V {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_9(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for V {}
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
