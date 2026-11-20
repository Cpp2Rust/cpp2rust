extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() {
    let mut m: UnsafeMap<i32, (i32, Vec<libc::c_char>)> = UnsafeMap::new();
    let mut key: i32 = 1;
    assert!((((*m.subscript(&key, || vec![0])).len() - 1) == (0_usize)));
    assert!(
        (((*(*m.subscript(&key, || vec![0])).as_ptr()) as i32) == (('\0' as libc::c_char) as i32))
    );
    assert!(((m.len()) == (1_usize)));
}
pub unsafe fn test_1() {
    let mut m: UnsafeMap<i32, (i32, Vec<libc::c_char>)> = UnsafeMap::new();
    assert!(
        (((*m.subscript_move(&mut 1, |__k| cpp_move_unsafe!(__k), || vec![0])).len() - 1)
            == (0_usize))
    );
    assert!(
        (((*(*m.subscript_move(&mut 1, |__k| cpp_move_unsafe!(__k), || vec![0])).as_ptr()) as i32)
            == (('\0' as libc::c_char) as i32))
    );
    assert!(((m.len()) == (1_usize)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_0() });
    (unsafe { test_1() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
