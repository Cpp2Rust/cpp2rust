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
pub struct XX {
    #[offset(0)]
    pub x: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let obj: Value<XX> = Rc::new(RefCell::new(<XX>::default()));
    let ptr: Value<Ptr<XX>> = Rc::new(RefCell::new((obj.as_pointer())));
    field!((*ptr.borrow()), x).write(2);
    let c: Value<bool> = Rc::new(RefCell::new(false));
    let r: Value<i32> = Rc::new(RefCell::new(if (*c.borrow()) {
        { (*obj.borrow()).x }
    } else {
        (*ptr.borrow()).with(|__s| __s.x)
    }));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((field_ptr!(obj.as_pointer(), x))));
    assert!((({ ((*p.borrow()).read()) } + { (*r.borrow()) }) == 4));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
