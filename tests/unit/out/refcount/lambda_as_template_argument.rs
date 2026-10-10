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
    return ({ (*fn_.borrow()).spec::<fn(i32) -> i32>(0).call(x) });
}
pub fn apply_int_2(fn_: FnPtr<Generic>, mut x: i32) -> i32 {
    let fn_: Value<FnPtr<Generic>> = Rc::new(RefCell::new(fn_));
    return ({ (*fn_.borrow()).spec::<fn(i32) -> i32>(0).call(x) });
}
pub fn apply_int_3(fn_: FnPtr<Generic>, mut x: i32) -> i32 {
    let fn_: Value<FnPtr<Generic>> = Rc::new(RefCell::new(fn_));
    return ({ (*fn_.borrow()).spec::<fn(i32) -> i32>(0).call(x) });
}
pub fn apply_double_4(fn_: FnPtr<Generic>, mut x: f64) -> f64 {
    let fn_: Value<FnPtr<Generic>> = Rc::new(RefCell::new(fn_));
    return ({ (*fn_.borrow()).spec::<fn(f64) -> f64>(1).call(x) });
}
pub fn apply_double_5(fn_: FnPtr<Generic>, mut x: f64) -> f64 {
    let fn_: Value<FnPtr<Generic>> = Rc::new(RefCell::new(fn_));
    return ({ (*fn_.borrow()).spec::<fn(f64) -> f64>(1).call(x) });
}
pub fn apply_twice_6(fn_: FnPtr<fn(i32) -> i32>, mut x: i32) -> i32 {
    let fn_: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(fn_));
    return ({
        let _x: i32 = ({ (*fn_.borrow()).call(x) });
        (*fn_.borrow()).call(_x)
    })
    .clone();
}
pub fn none_match_7(pred: FnPtr<fn(i32) -> bool>, mut a: i32, mut b: i32) -> bool {
    let pred: Value<FnPtr<fn(i32) -> bool>> = Rc::new(RefCell::new(pred));
    let neg_pred: Value<FnPtr<fn(i32) -> bool>> = Rc::new(RefCell::new(lambda!(
        {
            let pred: Ptr<FnPtr<fn(i32) -> bool>> = pred.as_pointer();
        },
        |mut x: i32| -> bool {
            return !({ (*pred.upgrade().deref()).call(x) });
        }
    )));
    return ({ (*neg_pred.borrow()).call(a) }) && ({ (*neg_pred.borrow()).call(b) });
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
        |mut x: i32| -> i32 {
            return (x * (*factor.borrow()));
        }
    )));
    assert!((({ apply_twice_6((*scale.borrow()).copy_from(), 4,) }) == 36));
    assert!(
        (({
            apply_int_0(
                FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
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
        |mut x: i32| -> i32 {
            return (x * (*factor.borrow()));
        },
        |mut x: f64| -> f64 {
            return (x * ((*factor.borrow()) as f64));
        }
    )));
    assert!((({ apply_int_1((*generic_scale.borrow()).copy_from(), 4,) }) == 12));
    assert!((({ apply_double_4((*generic_scale.borrow()).copy_from(), 1.5_f64,) }) == 4.5_f64));
    assert!(
        (({
            apply_int_2(
                lambda!(Generic, {}, |mut x: i32| -> i32 {
                    return -x;
                }),
                9,
            )
        }) == -9_i32)
    );
    let offset: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let factor: Value<i32> = Rc::new(RefCell::new((*factor.borrow())));
        },
        |mut x: i32| -> i32 {
            return (x + (*factor.borrow()));
        },
        |mut x: f64| -> f64 {
            return (x + ((*factor.borrow()) as f64));
        }
    )));
    assert!((({ apply_int_3((*offset.borrow()).copy_from(), 4,) }) == 7));
    assert!((({ apply_double_5((*offset.borrow()).copy_from(), 1.5_f64,) }) == 4.5_f64));
    let is_even: Value<FnPtr<fn(i32) -> bool>> = Rc::new(RefCell::new(
        FnPtr::<fn(i32) -> bool>::new(|mut x: i32| -> bool {
            {
                return ((x % 2) == 0);
            }
        }),
    ));
    assert!(({ none_match_7((*is_even.borrow()).copy_from(), 1, 3,) }));
    assert!(!({ none_match_7((*is_even.borrow()).copy_from(), 1, 4,) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
