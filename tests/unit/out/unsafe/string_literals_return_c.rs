extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn get_greeting_0() -> *const u8 {
    return b"hello\0".as_ptr().cast_mut().cast_const();
}
pub unsafe fn get_empty_1() -> *const u8 {
    return b"\0".as_ptr().cast_mut().cast_const();
}
pub unsafe fn get_branch_2(mut x: i32) -> *const u8 {
    if ((x) > (0)) {
        return b"positive\0".as_ptr().cast_mut().cast_const();
    }
    return b"non-positive\0".as_ptr().cast_mut().cast_const();
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: *const u8 = (unsafe { get_greeting_0() });
    assert!((((*a.offset((0) as isize)) as i32) == ('h' as i32)));
    assert!((((*a.offset((4) as isize)) as i32) == ('o' as i32)));
    assert!((((*a.offset((5) as isize)) as i32) == ('\0' as i32)));
    let mut b: *const u8 = (unsafe { get_empty_1() });
    assert!((((*b.offset((0) as isize)) as i32) == ('\0' as i32)));
    let mut c: *const u8 = (unsafe {
        let _x: i32 = 1;
        get_branch_2(_x)
    });
    assert!((((*c.offset((0) as isize)) as i32) == ('p' as i32)));
    assert!((((*c.offset((7) as isize)) as i32) == ('e' as i32)));
    assert!((((*c.offset((8) as isize)) as i32) == ('\0' as i32)));
    let mut d: *const u8 = (unsafe {
        let _x: i32 = -1_i32;
        get_branch_2(_x)
    });
    assert!((((*d.offset((0) as isize)) as i32) == ('n' as i32)));
    assert!((((*d.offset((11) as isize)) as i32) == ('e' as i32)));
    assert!((((*d.offset((12) as isize)) as i32) == ('\0' as i32)));
    return 0;
}
