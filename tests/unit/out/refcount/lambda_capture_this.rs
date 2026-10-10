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
pub struct S {
    #[offset(0)]
    pub n: i32,
    #[offset(4)]
    pub step: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S { n: 0, step: 2 }));
    ({ SImpl::bump(&s.as_pointer(), 3) });
    assert!(({ (*s.borrow()).n } == 6));
    ({ SImpl::bump_via_method(&s.as_pointer(), 4) });
    assert!(({ (*s.borrow()).n } == 10));
    assert!((({ SImpl::read_scaled(&s.as_pointer(),) }) == 20));
    return 0;
}
pub trait SImpl {
    fn add(&self, k: i32);
    fn scaled(&self) -> i32;
    fn bump(&self, by: i32);
    fn bump_via_method(&self, by: i32);
    fn read_scaled(&self) -> i32;
}
impl SImpl for Ptr<S> {
    fn add(&self, mut k: i32) {
        {
            let __rhs = k;
            field!((*self), n).with_mut(|__v| *__v = *__v + __rhs)
        };
    }
    fn scaled(&self) -> i32 {
        return ((*self).with(|__s| __s.n) * (*self).with(|__s| __s.step));
    }
    fn bump(&self, mut by: i32) {
        let inc: Value<FnPtr<fn(i32)>> = Rc::new(RefCell::new(lambda!(
            {
                let this_: Value<Ptr<S>> = Rc::new(RefCell::new((*self).clone()));
            },
            |mut k: i32| {
                {
                    let __rhs = k;
                    field!((*this_.borrow()).clone(), n).with_mut(|__v| *__v = *__v + __rhs)
                };
            }
        )));
        ({ (*inc.borrow()).call(by) });
        ({ (*inc.borrow()).call(by) });
    }
    fn bump_via_method(&self, mut by: i32) {
        let inc: Value<FnPtr<fn(i32)>> = Rc::new(RefCell::new(lambda!(
            {
                let this_: Value<Ptr<S>> = Rc::new(RefCell::new((*self).clone()));
            },
            |mut k: i32| {
                ({ SImpl::add(&(*this_.borrow()).clone(), k) });
            }
        )));
        ({ (*inc.borrow()).call(by) });
    }
    fn read_scaled(&self) -> i32 {
        let get: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
            {
                let this_: Value<Ptr<S>> = Rc::new(RefCell::new((*self).clone()));
            },
            || -> i32 {
                return ({ SImpl::scaled(&(*this_.borrow()).clone()) });
            }
        )));
        return ({ (*get.borrow()).call() }).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
