extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn mixed_args_0(count: i32, __args: &[VaArg]) -> i32 {
    let count: Value<i32> = Rc::new(RefCell::new(count));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    (*ap.borrow_mut()) = VaList::new(__args);
    let mut total: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while (((i < (*count.borrow())) as i32) != 0) {
        let mut tag: i32 = (*ap.borrow_mut()).arg::<i32>();
        if (((tag == 0) as i32) != 0) {
            total += (*ap.borrow_mut()).arg::<i32>();
        } else {
            let mut ptr: Ptr<i32> = (*ap.borrow_mut()).arg::<Ptr<i32>>();
            total += { (ptr.read()) };
        }
        i.postfix_inc();
    }
    return total;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(100));
    assert!(
        (((({
            mixed_args_0(
                3,
                &[
                    (0).into(),
                    (10).into(),
                    (1).into(),
                    (x.as_pointer()).into(),
                    (0).into(),
                    (20).into(),
                ],
            )
        }) == 130) as i32)
            != 0)
    );
    let y: Value<i32> = Rc::new(RefCell::new(50));
    assert!((((({ mixed_args_0(1, &[(1).into(), (y.as_pointer()).into(),]) }) == 50) as i32) != 0));
    assert!(
        (((({ mixed_args_0(2, &[(0).into(), (5).into(), (0).into(), (3).into(),]) }) == 8) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
