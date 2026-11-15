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
    let n: Value<i32> = Rc::new(RefCell::new(5));
    let f: Value<FnPtr<fn() -> Ptr<i32>>> = Rc::new(RefCell::new(lambda!(
        {
            let n: Value<i32> = Rc::new(RefCell::new((*n.borrow())));
        },
        || -> Ptr<i32> {
            return (n.as_pointer());
        }
    )));
    assert!(({ (({ (*f.borrow()).call() }) != (n.as_pointer())) }));
    assert!(({ ((({ (*f.borrow()).call() }).read()) == 5) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
