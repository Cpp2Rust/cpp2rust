extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl CppLess for Pair {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        PairImpl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for Pair {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(PairImpl::operator_lt(__lhs, __rhs.clone()))
            && !(PairImpl::operator_lt(__rhs, __lhs.clone()))
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let pair1: Value<Pair> = Rc::new(RefCell::new(Pair { x: 1, y: 2 }));
    let pair2: Value<Pair> = Rc::new(RefCell::new(Pair { x: 1, y: 3 }));
    assert!(({ PairImpl::operator_lt(&pair1.as_pointer(), pair2.as_pointer(),) }));
    return 0;
}
pub trait PairImpl {
    fn operator_lt(&self, other: Ptr<Pair>) -> bool;
}
impl PairImpl for Ptr<Pair> {
    fn operator_lt(&self, other: Ptr<Pair>) -> bool {
        return ({ (*self).with(|__s| __s.x) } < { other.with(|__s| __s.x) })
            || (({ (*self).with(|__s| __s.x) } == { other.with(|__s| __s.x) })
                && ({ (*self).with(|__s| __s.y) } < { other.with(|__s| __s.y) }));
    }
}
pub fn __cpp2rust_init_globals() {}
