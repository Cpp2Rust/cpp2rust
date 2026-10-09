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
pub struct Val {
    #[offset(0)]
    pub x: i32,
}
pub fn sum_0(mut a: Val, mut b: Val) -> i32 {
    return (a.x + b.x);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let total: Value<i32> = Rc::new(RefCell::new(0));
    let tally: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let total: Ptr<i32> = total.as_pointer();
        },
        || {
            total.write({
                (((total.read()) as usize).wrapping_add(
                    ((::std::mem::size_of::<i8>() as usize)
                        .wrapping_add((::std::mem::size_of::<i8>() as usize))
                        as usize),
                )) as i32
            });
        },
        || {
            total.write({
                (((total.read()) as usize).wrapping_add(
                    ((::std::mem::size_of::<i32>() as usize)
                        .wrapping_add((::std::mem::size_of::<i8>() as usize))
                        as usize),
                )) as i32
            });
        }
    )));
    ({ (*tally.borrow()).spec::<fn()>(0).call() });
    ({ (*tally.borrow()).spec::<fn()>(1).call() });
    assert!(((*total.borrow()) == 7));
    let v: Value<Val> = Rc::new(RefCell::new(Val { x: 5 }));
    let acc: Value<i32> = Rc::new(RefCell::new(0));
    let pick: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Ptr<Val> = v.as_pointer();
            let acc: Ptr<i32> = acc.as_pointer();
        },
        || {
            {
                let __rhs = ({
                    let _a: Val = (*v.upgrade().deref()).clone();
                    let _b: Val = (*v.upgrade().deref()).clone();
                    sum_0(_a, _b)
                });
                acc.with_mut(|__v| *__v = *__v + __rhs)
            };
        },
        || {
            {
                let __rhs = ({
                    let _a: Val = (*v.upgrade().deref()).clone();
                    let _b: Val = (*v.upgrade().deref()).clone();
                    sum_0(_a, _b)
                });
                acc.with_mut(|__v| *__v = *__v + __rhs)
            };
        },
        || {
            {
                let __rhs = ({
                    let _a: Val = (*v.upgrade().deref()).clone();
                    let _b: Val = (*v.upgrade().deref()).clone();
                    sum_0(_a, _b)
                });
                acc.with_mut(|__v| *__v = *__v + __rhs)
            };
        }
    )));
    ({ (*pick.borrow()).spec::<fn()>(0).call() });
    ({ (*pick.borrow()).spec::<fn()>(1).call() });
    ({ (*pick.borrow()).spec::<fn()>(2).call() });
    assert!(((*acc.borrow()) == 30));
    let cast_to: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |x: i32| -> i32 {
            return ((x as i32) / 2);
        },
        |x: i32| -> f64 {
            return ((x as f64) / 2_f64);
        }
    )));
    assert!((({ (*cast_to.borrow()).spec::<fn(i32) -> i32>(0).call(5,) }) == 2));
    assert!((({ (*cast_to.borrow()).spec::<fn(i32) -> f64>(1).call(5,) }) == 2.5_f64));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
