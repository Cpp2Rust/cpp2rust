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
        |mut x: i32| -> i32 {
            {
                return (x + 1);
            }
        },
    )));
    assert!((({ (*one.borrow()).call(1,) }) == 2));
    let three: Value<FnPtr<fn(i32, i32, i32) -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn(i32, i32, i32) -> i32>::new(
            |mut x: i32, mut y: i32, mut z: i32| -> i32 {
                {
                    return (((x * 100) + (y * 10)) + z);
                }
            },
        )));
    assert!((({ (*three.borrow()).call(1, 2, 3,) }) == 123));
    let mut k: i32 = 3;
    let mut m: i32 = 4;
    let constants: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(
        FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
            {
                return ((x + 3) + 4);
            }
        }),
    ));
    assert!((({ (*constants.borrow()).call(1,) }) == 8));
    let mut n: i32 = (k + m);
    let derived: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(
        FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
            {
                return (x + (3 + 4));
            }
        }),
    ));
    assert!((({ (*derived.borrow()).call(1,) }) == 8));
    let implicit: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(
        FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
            {
                return (x + 3);
            }
        }),
    ));
    assert!((({ (*implicit.borrow()).call(1,) }) == 4));
    let bump: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(FnPtr::<fn(i32) -> i32>::new(
        |mut x: i32| -> i32 {
            {
                x += 1;
                return x;
            }
        },
    )));
    assert!((({ (*bump.borrow()).call(1,) }) == 2));
    let through_ptr: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(
        FnPtr::<fn(i32) -> i32>::new(|x: i32| -> i32 {
            {
                let x: Value<i32> = Rc::new(RefCell::new(x));
                let mut p: Ptr<i32> = (x.as_pointer());
                p.write({ ((p.read()) * 2) });
                return (*x.borrow());
            }
        }),
    ));
    assert!((({ (*through_ptr.borrow()).call(4,) }) == 8));
    let mut seed: i32 = 5;
    let boxed: Value<i32> = Rc::new(RefCell::new(({ (*through_ptr.borrow()).call(seed) })));
    let mut boxed_ptr: Ptr<i32> = (boxed.as_pointer());
    assert!(((boxed_ptr.read()) == 10));
    assert!((seed == 5));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
