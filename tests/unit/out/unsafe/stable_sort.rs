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
    let mut arr1: [i32; 5] = [5, 2, 8, 1, 3];
    {
        let len = arr1
            .as_mut_ptr()
            .offset((5) as isize)
            .offset_from(arr1.as_mut_ptr()) as usize;
        ::std::slice::from_raw_parts_mut(arr1.as_mut_ptr(), len).sort_by(|x, y| {
            if (lambda_0 {}).call(*x, *y) {
                std::cmp::Ordering::Less
            } else if (lambda_0 {}).call(*y, *x) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        })
    };
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_0 {}
impl lambda_0 {
    pub unsafe fn operator_call(mut x: i32, mut y: i32) -> bool {
        return ((x) < (y));
    }
}
impl Callable2<i32, i32, bool> for lambda_0 {
    fn call(&self, a1: i32, a2: i32) -> bool {
        unsafe { lambda_0::operator_call(a1, a2) }
    }
}
impl lambda_0 {
    pub fn to_free_function(&self) -> Option<unsafe fn(i32, i32) -> bool> {
        Some(lambda_0::operator_call)
    }
}
