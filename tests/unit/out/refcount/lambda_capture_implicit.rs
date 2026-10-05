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
    let a: Value<i32> = Rc::new(RefCell::new(1));
    let b: Value<i32> = Rc::new(RefCell::new(2));
    let c: Value<i32> = Rc::new(RefCell::new(3));
    let by_value: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let a: Value<i32> = Rc::new(RefCell::new((*a.borrow())));
            let b: Value<i32> = Rc::new(RefCell::new((*b.borrow())));
            let c: Value<i32> = Rc::new(RefCell::new((*c.borrow())));
        },
        |x: i32| -> i32 {
            return ((((*a.borrow()) + (*b.borrow())) + (*c.borrow())) + x);
        }
    )));
    assert!((({ (*by_value.borrow()).call(10,) }) == 16));
    (*a.borrow_mut()) = 100;
    assert!((({ (*by_value.borrow()).call(10,) }) == 16));
    let by_ref: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let a: Ptr<i32> = a.as_pointer();
            let b: Ptr<i32> = b.as_pointer();
            let c: Ptr<i32> = c.as_pointer();
        },
        |x: i32| -> i32 {
            return ((((a.read()) + (b.read())) + (c.read())) + x);
        }
    )));
    assert!((({ (*by_ref.borrow()).call(10,) }) == 115));
    (*b.borrow_mut()) = 200;
    assert!((({ (*by_ref.borrow()).call(10,) }) == 313));
    let mixed: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let c: Ptr<i32> = c.as_pointer();
            let a: Value<i32> = Rc::new(RefCell::new((*a.borrow())));
            let b: Value<i32> = Rc::new(RefCell::new((*b.borrow())));
        },
        |x: i32| -> i32 {
            {
                let __rhs = x;
                c.with_mut(|__v| *__v = *__v + __rhs)
            };
            return (((*a.borrow()) + (*b.borrow())) + (c.read()));
        }
    )));
    assert!((({ (*mixed.borrow()).call(1,) }) == ((100 + 200) + 4)));
    assert!(((*c.borrow()) == 4));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
