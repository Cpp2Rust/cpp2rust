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
pub fn read_0(v: Ptr<i32>) -> i32 {
    return (v.read());
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
            return (x * (*factor.borrow()));
        },
        copy_from {
            factor: Rc::new(RefCell::new((*factor.borrow()))),
        },
        move_from {
            factor: Rc::new(RefCell::new((*factor.borrow()))),
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
        },
        copy_from {
            p: Rc::new(RefCell::new((*p.borrow()).clone())),
        },
        move_from {
            p: Rc::new(RefCell::new((*p.borrow()).clone())),
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
        },
        copy_from {
            s: Rc::new(RefCell::new((*s.borrow()).clone())),
        },
        move_from {
            s: Rc::new(RefCell::new((*s.borrow()).clone())),
        }
    )));
    (*s.borrow_mut()).x = 50;
    assert!((({ (*sum.borrow()).call() }) == 3));
    let mut base: i32 = 10;
    let shifted: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let y: Value<i32> = Rc::new(RefCell::new((base + 1)));
        },
        |x: i32| -> i32 {
            return (x + (*y.borrow()));
        },
        copy_from {
            y: Rc::new(RefCell::new((*y.borrow()))),
        },
        move_from {
            y: Rc::new(RefCell::new((*y.borrow()))),
        }
    )));
    assert!((({ (*shifted.borrow()).call(5,) }) == 16));
    let k: Value<i32> = Rc::new(RefCell::new(3));
    let by_copy: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let k: Value<i32> = Rc::new(RefCell::new((*k.borrow())));
        },
        |x: i32| -> i32 {
            return (x + 3);
        },
        copy_from {
            k: Rc::new(RefCell::new((*k.borrow()))),
        },
        move_from {
            k: Rc::new(RefCell::new((*k.borrow()))),
        }
    )));
    assert!((({ (*by_copy.borrow()).call(1,) }) == 4));
    let by_copy_used: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let k: Value<i32> = Rc::new(RefCell::new((*k.borrow())));
        },
        |x: i32| -> i32 {
            return (x + ({ read_0(k.as_pointer()) }));
        },
        copy_from {
            k: Rc::new(RefCell::new((*k.borrow()))),
        },
        move_from {
            k: Rc::new(RefCell::new((*k.borrow()))),
        }
    )));
    assert!((({ (*by_copy_used.borrow()).call(1,) }) == 4));
    let implicit_used: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let k: Value<i32> = Rc::new(RefCell::new((*k.borrow())));
        },
        |x: i32| -> i32 {
            return (x + ({ read_0(k.as_pointer()) }));
        },
        copy_from {
            k: Rc::new(RefCell::new((*k.borrow()))),
        },
        move_from {
            k: Rc::new(RefCell::new((*k.borrow()))),
        }
    )));
    assert!((({ (*implicit_used.borrow()).call(1,) }) == 4));
    let by_ref: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let k: Ptr<i32> = k.as_pointer();
        },
        |x: i32| -> i32 {
            return (x + 3);
        },
        copy_from { k: (k).clone() },
        move_from { k: (k).clone() }
    )));
    assert!((({ (*by_ref.borrow()).call(1,) }) == 4));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
