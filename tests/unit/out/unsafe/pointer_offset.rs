extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut out: i32 = 0;
    let mut arr: [i32; 5] = [1, 2, 3, 4, 0];
    let mut ptr: *const i32 = (&mut arr[(0) as usize] as *mut i32).cast_const();
    'loop_: while ((*ptr) != (0)) {
        out += (*ptr);
        ptr.prefix_inc();
    }
    let mut ptr: *mut i32 = (&mut arr[(1) as usize] as *mut i32);
    'loop_: while ((*ptr) != (4)) {
        out += (*ptr);
        ptr.postfix_inc();
    }
    let mut ptr: *mut i32 = (&mut arr[(4) as usize] as *mut i32);
    'loop_: while ((*ptr) != (1)) {
        out += (*ptr);
        ptr.postfix_dec();
    }
    let mut ptr: *const i32 = (&mut arr[(3) as usize] as *mut i32).cast_const();
    'loop_: while ((*ptr) != (2)) {
        out += (*ptr);
        ptr.prefix_dec();
    }
    let mut ptr: *mut i32 = (&mut arr[(0) as usize] as *mut i32);
    'loop_: while ((*ptr) != (0)) {
        out += (*ptr);
        ptr = ptr.offset((1) as isize);
    }
    let mut ptr: *mut i32 = (&mut arr[(0) as usize] as *mut i32);
    let mut i: i32 = 0;
    'loop_: while ((i) < (5)) {
        out += (*ptr.offset((i) as isize));
        i.prefix_inc();
    }
    return out;
}
