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
pub struct Item {
    pub flags: u8,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut item: Item = Item { flags: 0_u8 };
    let mut ptr: *mut Item = (&mut item as *mut Item);
    match ({ ((1) << (0)) }, &mut ((*(ptr)).flags)) {
        (__tmp_0, __tmp_1) => {
            (*__tmp_1) = (((*__tmp_1) as i32) | (__tmp_0 as i32)) as u8;
        }
    };
    match ({ ((1) << (1)) }, &mut ((*(ptr)).flags)) {
        (__tmp_2, __tmp_3) => {
            (*__tmp_3) = (((*__tmp_3) as i32) | (__tmp_2 as i32)) as u8;
        }
    };
    assert!((((*ptr).flags as i32) == (3)));
    match ({ ((!((1) << (0)) as u8) as i32) }, &mut ((*(ptr)).flags)) {
        (__tmp_4, __tmp_5) => {
            (*__tmp_5) = (((*__tmp_5) as i32) & (__tmp_4 as i32)) as u8;
        }
    };
    assert!((((*ptr).flags as i32) == (2)));
    let mut bits: [u8; 4] = [0_u8; 4];
    match (
        { ((((1) << ((5) & (7))) as u8) as i32) },
        &mut ((bits)[((5) / (8)) as usize]),
    ) {
        (__tmp_6, __tmp_7) => {
            (*__tmp_7) = (((*__tmp_7) as i32) | (__tmp_6 as i32)) as u8;
        }
    };
    match (
        { ((((1) << ((13) & (7))) as u8) as i32) },
        &mut ((bits)[((13) / (8)) as usize]),
    ) {
        (__tmp_8, __tmp_9) => {
            (*__tmp_9) = (((*__tmp_9) as i32) | (__tmp_8 as i32)) as u8;
        }
    };
    assert!(((bits[(0) as usize] as i32) == (32)));
    assert!(((bits[(1) as usize] as i32) == (32)));
    assert!(((bits[(2) as usize] as i32) == (0)));
    if (((*ptr).flags as i32) != (0)) {
        match ({ ((!((1) << (1)) as u8) as i32) }, &mut ((*(ptr)).flags)) {
            (__tmp_10, __tmp_11) => {
                (*__tmp_11) = (((*__tmp_11) as i32) & (__tmp_10 as i32)) as u8;
            }
        };
    }
    assert!((((*ptr).flags as i32) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
