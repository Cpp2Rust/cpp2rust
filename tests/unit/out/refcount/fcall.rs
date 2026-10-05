extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn f2_0(mut x: f64, mut y: f64) -> f64 {
    return (x - y);
}
pub fn f3_1(mut x: f64, mut y: f64, mut z: f64) -> f64 {
    return (({ f2_0(x, y) }) + z);
}
pub fn f1_2(mut x: f64, mut y: f64) -> f64 {
    let mut z1: f64 = ({ f2_0(x, y) });
    if (({ f2_0(z1, y) }) < 0_f64) {
        let mut z2: f64 = -({
            let _y: f64 = ({ f2_0(x, y) });
            let _z: f64 = y;
            f3_1(z1, _y, _z)
        });
        return ({
            f2_0(
                ({
                    let _x: f64 = z2;
                    let _y: f64 = ({ f3_1(z1, z2, x) });
                    f2_0(_x, _y)
                }),
                y,
            )
        });
    }
    return ({ f2_0(z1, x) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ f1_2(1.0E+0, 2.0E+0,) }) == (-6_i32 as f64)));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
