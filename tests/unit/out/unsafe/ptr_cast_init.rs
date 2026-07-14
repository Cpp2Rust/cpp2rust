extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct header {
    pub tag: i32,
    pub size: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct view {
    pub tag: i32,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let text: [libc::c_char; 3] = std::mem::transmute(*b"hi\0");
    let mut cp: *const libc::c_char = text.as_ptr();
    let mut u: *mut u8 = (cp as *mut u8);
    assert!((((((*u.offset((0) as isize)) as i32) == ('h' as i32)) as i32) != 0));
    assert!((((((*u.offset((1) as isize)) as i32) == ('i' as i32)) as i32) != 0));
    let mut h: header = header { tag: 7, size: 32 };
    let mut hp: *mut header = (&mut h as *mut header);
    let mut v: *mut view = (hp as *mut view);
    assert!((((((*v).tag) == (7)) as i32) != 0));
    let mut data: [libc::c_char; 3] = std::mem::transmute(*b"hi\0");
    let mut vp: *mut ::libc::c_void =
        (data.as_mut_ptr() as *mut libc::c_char as *mut ::libc::c_void);
    let mut n: i32 = 2;
    let mut sel: *mut libc::c_char = (if ((((n) < (100)) as i32) != 0) {
        vp
    } else {
        (0 as *mut ::libc::c_void)
    } as *mut libc::c_char);
    assert!((((!((sel).is_null())) as i32) != 0));
    assert!((((((*sel.offset((0) as isize)) as i32) == ('h' as i32)) as i32) != 0));
    n = 200;
    sel = (if ((((n) < (100)) as i32) != 0) {
        vp
    } else {
        (0 as *mut ::libc::c_void)
    } as *mut libc::c_char);
    assert!(((((sel).is_null()) as i32) != 0));
    return 0;
}
