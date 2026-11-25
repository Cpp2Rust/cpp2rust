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
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    printf!(b"%e|%E\n", 0_f64, -0_f64);
    printf!(b"%.0e|%#.0E|%02.2e\n", 0_f64, -0_f64, 0_f64);
    printf!(b"%+ 012.2e|%-#012.0E|% 12.1e\n", 0_f64, -0_f64, 0_f64);
    printf!(b"% .1e|% 012.1e\n", 0_f64, 0_f64);
    printf!(b"%g|%G|%#g\n", 0_f64, -0_f64, 0_f64);
    printf!(
        b"%.0g|%.1g|%#.0g|%#.1G|%#.4g\n",
        0_f64,
        -0_f64,
        0_f64,
        -0_f64,
        0_f64
    );
    printf!(
        b"%+ 08g|% g|% 08G|%-#08.4G|%02.4g\n",
        0_f64,
        0_f64,
        0_f64,
        -0_f64,
        0_f64
    );
    printf!(b"%f|%F|%e|%g\n", 0_f64, -0_f64, 1500_f64, 0.5_f64);
    return 0;
}
pub fn __cpp2rust_init_globals() {}
