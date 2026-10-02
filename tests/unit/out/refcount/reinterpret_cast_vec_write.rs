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
    let vec_: Value<Vec<u32>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = 67305985_u32;
        (*vec_.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 134678021_u32;
        (*vec_.borrow_mut()).push(__a1)
    };
    let bytes: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (vec_.as_pointer() as Ptr<u32>).reinterpret_cast::<u8>(),
    ));
    assert!((((elem!((*bytes.borrow()), 0).read()) as i32) == 1));
    assert!((((elem!((*bytes.borrow()), 1).read()) as i32) == 2));
    assert!((((elem!((*bytes.borrow()), 2).read()) as i32) == 3));
    assert!((((elem!((*bytes.borrow()), 3).read()) as i32) == 4));
    assert!((((elem!((*bytes.borrow()), 4).read()) as i32) == 5));
    assert!((((elem!((*bytes.borrow()), 7).read()) as i32) == 8));
    elem!((*bytes.borrow()), 4).write(255_u8);
    assert!(((elem!((vec_.as_pointer() as Ptr<u32>), 1_usize).read()) == 134678271_u32));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
