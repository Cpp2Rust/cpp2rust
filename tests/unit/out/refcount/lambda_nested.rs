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
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(10));
    let outer: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let x: Ptr<i32> = x.as_pointer();
        },
        |y: i32| -> i32 {
            let y: Value<i32> = Rc::new(RefCell::new(y));
            let inner: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(lambda!(
                {
                    let x: Ptr<i32> = (x).clone();
                    let y: Value<i32> = Rc::new(RefCell::new((*y.borrow())));
                },
                |mut z: i32| -> i32 {
                    return (((x.read()) + (*y.borrow())) + z);
                }
            )));
            return ({ (*inner.borrow()).call(1) });
        }
    )));
    assert!((({ (*outer.borrow()).call(20,) }) == 31));
    (*x.borrow_mut()) = 100;
    assert!((({ (*outer.borrow()).call(20,) }) == 121));
    let s: Value<S> = Rc::new(RefCell::new(S { v: 5 }));
    assert!((({ SImpl::nested_this(&s.as_pointer(),) }) == 26));
    return 0;
}
pub trait SImpl {
    fn nested_this(&self) -> i32;
}
impl SImpl for Ptr<S> {
    fn nested_this(&self) -> i32 {
        let outer: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(lambda!(
            {
                let this_: Value<Ptr<S>> = Rc::new(RefCell::new((*self).clone()));
            },
            |y: i32| -> i32 {
                let y: Value<i32> = Rc::new(RefCell::new(y));
                let inner: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(lambda!(
                    {
                        let this_: Value<Ptr<S>> = Rc::new(RefCell::new((*this_.borrow()).clone()));
                        let y: Value<i32> = Rc::new(RefCell::new((*y.borrow())));
                    },
                    |mut z: i32| -> i32 {
                        return (((*this_.borrow()).clone().with(|__s| __s.v) + (*y.borrow())) + z);
                    }
                )));
                return ({ (*inner.borrow()).call(1) });
            }
        )));
        return ({ (*outer.borrow()).call(20) });
    }
}
pub fn __cpp2rust_init_globals() {}
