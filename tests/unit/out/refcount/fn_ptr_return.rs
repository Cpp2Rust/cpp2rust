extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn inc_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ((*x.borrow()) + 1);
}
pub fn dec_1(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ((*x.borrow()) - 1);
}
pub fn pick_2(choose_inc: i32) -> FnPtr<fn(i32) -> i32> {
    let choose_inc: Value<i32> = Rc::new(RefCell::new(choose_inc));
    if ((*choose_inc.borrow()) != 0) {
        return fn_ptr!(inc_0, fn(i32) -> i32);
    }
    return fn_ptr!(dec_1, fn(i32) -> i32);
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let f: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(
        ({
            let _choose_inc: i32 = 1;
            pick_2(_choose_inc)
        }),
    ));
    assert!(!((*f.borrow()).is_null()));
    assert!({
        let _lhs = (*f.borrow()).clone();
        _lhs == fn_ptr!(inc_0, fn(i32) -> i32)
    });
    assert!(
        (({
            let _arg0: i32 = 10;
            (*f.borrow()).call()(_arg0)
        }) == 11)
    );
    let g: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(
        ({
            let _choose_inc: i32 = 0;
            pick_2(_choose_inc)
        }),
    ));
    assert!({
        let _lhs = (*g.borrow()).clone();
        _lhs == fn_ptr!(dec_1, fn(i32) -> i32)
    });
    assert!(
        (({
            let _arg0: i32 = 10;
            (*g.borrow()).call()(_arg0)
        }) == 9)
    );
    assert!({
        let _lhs = (*f.borrow()).clone();
        _lhs != (*g.borrow()).clone()
    });
    return 0;
}
