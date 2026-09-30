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
            SImpl::operator_cmp(
                &Rc::new(RefCell::new(S { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(S { v: other.v.clone() })).as_pointer(),
            )
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
            SImpl::operator_eq(
                &Rc::new(RefCell::new(S { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(S { v: other.v.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for S {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<S> = Rc::new(RefCell::new(S { v: 1 }));
    let b: Value<S> = Rc::new(RefCell::new(S { v: 2 }));
    assert!(
        ({ SImpl::operator_cmp(&a.as_pointer(), b.as_pointer(),) }) == std::cmp::Ordering::Less
    );
    assert!(
        ({ SImpl::operator_cmp(&b.as_pointer(), a.as_pointer(),) }) == std::cmp::Ordering::Greater
    );
    assert!(
        ({ SImpl::operator_cmp(&a.as_pointer(), b.as_pointer(),) }) != std::cmp::Ordering::Greater
    );
    assert!(
        ({ SImpl::operator_cmp(&b.as_pointer(), a.as_pointer(),) }) != std::cmp::Ordering::Less
    );
    assert!(!({ SImpl::operator_eq(&a.as_pointer(), b.as_pointer(),) }));
    assert!(
        ({ SImpl::operator_cmp(&a.as_pointer(), b.as_pointer(),) }) == std::cmp::Ordering::Less
    );
    return 0;
}
pub trait SImpl {
    fn operator_cmp(&self, o: Ptr<S>) -> std::cmp::Ordering;
    fn operator_eq(&self, o: Ptr<S>) -> bool;
}
impl SImpl for Ptr<S> {
    fn operator_cmp(&self, o: Ptr<S>) -> std::cmp::Ordering {
        if ({ (*self).with(|__s| __s.v) } < { o.with(|__s| __s.v) }) {
            return std::cmp::Ordering::Less;
        }
        if ({ (*self).with(|__s| __s.v) } > { o.with(|__s| __s.v) }) {
            return std::cmp::Ordering::Greater;
        }
        return std::cmp::Ordering::Equal;
    }
    fn operator_eq(&self, o: Ptr<S>) -> bool {
        return ({ (*self).with(|__s| __s.v) } == { o.with(|__s| __s.v) });
    }
}
pub fn __cpp2rust_init_globals() {}
