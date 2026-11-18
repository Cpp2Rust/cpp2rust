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
pub fn take_1(r: Ptr<i32>) {
    r.write(8);
}
pub fn test_2() {
    let v: Value<i32> = Rc::new(RefCell::new(1));
    let cr: Ptr<i32> = v.as_pointer();
    ({ set_0((cr).clone()) });
    assert!(({ ((*v.borrow()) == 7) }));
    ({ take_1((cr).clone()) });
    assert!(({ ((*v.borrow()) == 8) }));
    let vec_: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1])));
    let cvec: Ptr<Vec<i32>> = vec_.as_pointer();
    {
        let __a1 = 2;
        cvec.with_mut(|__v: &mut Vec<i32>| __v.push(__a1))
    };
    assert!(({ ((*vec_.borrow()).len() == 2_usize) }));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_2() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
