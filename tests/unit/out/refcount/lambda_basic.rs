extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let zero: Value<FnPtr<fn() -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn() -> i32>::new(|| -> i32 {
            {
                return 42;
            }
        })));
    assert!((({ (*zero.borrow()).call() }) == 42));
    let one: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(FnPtr::<fn(i32) -> i32>::new(
        |x: i32| -> i32 {
            {
                let x: Value<i32> = Rc::new(RefCell::new(x));
                return ((*x.borrow()) + 1);
            }
        },
    )));
    assert!((({ (*one.borrow()).call(1,) }) == 2));
    let three: Value<FnPtr<fn(i32, i32, i32) -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn(i32, i32, i32) -> i32>::new(
            |x: i32, y: i32, z: i32| -> i32 {
                {
                    let x: Value<i32> = Rc::new(RefCell::new(x));
                    let y: Value<i32> = Rc::new(RefCell::new(y));
                    let z: Value<i32> = Rc::new(RefCell::new(z));
                    return ((((*x.borrow()) * 100) + ((*y.borrow()) * 10)) + (*z.borrow()));
                }
            },
        )));
    assert!((({ (*three.borrow()).call(1, 2, 3,) }) == 123));
    let k: Value<i32> = Rc::new(RefCell::new(3));
    let m: Value<i32> = Rc::new(RefCell::new(4));
    let constants: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(
        FnPtr::<fn(i32) -> i32>::new(|x: i32| -> i32 {
            {
                let x: Value<i32> = Rc::new(RefCell::new(x));
                return (((*x.borrow()) + 3) + 4);
            }
        }),
    ));
    assert!((({ (*constants.borrow()).call(1,) }) == 8));
    let n: Value<i32> = Rc::new(RefCell::new(((*k.borrow()) + (*m.borrow()))));
    let derived: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(
        FnPtr::<fn(i32) -> i32>::new(|x: i32| -> i32 {
            {
                let x: Value<i32> = Rc::new(RefCell::new(x));
                return ((*x.borrow()) + (3 + 4));
            }
        }),
    ));
    assert!((({ (*derived.borrow()).call(1,) }) == 8));
    let implicit: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(
        FnPtr::<fn(i32) -> i32>::new(|x: i32| -> i32 {
            {
                let x: Value<i32> = Rc::new(RefCell::new(x));
                return ((*x.borrow()) + 3);
            }
        }),
    ));
    assert!((({ (*implicit.borrow()).call(1,) }) == 4));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
