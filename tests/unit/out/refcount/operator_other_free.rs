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
pub fn operator_comma_0(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: Rc::new(RefCell::new({
            let _lhs = ((*(*a.upgrade().deref()).v.borrow()) * 10);
            _lhs + (*(*b.upgrade().deref()).v.borrow())
        })),
    };
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(3)),
    }));
    let t: Value<S> = Rc::new(RefCell::new(S {
        v: Rc::new(RefCell::new(4)),
    }));
    assert!(
        ((*({
            let _a: Ptr<S> = s.as_pointer();
            operator_comma_0(_a, t.as_pointer())
        })
        .v
        .borrow())
            == 34)
    );
    assert!(
        ((*({
            let _a: Value<S> = Rc::new(RefCell::new(
                ({
                    let _a: Ptr<S> = s.as_pointer();
                    operator_comma_0(_a, t.as_pointer())
                }),
            ));
            let _b: Ptr<S> = s.as_pointer();
            operator_comma_0(_a.as_pointer(), _b)
        })
        .v
        .borrow())
            == 343)
    );
    return 0;
}
