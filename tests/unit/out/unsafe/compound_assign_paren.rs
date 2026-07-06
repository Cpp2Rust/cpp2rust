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
pub struct Item {
    pub flags: u8,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut item: Item = Item { flags: 0_u8 };
    let mut ptr: *mut Item = (&mut item as *mut Item);
    ((*(ptr)).flags = (((*(ptr)).flags as i32) | ((1) << (0))) as u8);
    ((*(ptr)).flags = (((*(ptr)).flags as i32) | ((1) << (1))) as u8);
    assert!((((*ptr).flags as i32) == (3)));
    ((*(ptr)).flags = (((*(ptr)).flags as i32) & ((!((1) << (0)) as u8) as i32)) as u8);
    assert!((((*ptr).flags as i32) == (2)));
    let mut bits: [u8; 4] = [0_u8, 0_u8, 0_u8, 0_u8];
    ((bits)[((5) / (8)) as usize] =
        (((bits)[((5) / (8)) as usize] as i32) | ((((1) << ((5) & (7))) as u8) as i32)) as u8);
    ((bits)[((13) / (8)) as usize] =
        (((bits)[((13) / (8)) as usize] as i32) | ((((1) << ((13) & (7))) as u8) as i32)) as u8);
    assert!(((bits[(0) as usize] as i32) == (32)));
    assert!(((bits[(1) as usize] as i32) == (32)));
    assert!(((bits[(2) as usize] as i32) == (0)));
    if (((*ptr).flags as i32) != (0)) {
        ((*(ptr)).flags = (((*(ptr)).flags as i32) & ((!((1) << (1)) as u8) as i32)) as u8);
    }
    assert!((((*ptr).flags as i32) == (0)));
    return 0;
}
