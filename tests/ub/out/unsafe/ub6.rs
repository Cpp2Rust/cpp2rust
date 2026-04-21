extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Copy, Clone, Default)]
pub struct Pair {
    pub x1: *mut i32,
    pub x2: *mut i32,
}
pub unsafe fn mkPair_0(x1: *mut i32, x2: *mut i32) -> Pair {
    return Pair { x1: x1, x2: x2 };
}
pub unsafe fn fill_1(arr: *mut Option<Box<[*mut i32]>>, n1: *mut i32) {
    let mut n2: i32 = (*n1);
    let mut pair: Pair = (unsafe {
        let _x1: *mut i32 = n1;
        let _x2: *mut i32 = &mut n2 as *mut i32;
        mkPair_0(_x1, _x2)
    });
    (*arr).as_mut().unwrap()[(0_u64) as usize] = (pair.x1);
    (*arr).as_mut().unwrap()[(1_u64) as usize] = (pair.x2);
}
pub unsafe fn any_2(arr: *mut Option<Box<[*mut i32]>>, n1: *mut i32) -> bool {
    let mut out: bool = false;
    let mut i: i32 = 0;
    'loop_: while ((i) < (*n1)) {
        out = ((out) || ((*(*arr).as_mut().unwrap()[(i as u64) as usize]) == (0)));
        i.prefix_inc();
    }
    return out;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut n: i32 = 2;
    let mut arr: Option<Box<[*mut i32]>> = Some(
        (0..(n as u64))
            .map(|_| <*mut i32>::default())
            .collect::<Box<[_]>>(),
    );
    (unsafe {
        let _arr: *mut Option<Box<[*mut i32]>> = &mut arr as *mut Option<Box<[*mut i32]>>;
        let _n1: *mut i32 = &mut n as *mut i32;
        fill_1(_arr, _n1)
    });
    return ((unsafe {
        let _arr: *mut Option<Box<[*mut i32]>> = &mut arr as *mut Option<Box<[*mut i32]>>;
        let _n1: *mut i32 = &mut n as *mut i32;
        any_2(_arr, _n1)
    }) as i32);
}
