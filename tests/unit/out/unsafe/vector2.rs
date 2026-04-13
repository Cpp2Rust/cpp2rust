extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fn_0(v: *mut Vec<i32>, mut v3: Vec<i32>) {
    (*v).push(20);
    let mut x: i32 = 0 as i32;
    let mut v2: Vec<i32> = Vec::new();
    let mut v4: *mut Vec<i32> = (&mut v3 as *mut Vec<i32>);
    v2.push(0);
    v2.push(1);
    v2.push(3);
    x = (*v)[(2_u64) as usize];
    v2[(0_u64) as usize] = 1;
    (if true { &mut v3 } else { &mut (*v) })[(0_u64) as usize] = 7;
    v2 = (*v).clone();
    (*v4)[(1_u64) as usize] = 13;
    assert!(((x) == (6)));
    assert!(((*((*v).first_mut().unwrap())) == (4)));
    assert!((((*v)[(1_u64) as usize]) == (5)));
    assert!((((*v)[(2_u64) as usize]) == (6)));
    assert!(((*((*v).last_mut().unwrap())) == (20)));
    assert!(((v2[(0_u64) as usize]) == (4)));
    assert!(((v2[(1_u64) as usize]) == (5)));
    assert!(((v2[(2_u64) as usize]) == (6)));
    assert!(((v3[(0_u64) as usize]) == (7)));
    assert!(((v3[(1_u64) as usize]) == (13)));
    (*v).push(20);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();
    v.push(4);
    v.push(5);
    v.push(6);
    v2.push(8);
    v2.push(9);
    (unsafe {
        let _v: *mut Vec<i32> = &mut v as *mut Vec<i32>;
        let _v3: Vec<i32> = v2.clone();
        fn_0(_v, _v3)
    });
    return 0;
}
