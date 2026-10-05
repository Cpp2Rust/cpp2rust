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
    let i1: Value<i32> = Rc::new(RefCell::new(3));
    let mut i2: i32 = (*i1.borrow());
    assert!(((*i1.borrow()) == 3));
    assert!((i2 == 3));
    let __tmp_0: Value<i32> = Rc::new(RefCell::new(40));
    let i3: Ptr<i32> = __tmp_0.as_pointer();
    {
        let _ptr = i3.clone();
        _ptr.write(_ptr.read() + 2)
    };
    assert!(((i3.read()) == 42));
    let __tmp_1: Value<i32> = Rc::new(RefCell::new((2 + 3)));
    let i4: Ptr<i32> = __tmp_1.as_pointer();
    assert!(((i4.read()) == 5));
    let __tmp_2: Value<i32> = Rc::new(RefCell::new(40));
    let i5: Ptr<i32> = __tmp_2.as_pointer();
    let i6: Ptr<i32> = (i3).clone();
    let i7: Ptr<i32> = (i4).clone();
    assert!(({ (i6.read()) } == { (i3.read()) }));
    assert!(({ (i7.read()) } == { (i4.read()) }));
    let i8: Value<i32> = Rc::new(RefCell::new(3));
    let i9: Ptr<i32> = i8.as_pointer();
    assert!(((i9.read()) == 3));
    let mut p1: Ptr<i32> = (i1.as_pointer());
    let mut p2: Ptr<i32> = (i3).clone();
    let mut p3: Ptr<i32> = (i6).clone();
    assert!(({ (p1.read()) } == { (*i1.borrow()) }));
    assert!(({ (p2.read()) } == { (i3.read()) }));
    assert!(({ (p3.read()) } == { (i6.read()) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
