extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn f2_0(x: f64, y: f64) -> f64 {
    let x: Value<f64> = Rc::new(RefCell::new(x));
    let y: Value<f64> = Rc::new(RefCell::new(y));
    return ((*x.borrow()) - (*y.borrow()));
}
pub fn f3_1(x: f64, y: f64, z: f64) -> f64 {
    let x: Value<f64> = Rc::new(RefCell::new(x));
    let y: Value<f64> = Rc::new(RefCell::new(y));
    let z: Value<f64> = Rc::new(RefCell::new(z));
    return (({ f2_0((*x.borrow()), (*y.borrow())) }) + (*z.borrow()));
}
pub fn f1_2(x: f64, y: f64) -> f64 {
    let x: Value<f64> = Rc::new(RefCell::new(x));
    let y: Value<f64> = Rc::new(RefCell::new(y));
    let z1: Value<f64> = Rc::new(RefCell::new(({ f2_0((*x.borrow()), (*y.borrow())) })));
    if (({ f2_0((*z1.borrow()), (*y.borrow())) }) < 0_f64) {
        let z2: Value<f64> = Rc::new(RefCell::new(
            -({
                let _y: f64 = ({ f2_0((*x.borrow()), (*y.borrow())) });
                let _z: f64 = (*y.borrow());
                f3_1((*z1.borrow()), _y, _z)
            }),
        ));
        return ({
            f2_0(
                ({
                    let _x: f64 = (*z2.borrow());
                    let _y: f64 = ({ f3_1((*z1.borrow()), (*z2.borrow()), (*x.borrow())) });
                    f2_0(_x, _y)
                }),
                (*y.borrow()),
            )
        });
    }
    return ({ f2_0((*z1.borrow()), (*x.borrow())) });
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    return (({ f1_2(1.0E+0, 2.0E+0) }) as i32);
}
