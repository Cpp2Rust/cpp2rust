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
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    let mut __decomp_0: [i32; 3] = Box::new(std::array::from_fn::<_, 3, _>(|__i: usize| {
        (*arr.borrow())[(__i) as usize]
    }));
    __decomp_0[(0) as usize] = 10;
    assert!((__decomp_0[(0) as usize] == 10));
    assert!((__decomp_0[(1) as usize] == 2));
    assert!((__decomp_0[(2) as usize] == 3));
    assert!(((*arr.borrow())[(0) as usize] == 1));
    let __decomp_1: Ptr<i32> = arr.as_pointer();
    elem!((__decomp_1), 0).write(7);
    {
        let __rhs = (elem!((__decomp_1), 1).read());
        elem!((__decomp_1), 2).with_mut(|__v| *__v = *__v + __rhs)
    };
    assert!(((*arr.borrow())[(0) as usize] == 7));
    assert!(((*arr.borrow())[(2) as usize] == 5));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
