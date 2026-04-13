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
pub fn pick_2(choose_inc: i32) -> Option<fn(i32) -> i32> {
    let choose_inc: Value<i32> = Rc::new(RefCell::new(choose_inc));
    if ((*choose_inc.borrow()) != 0) {
        return Some(inc_0 as _);
    }
    return Some(dec_1 as _);
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let f: Value<Option<fn(i32) -> i32>> = Rc::new(RefCell::new(
        ({
            let _choose_inc: i32 = 1;
            pick_2(_choose_inc)
        }),
    ));
    assert!(!((*f.borrow()).is_none()));
    assert!({
        let _lhs = (*f.borrow()).clone();
        _lhs == Some(inc_0 as _)
    });
    assert!(
        (({
            let _arg0: i32 = 10;
            (*f.borrow()).unwrap()(_arg0)
        }) == 11)
    );
    let g: Value<Option<fn(i32) -> i32>> = Rc::new(RefCell::new(
        ({
            let _choose_inc: i32 = 0;
            pick_2(_choose_inc)
        }),
    ));
    assert!({
        let _lhs = (*g.borrow()).clone();
        _lhs == Some(dec_1 as _)
    });
    assert!(
        (({
            let _arg0: i32 = 10;
            (*g.borrow()).unwrap()(_arg0)
        }) == 9)
    );
    assert!({
        let _lhs = (*f.borrow()).clone();
        _lhs != (*g.borrow()).clone()
    });
    return 0;
}
