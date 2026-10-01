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
    let start: Value<i32> = Rc::new(RefCell::new(5));
    let next: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let start: Value<i32> = Rc::new(RefCell::new((*start.borrow())));
        },
        || -> i32 {
            return (*start.borrow_mut()).postfix_inc();
        }
    )));
    assert!((({ (*next.borrow()).call() }) == 5));
    assert!((({ (*next.borrow()).call() }) == 6));
    assert!((({ (*next.borrow()).call() }) == 7));
    assert!(((*start.borrow()) == 5));
    let total: Value<i32> = Rc::new(RefCell::new(0));
    let accumulate: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let total: Value<i32> = Rc::new(RefCell::new((*total.borrow())));
        },
        |x: i32| -> i32 {
            let x: Value<i32> = Rc::new(RefCell::new(x));
            (*total.borrow_mut()) += (*x.borrow());
            return (*total.borrow());
        }
    )));
    assert!((({ (*accumulate.borrow()).call(1,) }) == 1));
    assert!((({ (*accumulate.borrow()).call(2,) }) == 3));
    assert!(((*total.borrow()) == 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
