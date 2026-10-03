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
    pub n: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S { n: 1 }));
    assert!((({ SImpl::modify_copy(&s.as_pointer(),) }) == 1101));
    assert!(({ (*s.borrow()).n } == 1));
    assert!((({ SImpl::snapshot(&s.as_pointer(),) }) == 2));
    assert!(({ (*s.borrow()).n } == 99));
    assert!((({ SImpl::mixed(&s.as_pointer(), 1,) }) == 100));
    assert!(({ (*s.borrow()).n } == 0));
    return 0;
}
pub trait SImpl {
    fn twice(&self) -> i32;
    fn modify_copy(&self) -> i32;
    fn snapshot(&self) -> i32;
    fn mixed(&self, k: i32) -> i32;
}
impl SImpl for Ptr<S> {
    fn twice(&self) -> i32 {
        return ((*self).with(|__s| __s.n) * 2);
    }
    fn modify_copy(&self) -> i32 {
        let f: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
            {
                let this_: Value<S> = Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
            },
            || -> i32 {
                {
                    let _ptr = field!(this_.as_pointer(), n);
                    _ptr.write(_ptr.read() + 10)
                };
                return this_.as_pointer().with(|__s| __s.n);
            }
        )));
        let r: Value<i32> = Rc::new(RefCell::new(({ (*f.borrow()).call() }).clone()));
        return (((*r.borrow()) * 100) + (*self).with(|__s| __s.n));
    }
    fn snapshot(&self) -> i32 {
        let f: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
            {
                let this_: Value<S> = Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
            },
            || -> i32 {
                return ({ SImpl::twice(&this_.as_pointer()) });
            }
        )));
        field!((*self), n).write(99);
        return ({ (*f.borrow()).call() }).clone();
    }
    fn mixed(&self, k: i32) -> i32 {
        let k: Value<i32> = Rc::new(RefCell::new(k));
        let f: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
            {
                let this_: Value<S> = Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
                let k: Value<i32> = Rc::new(RefCell::new((*k.borrow())));
            },
            || -> i32 {
                return (this_.as_pointer().with(|__s| __s.n) + (*k.borrow()));
            }
        )));
        field!((*self), n).write(0);
        return ({ (*f.borrow()).call() }).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
