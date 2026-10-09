extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn apply_int_0(fn_: FnPtr<fn(i32) -> i32>, mut x: i32) -> i32 {
    let fn_: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(fn_));
    return ({ (*fn_.borrow()).call(x) });
}
pub fn apply_int_1(fn_: FnPtr<Generic>, mut x: i32) -> i32 {
    let fn_: Value<FnPtr<Generic>> = Rc::new(RefCell::new(fn_));
    return ({ (*fn_.borrow()).spec::<fn(i32) -> i32>().call(x) });
}
pub fn apply_int_2(fn_: FnPtr<Generic>, mut x: i32) -> i32 {
    let fn_: Value<FnPtr<Generic>> = Rc::new(RefCell::new(fn_));
    return ({ (*fn_.borrow()).spec::<fn(i32) -> i32>().call(x) });
}
pub fn apply_int_3(fn_: FnPtr<Generic>, mut x: i32) -> i32 {
    let fn_: Value<FnPtr<Generic>> = Rc::new(RefCell::new(fn_));
    return ({ (*fn_.borrow()).spec::<fn(i32) -> i32>().call(x) });
}
pub fn apply_double_4(fn_: FnPtr<Generic>, mut x: f64) -> f64 {
    let fn_: Value<FnPtr<Generic>> = Rc::new(RefCell::new(fn_));
    return ({ (*fn_.borrow()).spec::<fn(f64) -> f64>().call(x) });
}
pub fn apply_double_5(fn_: FnPtr<Generic>, mut x: f64) -> f64 {
    let fn_: Value<FnPtr<Generic>> = Rc::new(RefCell::new(fn_));
    return ({ (*fn_.borrow()).spec::<fn(f64) -> f64>().call(x) });
}
pub fn apply_twice_6(fn_: FnPtr<fn(i32) -> i32>, mut x: i32) -> i32 {
    let fn_: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(fn_));
    return ({
        let _x: i32 = ({ (*fn_.borrow()).call(x) });
        (*fn_.borrow()).call(_x)
    })
    .clone();
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
    )));
    assert!((({ apply_twice_6((*scale.borrow()).copy_from(), 4,) }) == 36));
    assert!(
        (({
            apply_int_0(
                FnPtr::<fn(i32) -> i32>::new(|x: i32| -> i32 {
                    {
                        return -x;
                    }
                }),
                9,
            )
        }) == -9_i32)
    );
    let generic_scale: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let factor: Value<i32> = Rc::new(RefCell::new((*factor.borrow())));
        },
        |x: i32| -> i32 {
            return (x * (*factor.borrow()));
        },
        |x: f64| -> f64 {
            return (x * ((*factor.borrow()) as f64));
        },
    )));
    assert!((({ apply_int_1((*generic_scale.borrow()).copy_from(), 4,) }) == 12));
    assert!((({ apply_double_4((*generic_scale.borrow()).copy_from(), 1.5_f64,) }) == 4.5_f64));
    assert!(
        (({
            apply_int_2(
                lambda!(Generic, {}, |x: i32| -> i32 {
                    return -x;
                },),
                9,
            )
        }) == -9_i32)
    );
    let offset: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let factor: Value<i32> = Rc::new(RefCell::new((*factor.borrow())));
        },
        |x: i32| -> i32 {
            return (x + (*factor.borrow()));
        },
        |x: f64| -> f64 {
            return (x + ((*factor.borrow()) as f64));
        },
    )));
    assert!((({ apply_int_3((*offset.borrow()).copy_from(), 4,) }) == 7));
    assert!((({ apply_double_5((*offset.borrow()).copy_from(), 1.5_f64,) }) == 4.5_f64));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
