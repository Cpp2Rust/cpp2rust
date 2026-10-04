extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct Triple {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: bool,
    #[offset(8)]
    pub c: i32,
}
pub fn sum_0(t: Ptr<Triple>) -> i32 {
    let __decomp_1: Ptr<Triple> = (t).clone();
    return ((__decomp_1.with(|__s| __s.a) + (if __decomp_1.with(|__s| __s.b) { 1 } else { 0 }))
        + __decomp_1.with(|__s| __s.c));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let t: Value<Triple> = Rc::new(RefCell::new(Triple {
        a: 10,
        b: false,
        c: 20,
    }));
    let __decomp_2: Ptr<Triple> = t.as_pointer();
    field!(__decomp_2, a).write(11);
    field!(__decomp_2, b).write(true);
    {
        let _ptr = field!(__decomp_2, c);
        _ptr.write(_ptr.read() + 1)
    };
    assert!(({ (*t.borrow()).a } == 11));
    assert!({ (*t.borrow()).b });
    assert!(({ (*t.borrow()).c } == 21));
    (*t.borrow_mut()).a = 12;
    assert!((__decomp_2.with(|__s| __s.a) == 12));
    let __decomp_3: Ptr<Triple> = t.as_pointer();
    (*t.borrow_mut()).c = 30;
    assert!((__decomp_3.with(|__s| __s.a) == 12));
    assert!(__decomp_3.with(|__s| __s.b));
    assert!((__decomp_3.with(|__s| __s.c) == 30));
    assert!((({ sum_0(t.as_pointer(),) }) == 43));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
