extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn set_0(r: Ptr<i32>) {
    r.write(7);
}
pub fn set_through_1(v: Ptr<i32>) {
    ({ set_0((v).clone()) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(1));
    let mut p: Ptr<i32> = (a.as_pointer());
    let mut q: Ptr<i32> = (<AnyPtr>::from_int((p).to_any().to_int())).reinterpret_cast::<i32>();
    assert!(({ (p).clone() } == { (q).clone() }));
    let v: Value<i32> = Rc::new(RefCell::new(1));
    let cr: Ptr<i32> = v.as_pointer();
    ({ set_0((cr).clone()) });
    assert!(((*v.borrow()) == 7));
    (*v.borrow_mut()) = 0;
    ({ set_through_1(v.as_pointer()) });
    assert!(((*v.borrow()) == 7));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
