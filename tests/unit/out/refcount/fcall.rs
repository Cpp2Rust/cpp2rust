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
    return (({
        let _x: f64 = (*x.borrow());
        let _y: f64 = (*y.borrow());
        f2_0(_x, _y)
    }) + (*z.borrow()));
}
pub fn f1_2(x: f64, y: f64) -> f64 {
    let x: Value<f64> = Rc::new(RefCell::new(x));
    let y: Value<f64> = Rc::new(RefCell::new(y));
    let z1: Value<f64> = Rc::new(RefCell::new(
        ({
            let _x: f64 = (*x.borrow());
            let _y: f64 = (*y.borrow());
            f2_0(_x, _y)
        }),
    ));
    if (({
        let _x: f64 = (*z1.borrow());
        let _y: f64 = (*y.borrow());
        f2_0(_x, _y)
    }) < 0_f64)
    {
        let z2: Value<f64> = Rc::new(RefCell::new(
            -({
                let _x: f64 = (*z1.borrow());
                let _y: f64 = ({
                    let _x: f64 = (*x.borrow());
                    let _y: f64 = (*y.borrow());
                    f2_0(_x, _y)
                });
                let _z: f64 = (*y.borrow());
                f3_1(_x, _y, _z)
            }),
        ));
        return ({
            let _x: f64 = ({
                let _x: f64 = (*z2.borrow());
                let _y: f64 = ({
                    let _x: f64 = (*z1.borrow());
                    let _y: f64 = (*z2.borrow());
                    let _z: f64 = (*x.borrow());
                    f3_1(_x, _y, _z)
                });
                f2_0(_x, _y)
            });
            let _y: f64 = (*y.borrow());
            f2_0(_x, _y)
        });
    }
    return ({
        let _x: f64 = (*z1.borrow());
        let _y: f64 = (*x.borrow());
        f2_0(_x, _y)
    });
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    return (({
        let _x: f64 = 1.0E+0;
        let _y: f64 = 2.0E+0;
        f1_2(_x, _y)
    }) as i32);
}
