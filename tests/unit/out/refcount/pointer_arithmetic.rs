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
    let x: Value<i32> = Rc::new(RefCell::new(1));
    let mut p: Ptr<i32> = (x.as_pointer());
    {
        let _ptr = p.clone();
        _ptr.write(_ptr.read() + 1)
    };
    assert!(((*x.borrow()) == 2));
    let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
    p = ((a.as_pointer() as Ptr<i32>).offset(1));
    {
        let _ptr = p.clone();
        _ptr.write(_ptr.read() + 1)
    };
    assert!(((*a.borrow())[(0) as usize] == 1) && ((*a.borrow())[(1) as usize] == 3));
    p.prefix_dec();
    {
        let _ptr = p.clone();
        _ptr.write(_ptr.read() + 1)
    };
    assert!(((*a.borrow())[(0) as usize] == 2) && ((*a.borrow())[(1) as usize] == 3));
    p = (x.as_pointer());
    {
        let _ptr = p.clone();
        _ptr.write(_ptr.read() + 1)
    };
    assert!(((*x.borrow()) == 3));
    let mut p2: Ptr<i32> = (p).clone();
    {
        let _ptr = p2.clone();
        _ptr.write(_ptr.read() + 1)
    };
    assert!(((*x.borrow()) == 4));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
