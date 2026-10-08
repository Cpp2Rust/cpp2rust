extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_errno_0() {
    libcc2rs::cpp2rust_errno().write(0);
    assert!(((((libcc2rs::cpp2rust_errno().read()) == 0) as i32) != 0));
    libcc2rs::cpp2rust_errno().write(42);
    assert!(((((libcc2rs::cpp2rust_errno().read()) == 42) as i32) != 0));
    let mut saved: i32 = (libcc2rs::cpp2rust_errno().read());
    assert!((((saved == 42) as i32) != 0));
    libcc2rs::cpp2rust_errno().write(0);
}
pub fn test_errno_preserved_across_strdup_1() {
    libcc2rs::cpp2rust_errno().write(99);
    let mut d: Ptr<i8> = libcc2rs::strdup_refcount(Ptr::<i8>::from_string_literal(b"hello"));
    assert!((((!((d).is_null())) as i32) != 0));
    assert!(((((libcc2rs::cpp2rust_errno().read()) == 99) as i32) != 0));
    libcc2rs::free_refcount((d).to_any());
    libcc2rs::cpp2rust_errno().write(0);
}
pub fn test_errno_from_fseek_2() {
    libcc2rs::cpp2rust_errno().write(0);
    let mut r: i32 =
        match libcc2rs::c_stdin().with_mut(|__v: &mut CFile| __v.seek(0_i64, ::libc::SEEK_SET)) {
            -1 => -1,
            _ => 0,
        };
    assert!((((r == -1_i32) as i32) != 0));
    assert!(((((libcc2rs::cpp2rust_errno().read()) == libc::ESPIPE) as i32) != 0));
    libcc2rs::cpp2rust_errno().write(0);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_errno_0() });
    ({ test_errno_preserved_across_strdup_1() });
    ({ test_errno_from_fseek_2() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
