extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct table {
    pub rows: [[libc::c_char; 10]; 3],
    pub count: usize,
}
pub static mut T1_0: std::cell::LazyCell<table> = std::cell::LazyCell::new(|| unsafe {
    table {
        rows: [std::mem::transmute(*b"alpha\0\0\0\0\0"), [0; 10], [0; 10]],
        count: 1_usize,
    }
});
pub static mut T2_1: std::cell::LazyCell<table> = std::cell::LazyCell::new(|| unsafe {
    table {
        rows: [
            std::mem::transmute(*b"alpha\0\0\0\0\0"),
            std::mem::transmute(*b"beta\0\0\0\0\0\0"),
            [0; 10],
        ],
        count: 2_usize,
    }
});
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        (((((*std::cell::LazyCell::force_mut(&mut *&raw mut T1_0)).count) == (1_usize)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut T1_0)).rows[(0) as usize].as_ptr(),
            (c"alpha".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        (((((*std::cell::LazyCell::force_mut(&mut *&raw mut T1_0)).rows[(1) as usize][(0) as usize]
            as i32)
            == ('\0' as i32)) as i32)
            != 0)
    );
    assert!(
        (((((*std::cell::LazyCell::force_mut(&mut *&raw mut T2_1)).count) == (2_usize)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut T2_1)).rows[(0) as usize].as_ptr(),
            (c"alpha".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut T2_1)).rows[(1) as usize].as_ptr(),
            (c"beta".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        (((((*std::cell::LazyCell::force_mut(&mut *&raw mut T2_1)).rows[(2) as usize][(0) as usize]
            as i32)
            == ('\0' as i32)) as i32)
            != 0)
    );
    let mut local: table = table {
        rows: [
            std::mem::transmute(*b"one\0\0\0\0\0\0\0"),
            std::mem::transmute(*b"two\0\0\0\0\0\0\0"),
            std::mem::transmute(*b"three\0\0\0\0\0"),
        ],
        count: 3_usize,
    };
    assert!(
        ((((libc::strcmp(
            (local.rows[(2) as usize].as_mut_ptr()).cast_const(),
            (c"three".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    local.rows[(1) as usize][(0) as usize] = (('T' as i32) as libc::c_char);
    assert!(
        ((((libc::strcmp(
            (local.rows[(1) as usize].as_mut_ptr()).cast_const(),
            (c"Two".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (local.rows[(0) as usize].as_mut_ptr()).cast_const(),
            (c"one".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    let mut p: *const libc::c_char = (local.rows[(2) as usize].as_mut_ptr()).cast_const();
    assert!((((((*p.offset((0) as isize)) as i32) == ('t' as i32)) as i32) != 0));
    assert!(((((local.count) == (3_usize)) as i32) != 0));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const T1_0);
    std::cell::LazyCell::force(&*&raw const T2_1);
}
