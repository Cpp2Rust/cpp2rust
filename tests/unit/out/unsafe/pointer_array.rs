extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Copy, Clone)]
pub struct StackArray {
    pub arr: [*mut i32; 3],
}
impl Default for StackArray {
    fn default() -> Self {
        StackArray {
            arr: [Default::default(); 3],
        }
    }
}
pub unsafe fn IncrementAll_0(s: *mut StackArray) {
    let mut i: i32 = 0;
    'loop_: while ((i) < (3)) {
        (*(*s).arr[(i) as usize]) += 1;
        i.prefix_inc();
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: i32 = 0;
    let mut s: StackArray = StackArray {
        arr: [
            (&mut x as *mut i32),
            (&mut x as *mut i32),
            (&mut x as *mut i32),
        ],
    };
    (unsafe {
        let _s: *mut StackArray = &mut s as *mut StackArray;
        IncrementAll_0(_s)
    });
    return x;
}
