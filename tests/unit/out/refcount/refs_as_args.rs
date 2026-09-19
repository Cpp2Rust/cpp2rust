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
        let _ptr = rx2.clone();
        _ptr.write(_ptr.read() + __rhs)
    };
    let __rhs = (rx2.read());
    r1.write(__rhs);
}
#[derive(Default)]
pub struct Val {
    pub x: Value<i32>,
}
impl Clone for Val {
    fn clone(&self) -> Self {
        let __this: Value<Val> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
        }));
        let this: Ptr<Val> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Val {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn sum_1(a: Val, b: Val) -> i32 {
    let a: Value<Val> = Rc::new(RefCell::new(a));
    let b: Value<Val> = Rc::new(RefCell::new(b));
    return ((*(*a.borrow()).x.borrow()) + (*(*b.borrow()).x.borrow()));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x1: Value<i32> = Rc::new(RefCell::new(1));
    let x2: Value<i32> = Rc::new(RefCell::new(2));
    ({ more_refs_0(3, 4, x1.as_pointer(), x2.as_pointer()) });
    assert!((((*x1.borrow()) + (*x2.borrow())) == 21));
    let v: Value<Val> = Rc::new(RefCell::new(Val {
        x: Rc::new(RefCell::new(5)),
    }));
    let acc: Value<i32> = Rc::new(RefCell::new(
        ({
            let _a: Val = (*v.borrow()).clone();
            let _b: Val = (*v.borrow()).clone();
            sum_1(_a, _b)
        }),
    ));
    (*acc.borrow_mut()) += ({
        let _a: Val = (*v.borrow()).clone();
        let _b: Val = (*v.borrow()).clone();
        sum_1(_a, _b)
    });
    (*acc.borrow_mut()) += ({
        let _a: Val = (*v.borrow()).clone();
        let _b: Val = (*v.borrow()).clone();
        sum_1(_a, _b)
    });
    (*acc.borrow_mut()) += ({
        let _a: Val = (*v.borrow()).clone();
        let _b: Val = (*v.borrow()).clone();
        sum_1(_a, _b)
    });
    assert!(((*acc.borrow()) == 40));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
