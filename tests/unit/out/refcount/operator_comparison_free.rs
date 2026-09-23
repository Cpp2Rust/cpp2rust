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
pub fn operator_eq_1(a: Ptr<S>, b: Ptr<S>) -> bool {
    return {
        let _lhs = (*(*a.upgrade().deref()).v.borrow());
        _lhs == (*(*b.upgrade().deref()).v.borrow())
    };
}
pub fn operator_ne_2(a: Ptr<S>, b: Ptr<S>) -> bool {
    return {
        let _lhs = (*(*a.upgrade().deref()).v.borrow());
        _lhs != (*(*b.upgrade().deref()).v.borrow())
    };
}
pub fn operator_lt_0(a: Ptr<S>, b: Ptr<S>) -> bool {
    return {
        let _lhs = (*(*a.upgrade().deref()).v.borrow());
        _lhs < (*(*b.upgrade().deref()).v.borrow())
    };
}
pub fn operator_gt_3(a: Ptr<S>, b: Ptr<S>) -> bool {
    return {
        let _lhs = (*(*a.upgrade().deref()).v.borrow());
        _lhs > (*(*b.upgrade().deref()).v.borrow())
    };
}
pub fn operator_le_4(a: Ptr<S>, b: Ptr<S>) -> bool {
    return {
        let _lhs = (*(*a.upgrade().deref()).v.borrow());
        _lhs <= (*(*b.upgrade().deref()).v.borrow())
    };
}
pub fn operator_ge_5(a: Ptr<S>, b: Ptr<S>) -> bool {
    return {
        let _lhs = (*(*a.upgrade().deref()).v.borrow());
        _lhs >= (*(*b.upgrade().deref()).v.borrow())
    };
}
pub fn operator_lt_6(a: Ptr<S>, b: i32) -> bool {
    let b: Value<i32> = Rc::new(RefCell::new(b));
    return {
        let _lhs = (*(*a.upgrade().deref()).v.borrow());
        _lhs < (*b.borrow())
    };
}
pub fn operator_lt_7(a: i32, b: Ptr<S>) -> bool {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    return {
        let _lhs = (*a.borrow());
        _lhs < (*(*b.upgrade().deref()).v.borrow())
    };
}
#[derive(Default)]
pub struct V {
    pub v: Value<i32>,
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
impl Clone for V {
    fn clone(&self) -> Self {
        let __this: Value<V> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<V> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for V {
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
pub fn operator_eq_9(a: V, b: V) -> bool {
    let a: Value<V> = Rc::new(RefCell::new(a));
    let b: Value<V> = Rc::new(RefCell::new(b));
    return ((*(*a.borrow()).v.borrow()) == (*(*b.borrow()).v.borrow()));
}
pub fn operator_ne_10(a: V, b: V) -> bool {
    let a: Value<V> = Rc::new(RefCell::new(a));
    let b: Value<V> = Rc::new(RefCell::new(b));
    return ((*(*a.borrow()).v.borrow()) != (*(*b.borrow()).v.borrow()));
}
pub fn operator_lt_8(a: V, b: V) -> bool {
    let a: Value<V> = Rc::new(RefCell::new(a));
    let b: Value<V> = Rc::new(RefCell::new(b));
    return ((*(*a.borrow()).v.borrow()) < (*(*b.borrow()).v.borrow()));
}
pub fn operator_gt_11(a: V, b: V) -> bool {
    let a: Value<V> = Rc::new(RefCell::new(a));
    let b: Value<V> = Rc::new(RefCell::new(b));
    return ((*(*a.borrow()).v.borrow()) > (*(*b.borrow()).v.borrow()));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<V> = Rc::new(RefCell::new(V {
        v: Rc::new(RefCell::new(1)),
    }));
    let y: Value<V> = Rc::new(RefCell::new(V {
        v: Rc::new(RefCell::new(2)),
    }));
    let z: Value<V> = Rc::new(RefCell::new(V {
        v: Rc::new(RefCell::new(1)),
    }));
    assert!(
        ({
            let _a: V = (*x.borrow()).clone();
            operator_eq_9(_a, (*z.borrow()).clone())
        })
    );
    assert!(
        ({
            let _a: V = (*x.borrow()).clone();
            operator_ne_10(_a, (*y.borrow()).clone())
        })
    );
    assert!(
        ({
            let _a: V = (*x.borrow()).clone();
            operator_lt_8(_a, (*y.borrow()).clone())
        })
    );
    assert!(
        ({
            let _a: V = (*y.borrow()).clone();
            operator_gt_11(_a, (*x.borrow()).clone())
        })
    );
    assert!(
        !({
            let _a: V = (*y.borrow()).clone();
            operator_lt_8(_a, (*x.borrow()).clone())
        })
    );
    let a: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(1)),
    }));
    let b: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(2)),
    }));
    let c: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(1)),
    }));
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
