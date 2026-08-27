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
pub struct Point {
    pub x: i32,
    pub y: i32,
}
pub unsafe fn set_0(mut ref_: *mut i32, mut val: i32) {
    (*ref_) = val;
}
pub unsafe fn read_1(mut ref_: *mut i32) -> i32 {
    let r: *mut i32 = ref_;
    return (*r);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut i1: i32 = 10;
    let mut ref_1: *mut i32 = &mut i1;
    (*ref_1) = 20;
    let i2: *mut i32 = ref_1;
    (*i2) += 5;
    write!(
        std::fs::File::from_raw_fd(
            std::io::stdout()
                .as_fd()
                .try_clone_to_owned()
                .unwrap()
                .into_raw_fd(),
        ),
        "{:}\n",
        i1,
    );
    let mut i3: i32 = 1;
    let mut i4: i32 = 2;
    let mut ref_3: *mut i32 = &mut i3;
    let mut ref_4: *mut i32 = &mut i4;
    (*ref_3) = (*ref_4);
    write!(
        std::fs::File::from_raw_fd(
            std::io::stdout()
                .as_fd()
                .try_clone_to_owned()
                .unwrap()
                .into_raw_fd(),
        ),
        "{:} {:}\n",
        i3,
        i4,
    );
    (unsafe { set_0(ref_1.clone(), 99) });
    write!(
        std::fs::File::from_raw_fd(
            std::io::stdout()
                .as_fd()
                .try_clone_to_owned()
                .unwrap()
                .into_raw_fd(),
        ),
        "{:} {:}\n",
        i1,
        (unsafe { read_1(ref_1.clone(),) }),
    );
    let mut point: Point = Point { x: 3, y: 4 };
    let mut point_ref: *mut Point = &mut point;
    (*point_ref).x = 30;
    (*point_ref).y = 40;
    write!(
        std::fs::File::from_raw_fd(
            std::io::stdout()
                .as_fd()
                .try_clone_to_owned()
                .unwrap()
                .into_raw_fd(),
        ),
        "{:} {:}\n",
        point.x,
        point.y,
    );
    return 0;
}
