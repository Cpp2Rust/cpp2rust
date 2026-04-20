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
    let mut N: i32 = 100000000;
    let mut sum: i64 = 0_i64;
    let mut k: i32 = 0;
    'loop_: while ((k) < (35)) {
        let mut array: Option<Box<[i32]>> = Some(
            (0..(N as u64))
                .map(|_| <i32>::default())
                .collect::<Box<[_]>>(),
        );
        let mut i: i32 = 0;
        'loop_: while ((i) < (N)) {
            array.as_mut().unwrap()[(i as u64) as usize] = i;
            i.prefix_inc();
        }
        let mut i: i32 = 0;
        'loop_: while ((i) < (N)) {
            sum += (array.as_mut().unwrap()[(i as u64) as usize] as i64);
            i.prefix_inc();
        }
        k.prefix_inc();
    }
    return (sum as i32);
}
