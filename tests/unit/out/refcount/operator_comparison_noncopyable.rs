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
        let _lhs = (*(*x.upgrade().deref()).data_.borrow());
        _lhs == (*(*y.upgrade().deref()).data_.borrow())
    };
}
pub fn operator_lt_1(x: Ptr<S>, y: Ptr<S>) -> bool {
    return {
        let _lhs = (*(*x.upgrade().deref()).data_.borrow());
        _lhs < (*(*y.upgrade().deref()).data_.borrow())
    };
}
#[derive(Default)]
pub struct S {
    data_: Value<i32>,
}
impl S {
    pub fn S(data: i32) -> Self {
        let data: Value<i32> = Rc::new(RefCell::new(data));
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            data_: Rc::new(RefCell::new((*data.borrow()))),
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
impl ByteRepr for S {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.data_.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            data_: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<S> = Rc::new(RefCell::new(S::S({ 1 })));
    let b: Value<S> = Rc::new(RefCell::new(S::S({ 2 })));
    let c: Value<S> = Rc::new(RefCell::new(S::S({ 1 })));
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
