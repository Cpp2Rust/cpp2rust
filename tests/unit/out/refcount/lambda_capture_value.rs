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
    let scale: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new({
        #[derive(Record, ByteRepr)]
        #[byte_size(4)]
        struct Captures {
            #[offset(0)]
            factor: i32,
        }
        FnPtr::<fn(i32) -> i32>::with_captures(
            Captures {
                factor: (*factor.borrow()),
            },
            (|this: Ptr<Captures>, x: i32| {
                let x: Value<i32> = Rc::new(RefCell::new(x));
                return ((*x.borrow()) * this.with(|__s| __s.factor));
            }),
        )
    }));
    assert!((({ (*scale.borrow()).call(4,) }) == 12));
    (*factor.borrow_mut()) = 100;
    assert!((({ (*scale.borrow()).call(4,) }) == 12));
    let slot: Value<i32> = Rc::new(RefCell::new(7));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((slot.as_pointer())));
    let read_ptr: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new({
        #[derive(Record, ByteRepr)]
        #[byte_size(8)]
        struct Captures {
            #[offset(0)]
            #[byte_size(8)]
            p: Ptr<i32>,
        }
        FnPtr::<fn() -> i32>::with_captures(
            Captures {
                p: (*p.borrow()).clone(),
            },
            (|this: Ptr<Captures>| {
                return (this.with(|__s| __s.p.clone()).read());
            }),
        )
    }));
    (*slot.borrow_mut()) = 8;
    assert!((({ (*read_ptr.borrow()).call() }) == 8));
    let s: Value<S> = Rc::new(RefCell::new(S { x: 1, y: 2 }));
    let sum: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new({
        #[derive(Record, ByteRepr)]
        #[byte_size(8)]
        struct Captures {
            #[offset(0)]
            #[byte_size(8)]
            s: S,
        }
        FnPtr::<fn() -> i32>::with_captures(
            Captures {
                s: (*s.borrow()).clone(),
            },
            (|this: Ptr<Captures>| {
                return ({ (*this.upgrade().deref()).s.x } + { (*this.upgrade().deref()).s.y });
            }),
        )
    }));
    (*s.borrow_mut()).x = 50;
    assert!((({ (*sum.borrow()).call() }) == 3));
    let base: Value<i32> = Rc::new(RefCell::new(10));
    let shifted: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new({
        #[derive(Record, ByteRepr)]
        #[byte_size(4)]
        struct Captures {
            #[offset(0)]
            y: i32,
        }
        FnPtr::<fn(i32) -> i32>::with_captures(
            Captures {
                y: ((*base.borrow()) + 1),
            },
            (|this: Ptr<Captures>, x: i32| {
                let x: Value<i32> = Rc::new(RefCell::new(x));
                return ((*x.borrow()) + this.with(|__s| __s.y));
            }),
        )
    }));
    assert!((({ (*shifted.borrow()).call(5,) }) == 16));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
