extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct label {
    pub name: *const libc::c_char,
    pub probe: Option<unsafe fn() -> i32>,
    pub mask: i32,
}
impl Default for label {
    fn default() -> Self {
        label {
            name: std::ptr::null(),
            probe: None,
            mask: 0_i32,
        }
    }
}
pub unsafe fn probe_two_0() -> i32 {
    return 1;
}
pub static mut table_1: [label; 2] = unsafe {
    [
        label {
            name: ((c"first").as_ptr().cast_mut()).cast_const(),
            probe: None,
            mask: ((1) << (4)),
        },
        label {
            name: ((c"second").as_ptr().cast_mut()).cast_const(),
            probe: (Some(probe_two_0)),
            mask: ((1) << (5)),
        },
    ]
};
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(
        (((((*table_1[(0) as usize].name.offset((0) as isize)) as i32) == ('f' as i32)) as i32)
            != 0)
    );
    assert!(
        (((((*table_1[(0) as usize].name.offset((4) as isize)) as i32) == ('t' as i32)) as i32)
            != 0)
    );
    assert!(((((table_1[(0) as usize].probe).is_none()) as i32) != 0));
    assert!(((((table_1[(0) as usize].mask) == (16)) as i32) != 0));
    assert!(
        (((((*table_1[(1) as usize].name.offset((0) as isize)) as i32) == ('s' as i32)) as i32)
            != 0)
    );
    assert!(((((unsafe { (table_1[(1) as usize].probe).unwrap()() }) == (1)) as i32) != 0));
    assert!(((((table_1[(1) as usize].mask) == (32)) as i32) != 0));
    let mut tail: *const libc::c_char = (&mut (*c"ab.cd".as_ptr().cast_mut().offset((2) as isize))
        as *mut libc::c_char)
        .cast_const();
    assert!((((((*tail.offset((0) as isize)) as i32) == ('.' as i32)) as i32) != 0));
    assert!((((((*tail.offset((1) as isize)) as i32) == ('c' as i32)) as i32) != 0));
    assert!((((((*tail.offset((2) as isize)) as i32) == ('d' as i32)) as i32) != 0));
    let mut have: i32 = 0;
    let mut p: *mut ::libc::c_void = if (have != 0) {
        (table_1[(0) as usize].name as *mut ::libc::c_void)
    } else {
        (c"".as_ptr().cast_mut() as *mut libc::c_char as *mut ::libc::c_void)
    };
    assert!(
        (((((*(p as *const libc::c_char).offset((0) as isize)) as i32) == ('\0' as i32)) as i32)
            != 0)
    );
    have = 1;
    p = if (have != 0) {
        (table_1[(0) as usize].name as *mut ::libc::c_void)
    } else {
        (c"".as_ptr().cast_mut() as *mut libc::c_char as *mut ::libc::c_void)
    };
    assert!(
        (((((*(p as *const libc::c_char).offset((0) as isize)) as i32) == ('f' as i32)) as i32)
            != 0)
    );
    return 0;
}
