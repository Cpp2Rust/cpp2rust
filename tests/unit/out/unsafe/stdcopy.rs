extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut input: [i32; 3] = [1, 2, 3];
    let mut output: [i32; 3] = [0 as i32; 3];
    {
        let mut outptr = output.as_mut_ptr().clone();
        let mut curr = input.as_mut_ptr().clone();
        while curr < input.as_mut_ptr().offset((3) as isize) {
            *outptr = (*curr).clone().into();
            curr = curr.offset(1);
            outptr = outptr.offset(1);
        }
        outptr
    };
    return 0;
}
