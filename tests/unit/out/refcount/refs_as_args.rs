extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn more_refs_0(x1: i32, x2: i32, r1: Ptr<i32>, r2: Ptr<i32>) {
    let x1: Value<i32> = Rc::new(RefCell::new(x1));
    let x2: Value<i32> = Rc::new(RefCell::new(x2));
    let rx1: Ptr<i32> = x1.as_pointer();
    let rx2: Ptr<i32> = x2.as_pointer();
    let pr1: Value<Ptr<i32>> = Rc::new(RefCell::new((r1).clone()));
    let pr2: Value<Ptr<i32>> = Rc::new(RefCell::new((r2).clone()));
    let rpr1: Ptr<i32> = (*pr1.borrow()).clone();
    let rpr2: Ptr<i32> = (*pr2.borrow()).clone();
    let r: Ptr<i32> = (r1).clone();
    let __rhs = {
        let _lhs = {
            let _lhs = {
                let _lhs = {
                    let _lhs = {
                        let _lhs = {
                            let _lhs = (1 + (rx1.read()));
                            _lhs + (rx2.read())
                        };
                        _lhs + ((*pr1.borrow()).read())
                    };
                    _lhs + ((*pr2.borrow()).read())
                };
                _lhs + (rpr1.read())
            };
            _lhs + (rpr2.read())
        };
        _lhs + (r.read())
    };
    {
        let __ptr = rx2.clone();
        let __tmp = __ptr.read() + __rhs;
        __ptr.write(__tmp)
    };
    let __rhs = (rx2.read());
    r1.write(__rhs);
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x1: Value<i32> = Rc::new(RefCell::new(1));
    let x2: Value<i32> = Rc::new(RefCell::new(2));
    ({
        let _x1: i32 = 3;
        let _x2: i32 = 4;
        let _r1: Ptr<i32> = x1.as_pointer();
        let _r2: Ptr<i32> = x2.as_pointer();
        more_refs_0(_x1, _x2, _r1, _r2)
    });
    return ((*x1.borrow()) + (*x2.borrow()));
}
