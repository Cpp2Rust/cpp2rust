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
    let __decomp_0: Value<Box<[i32]>> =
        Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 3, _>(
            |__i: usize| (*arr.borrow())[(__i) as usize],
        ))));
    (*__decomp_0.borrow_mut())[(0) as usize] = 10;
    assert!(((*__decomp_0.borrow())[(0) as usize] == 10));
    assert!(((*__decomp_0.borrow())[(1) as usize] == 2));
    assert!(((*__decomp_0.borrow())[(2) as usize] == 3));
    assert!(((*arr.borrow())[(0) as usize] == 1));
    let __decomp_1: Ptr<i32> = (arr.as_pointer() as Ptr<i32>);
    (__decomp_1).offset((0) as isize).write(7);
    {
        let _ptr = (__decomp_1).offset((2) as isize);
        _ptr.write(_ptr.read() + ((__decomp_1).offset((1) as isize).read()))
    };
    assert!(((*arr.borrow())[(0) as usize] == 7));
    assert!(((*arr.borrow())[(2) as usize] == 5));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
