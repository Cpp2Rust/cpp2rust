extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(x: Ptr<S>, y: Ptr<S>) -> bool {
    return {
        let _lhs = x.with(|__s| __s.data_);
        _lhs == y.with(|__s| __s.data_)
    };
}
pub fn operator_lt_1(x: Ptr<S>, y: Ptr<S>) -> bool {
    return {
        let _lhs = x.with(|__s| __s.data_);
        _lhs < y.with(|__s| __s.data_)
    };
}
#[derive(Record, ByteRepr, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    data_: i32,
}
impl S {
    pub fn new(data: i32) -> Self {
        let data: Value<i32> = Rc::new(RefCell::new(data));
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            data_: (*data.borrow()),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(_a0: Ptr<S>) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            data_: { (*_a0.upgrade().deref()).data_ },
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::Ord for S {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_1(
                Rc::new(RefCell::new(S {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(S {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_1(
                Rc::new(RefCell::new(S {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(S {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
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
            operator_eq_0(
                Rc::new(RefCell::new(S {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(S {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
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
    let a: Value<S> = Rc::new(RefCell::new(S::new({ 1 })));
    let b: Value<S> = Rc::new(RefCell::new(S::new({ 2 })));
    let c: Value<S> = Rc::new(RefCell::new(S::new({ 1 })));
    assert!(
        ({
            let _x: Ptr<S> = a.as_pointer();
            operator_eq_0(_x, c.as_pointer())
        })
    );
    assert!(
        ({
            let _x: Ptr<S> = a.as_pointer();
            operator_lt_1(_x, b.as_pointer())
        })
    );
    assert!(
        !({
            let _x: Ptr<S> = b.as_pointer();
            operator_lt_1(_x, a.as_pointer())
        })
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
