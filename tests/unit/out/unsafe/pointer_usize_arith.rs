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
    let mut arr: [i32; 8] = [10, 11, 12, 13, 14, 15, 16, 17];
    let mut p: *mut i32 = arr.as_mut_ptr();
    let mut q: *mut i32 = p.offset((1) as isize);
    assert!(((*q) == (11)));
    let mut r: *mut i32 = p.offset((3) as isize);
    assert!(((*r) == (13)));
    let mut s: *mut i32 = r.offset(-((2) as isize));
    assert!(((*s) == (11)));
    let mut diff: i64 = ((r as usize - p as usize) / ::std::mem::size_of::<i32>()) as i64;
    assert!(((diff) == (3_i64)));
    let mut idx: u64 = ((((r as usize - p as usize) / ::std::mem::size_of::<i32>()) as i64) as u64);
    assert!(((idx) == (3_u64)));
    let mut q2: *mut i32 = p;
    q2.prefix_inc();
    assert!(((*q2) == (11)));
    q2.postfix_inc();
    assert!(((*q2) == (12)));
    q2.prefix_dec();
    assert!(((*q2) == (11)));
    q2.postfix_dec();
    assert!(((*q2) == (10)));
    assert!(((q2) == (p)));
    let mut q3: *mut i32 = p;
    q3 = (q3).wrapping_add(4 as i32 as usize);
    assert!(((*q3) == (14)));
    q3 = (q3).wrapping_sub(2 as i32 as usize);
    assert!(((*q3) == (12)));
    let mut step: u64 = 2_u64;
    let mut q4: *mut i32 = p.offset((step) as isize);
    assert!(((*q4) == (12)));
    let mut v: i32 = (*p.offset((3) as isize));
    assert!(((v) == (13)));
    let mut v2: i32 = (*(p.offset((4) as isize)));
    assert!(((v2) == (14)));
    (*(p.offset((5) as isize))) = 99;
    assert!(((*p.offset((5) as isize)) == (99)));
    assert!(((arr[(5) as usize]) == (99)));
    let mut end: *mut i32 = p.offset((8) as isize);
    let mut sum: i32 = 0;
    let mut it: *mut i32 = p;
    'loop_: while ((it) != (end)) {
        sum += (*it);
        it.prefix_inc();
    }
    assert!(((sum) == ((((((((10) + (11)) + (12)) + (13)) + (14)) + (99)) + (16)) + (17))));
    let mut bytes: [u8; 8] = [0_u8, 1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8];
    let mut bp: *mut u8 = bytes.as_mut_ptr();
    let mut bq: *mut u8 = bp.offset((4) as isize);
    assert!((((*bq) as i32) == (4)));
    let mut bdiff: i64 = ((bq as usize - bp as usize) / ::std::mem::size_of::<u8>()) as i64;
    assert!(((bdiff) == (4_i64)));
    let mut cp: *const i32 = arr.as_mut_ptr().cast_const();
    let mut cq: *const i32 = cp.offset((2) as isize);
    assert!(((*cq) == (12)));
    let mut cdiff: i64 = ((cq as usize - cp as usize) / ::std::mem::size_of::<i32>()) as i64;
    assert!(((cdiff) == (2_i64)));
    let mut n: u64 = 3_u64;
    let mut q5: *mut i32 = arr.as_mut_ptr().offset((n) as isize);
    assert!(((*q5) == (13)));
    let mut q6: *mut i32 = (&mut arr[(n) as usize] as *mut i32);
    assert!(((q6) == (q5)));
    let mut matrix: [[i32; 4]; 3] = [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11]];
    let mut row1: *mut i32 = (&mut matrix[(1) as usize][(0) as usize] as *mut i32);
    assert!(((*row1.offset((2) as isize)) == (6)));
    let mut back: *mut i32 = end.offset(-((1) as isize));
    assert!(((*back) == (17)));
    return 0;
}
