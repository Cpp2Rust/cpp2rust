extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    let increment_twice: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |n: i32| -> i32 {
            let n: Value<i32> = Rc::new(RefCell::new(n));
            let increment: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                {
                    let n: Ptr<i32> = n.as_pointer();
                },
                || {
                    n.with_mut(|__v| {
                        __v.prefix_inc();
                    });
                }
            )));
            ({ (*increment.borrow()).call() });
            ({ (*increment.borrow()).call() });
            return (*n.borrow());
        })));
    assert!(
        ({
            (({
                (*increment_twice.borrow())
                    .spec::<fn(i32) -> i32>(0)
                    .call(3)
            }) == 5)
        })
    );
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
