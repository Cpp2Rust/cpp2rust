extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(a: i32, b: Option<i32>) -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    let b: Value<i32> = Rc::new(RefCell::new(b.unwrap_or(10)));
    return ((*a.borrow()) + (*b.borrow()));
}
pub fn baz_1(a: Ptr<i32>, b: Option<Ptr<i32>>) -> bool {
    let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Ptr<i32>> = Rc::new(RefCell::new(b.unwrap_or(Ptr::<i32>::null())));
    return {
        let _lhs = (*a.borrow()).clone();
        _lhs == (*b.borrow()).clone()
    };
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ foo_0(1, None,) }) == 11));
    assert!((({ foo_0(1, Some(2),) }) == 3));
    let a: Value<i32> = Rc::new(RefCell::new(0));
    assert!(((({ baz_1((a.as_pointer()), None,) }) as i32) == (false as i32)));
    assert!(
        ((({
            let _a: Ptr<i32> = (a.as_pointer());
            let _b: Ptr<i32> = (a.as_pointer());
            baz_1(_a, Some(_b))
        }) as i32)
            == (true as i32))
    );
    return 0;
}
