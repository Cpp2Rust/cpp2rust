extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn strlen_0(mut ptr: *mut u8) -> u32 {
    let mut count: u32 = 0_u32;
    'loop_: while (((*ptr.postfix_inc()) as i32) != (('\0' as u8) as i32)) {
        count.prefix_inc();
    }
    return count;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut string: [u8; 6] = [
        ('h' as u8),
        ('e' as u8),
        ('l' as u8),
        ('l' as u8),
        ('o' as u8),
        ('\0' as u8),
    ];
    return ((unsafe {
        let _ptr: *mut u8 = (&mut string[(0) as usize] as *mut u8);
        strlen_0(_ptr)
    }) as i32);
}
