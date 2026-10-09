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
    let fact: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |self_: FnPtr<
            Generic,
        >,
                                                   n: i32|
         -> i32 {
            let self_: Value<FnPtr<Generic>> = Rc::new(RefCell::new(self_));
            if (n <= 1) {
                return 1;
            }
            return (n
                * ({
                    let _self_: FnPtr<Generic> = (*self_.borrow()).clone();
                    (*self_.borrow())
                        .spec::<fn(FnPtr<Generic>, i32) -> i32>(0)
                        .call(_self_, (n - 1))
                }));
        },)));
    assert!(
        (({
            let _self_: FnPtr<Generic> = (*fact.borrow()).clone();
            (*fact.borrow())
                .spec::<fn(FnPtr<Generic>, i32) -> i32>(0)
                .call(_self_, 5)
        }) == 120)
    );
    let calls: Value<i32> = Rc::new(RefCell::new(0));
    let fib: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let calls: Ptr<i32> = calls.as_pointer();
        },
        |self_: Ptr<FnPtr<Generic>>, n: i32| -> i32 {
            calls.with_mut(|__v| __v.postfix_inc());
            if (n <= 2) {
                return 1;
            }
            return (({
                let _self_: Ptr<FnPtr<Generic>> = (self_).clone();
                let _n: i32 = (n - 1);
                (*self_.upgrade().deref())
                    .spec::<fn(Ptr<FnPtr<Generic>>, i32) -> i32>(0)
                    .call(_self_, _n)
            }) + ({
                let _self_: Ptr<FnPtr<Generic>> = (self_).clone();
                let _n: i32 = (n - 2);
                (*self_.upgrade().deref())
                    .spec::<fn(Ptr<FnPtr<Generic>>, i32) -> i32>(0)
                    .call(_self_, _n)
            }));
        },
    )));
    assert!(
        (({
            let _self_: Ptr<FnPtr<Generic>> = fib.as_pointer();
            (*fib.borrow())
                .spec::<fn(Ptr<FnPtr<Generic>>, i32) -> i32>(0)
                .call(_self_, 6)
        }) == 8)
    );
    assert!(((*calls.borrow()) == 15));
    let depth: Value<i32> = Rc::new(RefCell::new(0));
    let count_down: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let depth: Ptr<i32> = depth.as_pointer();
        },
        |self_: Ptr<FnPtr<Generic>>, n: i32| {
            if (n == 0) {
                return;
            }
            depth.with_mut(|__v| __v.postfix_inc());
            ({
                let _self_: Ptr<FnPtr<Generic>> = (self_).clone();
                let _n: i32 = (n - 1);
                (*self_.upgrade().deref())
                    .spec::<fn(Ptr<FnPtr<Generic>>, i32)>(0)
                    .call(_self_, _n)
            });
        },
    )));
    ({
        let _self_: Ptr<FnPtr<Generic>> = count_down.as_pointer();
        (*count_down.borrow())
            .spec::<fn(Ptr<FnPtr<Generic>>, i32)>(0)
            .call(_self_, 4)
    });
    assert!(((*depth.borrow()) == 4));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
