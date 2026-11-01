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
    let __decomp_1: Ptr<i32> = arr.as_pointer();
    elem!((__decomp_1), 0).write(7);
    {
        let __rhs = (elem!((__decomp_1), 1).read());
        elem!((__decomp_1), 2).with_mut(|__v| *__v = *__v + __rhs)
    };
    assert!(((*arr.borrow())[(0) as usize] == 7));
    assert!(((*arr.borrow())[(2) as usize] == 5));
    let std_arr: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([4, 5, 6])));
    let mut last: i32 = (std_arr.decay().offset(2 as isize).read());
    std_arr.decay().offset(0 as isize).write(8);
    let __decomp_2: Ptr<Vec<i32>> = std_arr.as_pointer();
    let p: Ptr<i32> = __decomp_2.decay().offset(0 as isize);
    let q: Ptr<i32> = __decomp_2.decay().offset(1 as isize);
    let r: Ptr<i32> = __decomp_2.decay().offset(2 as isize);
    {
        q.with_mut(|__v| *__v = *__v + 10)
    };
    assert!((last == 6));
    assert!(({ (*std_arr.borrow())[0_usize] } == 8));
    assert!(({ (*std_arr.borrow())[1_usize] } == 15));
    assert!(((r.read()) == 6));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
