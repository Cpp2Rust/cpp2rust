extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn All_0(arr: *mut Option<Box<[i32]>>, mut N: i32, mut element: i32) {
    let mut all: Option<Box<[i32]>> = Some(
        (0..(N as u64))
            .map(|_| <i32>::default())
            .collect::<Box<[_]>>(),
    );
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        all.as_mut().unwrap()[(i as u64) as usize] = element;
        i.prefix_inc();
    }
    (*arr) = all;
}
pub unsafe fn Consume_1(mut arr: Option<Box<[i32]>>, mut N: i32) -> i32 {
    let mut sum: i32 = 0;
    let mut i: i32 = -1_i32;
    'loop_: while ((i.prefix_inc()) < (N)) {
        sum += arr.as_mut().unwrap()[(i as u64) as usize];
    }
    return sum;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let N: i32 = 10;
    let mut arr: Option<Box<[i32]>> = Some(
        (0..(N as u64))
            .map(|_| <i32>::default())
            .collect::<Box<[_]>>(),
    );
    (unsafe {
        let _arr: *mut Option<Box<[i32]>> = &mut arr as *mut Option<Box<[i32]>>;
        let _N: i32 = N;
        let _element: i32 = 1;
        All_0(_arr, _N, _element)
    });
    return (unsafe {
        let _arr: Option<Box<[i32]>> = arr;
        let _N: i32 = N;
        Consume_1(_arr, _N)
    });
}
