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
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p1: (f64, i32) = ((1 as f64), 2);
    assert!(((p1.0) == (1_f64)));
    assert!(((p1.1) == (2)));
    let mut p2: (f64, i32) = ((3 as f64), 4);
    assert!(((p2.0) == (3_f64)));
    assert!(((p2.1) == (4)));
    let mut p3: (f64, i32) = ((5, 6).0.clone().into(), (5, 6).1.clone().into());
    assert!(((p3.0) == (5_f64)));
    assert!(((p3.1) == (6)));
    let arr: [i32; 2] = [1, 2];
    let mut n: u32 = 2_u32;
    let mut const_ptr_pairs: Vec<(*const i32, u64)> = (0..(1_usize) as usize)
        .map(|_| <(*const i32, u64)>::default())
        .collect::<Vec<_>>();
    const_ptr_pairs[(0_usize)] = (
        (arr.as_ptr().offset((1) as isize), n).0.clone().into(),
        (arr.as_ptr().offset((1) as isize), n).1.clone().into(),
    );
    assert!(
        ((*const_ptr_pairs[(0_usize)].0) == (2)) && ((const_ptr_pairs[(0_usize)].1) == (2_u64))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
