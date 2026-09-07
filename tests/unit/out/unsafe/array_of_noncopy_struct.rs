extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, Default)]
pub struct NonCopy {
    pub data: Vec<i32>,
    pub tag: i32,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut arr: [NonCopy; 3] = std::array::from_fn::<_, 3, _>(|_| <NonCopy>::default());
    arr[(0) as usize].tag = 7;
    arr[(1) as usize].data.push(42);
    assert!(((arr[(0) as usize].tag) == (7)));
    assert!(((arr[(1) as usize].data.len()) == (1_usize)));
    assert!(((arr[(1) as usize].data[(0_usize)]) == (42)));
    assert!(((arr[(2) as usize].tag) == (0)));
    assert!(((arr[(2) as usize].data.len()) == (0_usize)));
    return 0;
}
