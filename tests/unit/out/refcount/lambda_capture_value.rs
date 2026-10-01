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
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let factor: Value<i32> = Rc::new(RefCell::new(3));
    let scale: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let factor: Value<i32> = Rc::new(RefCell::new((*factor.borrow())));
        },
        |x: i32| -> i32 {
            let x: Value<i32> = Rc::new(RefCell::new(x));
            return ((*x.borrow()) * (*factor.borrow()));
        }
    )));
    assert!((({ (*scale.borrow()).call(4,) }) == 12));
    (*factor.borrow_mut()) = 100;
    assert!((({ (*scale.borrow()).call(4,) }) == 12));
    let slot: Value<i32> = Rc::new(RefCell::new(7));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((slot.as_pointer())));
    let read_ptr: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let p: Value<Ptr<i32>> = Rc::new(RefCell::new((*p.borrow()).clone()));
        },
        || -> i32 {
            return ((*p.borrow()).read());
        }
    )));
    (*slot.borrow_mut()) = 8;
    assert!((({ (*read_ptr.borrow()).call() }) == 8));
    let s: Value<S> = Rc::new(RefCell::new(S { x: 1, y: 2 }));
    let sum: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let s: Value<S> = Rc::new(RefCell::new((*s.borrow()).clone()));
        },
        || -> i32 {
            return ({ (*s.borrow()).x } + { (*s.borrow()).y });
        }
    )));
    (*s.borrow_mut()).x = 50;
    assert!((({ (*sum.borrow()).call() }) == 3));
    let base: Value<i32> = Rc::new(RefCell::new(10));
    let shifted: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let y: Value<i32> = Rc::new(RefCell::new(((*base.borrow()) + 1)));
        },
        |x: i32| -> i32 {
            let x: Value<i32> = Rc::new(RefCell::new(x));
            return ((*x.borrow()) + (*y.borrow()));
        }
    )));
    assert!((({ (*shifted.borrow()).call(5,) }) == 16));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
