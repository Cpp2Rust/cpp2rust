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
    let mut c: i8 = (('a' as i32) as i8);
    let mut n: i32 = 3;
    printf!(b"%c\n", (c as i32));
    printf!(b"%d %c\n", n, (c as i32));
    printf!(b"100%% %c\n", (c as i32));
    printf!(b"%c%c%d\n", (c as i32), ((c as i32) + 1), n);
    return 0;
}
pub fn __cpp2rust_init_globals() {}
