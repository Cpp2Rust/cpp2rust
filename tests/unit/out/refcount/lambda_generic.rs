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
    let twice: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |mut x: i32| -> i32 {
            return (x + x);
        },
        |mut x: f64| -> f64 {
            return (x + x);
        }
    )));
    assert!((({ (*twice.borrow()).spec::<fn(i32) -> i32>(0).call(4,) }) == 8));
    assert!((({ (*twice.borrow()).spec::<fn(f64) -> f64>(1).call(1.5_f64,) }) == 3_f64));
    let base: Value<i32> = Rc::new(RefCell::new(10));
    let add_base: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let base: Value<i32> = Rc::new(RefCell::new((*base.borrow())));
        },
        |mut x: i32| -> i32 {
            return (x + (*base.borrow()));
        },
        |mut x: f64| -> f64 {
            return (x + ((*base.borrow()) as f64));
        }
    )));
    assert!((({ (*add_base.borrow()).spec::<fn(i32) -> i32>(0).call(5,) }) == 15));
    assert!(
        (({
            (*add_base.borrow()).spec::<fn(f64) -> f64>(1).call(2.5_f64)
        }) == 12.5_f64)
    );
    let total: Value<i32> = Rc::new(RefCell::new(0));
    let accumulate: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let total: Ptr<i32> = total.as_pointer();
        },
        |mut x: i32, mut y: i32| {
            {
                let __rhs = (x * y);
                total.with_mut(|__v| *__v = *__v + __rhs)
            };
        },
        |mut x: u32, mut y: u32| {
            total.write({ (((total.read()) as u32).wrapping_add((x).wrapping_mul(y))) as i32 });
        }
    )));
    ({ (*accumulate.borrow()).spec::<fn(i32, i32)>(0).call(2, 3) });
    ({
        (*accumulate.borrow())
            .spec::<fn(u32, u32)>(1)
            .call(4_u32, 5_u32)
    });
    assert!(((*total.borrow()) == 26));
    let sub: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |mut x: i32, mut y: i32| -> i32 {
            return (x - y);
        },
        |mut x: f64, mut y: f64| -> f64 {
            return (x - y);
        }
    )));
    assert!((({ (*sub.borrow()).spec::<fn(i32, i32) -> i32>(0).call(9, 4,) }) == 5));
    assert!(
        (({
            (*sub.borrow())
                .spec::<fn(f64, f64) -> f64>(1)
                .call(2.5_f64, 1_f64)
        }) == 1.5_f64)
    );
    let mixed: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let base: Value<i32> = Rc::new(RefCell::new((*base.borrow())));
        },
        |mut x: i32, mut y: i32| -> i32 {
            return ((x * y) + (*base.borrow()));
        },
        |mut x: i32, mut y: f64| -> f64 {
            return (((x as f64) * y) + ((*base.borrow()) as f64));
        }
    )));
    assert!((({ (*mixed.borrow()).spec::<fn(i32, i32) -> i32>(0).call(2, 3,) }) == 16));
    assert!(
        (({
            (*mixed.borrow())
                .spec::<fn(i32, f64) -> f64>(1)
                .call(2, 0.5_f64)
        }) == 11_f64)
    );
    let unused: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(Generic, {})));
    &(*unused.borrow_mut());
    let outer: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let base: Value<i32> = Rc::new(RefCell::new((*base.borrow())));
        },
        |mut y: i32| -> i32 {
            let inner: Value<FnPtr<Generic>> =
                Rc::new(RefCell::new(lambda!(Generic, {}, |mut z: i32| -> i32 {
                    return (z + 1);
                })));
            return (({ (*inner.borrow()).spec::<fn(i32) -> i32>(0).call(y) }) + (*base.borrow()));
        }
    )));
    assert!((({ (*outer.borrow()).call(5,) }) == 16));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
