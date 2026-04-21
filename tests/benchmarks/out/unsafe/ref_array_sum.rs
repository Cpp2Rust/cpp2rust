extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn initialize_0(array: *mut Option<Box<[i32]>>, mut N: i32) {
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        (*array).as_mut().unwrap()[(i as u64) as usize] = i;
        i.prefix_inc();
    }
}
pub unsafe fn sum_1(mut array: *mut Option<Box<[i32]>>, mut N: i32) -> i64 {
    let mut sum: i64 = 0_i64;
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        sum += ((*array).as_mut().unwrap()[(i as u64) as usize] as i64);
        i.prefix_inc();
    }
    return sum;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut N: i32 = 100000000;
    let mut out: i64 = 0_i64;
    let mut k: i32 = 0;
    'loop_: while ((k) < (35)) {
        let mut array: Option<Box<[i32]>> = Some(
            (0..(N as u64))
                .map(|_| <i32>::default())
                .collect::<Box<[_]>>(),
        );
        (unsafe {
            let _array: *mut Option<Box<[i32]>> = &mut array as *mut Option<Box<[i32]>>;
            let _N: i32 = N;
            initialize_0(_array, _N)
        });
        out += (unsafe {
            let _array: *mut Option<Box<[i32]>> = (&mut array as *mut Option<Box<[i32]>>);
            let _N: i32 = N;
            sum_1(_array, _N)
        });
        k.prefix_inc();
    }
    return (out as i32);
}
