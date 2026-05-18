extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;

// vector_iter_range_ctor.rs
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut src: [u32; 3] = [1_u32, 2_u32, 3_u32];
    let mut v1: Vec<u32> = core::slice::from_raw_parts(
        src.as_mut_ptr(),
        (src.as_mut_ptr().offset((3) as isize)).offset_from(src.as_mut_ptr()) as usize,
    )
    .iter()
    .map(|x| u32::try_from(x.clone()).ok().unwrap())
    .collect();
    assert!(((v1.len() as u64) == (3_u64)));
    assert!(
        (((v1[(0_u64) as usize]) == (1_u32)) && ((v1[(1_u64) as usize]) == (2_u32)))
            && ((v1[(2_u64) as usize]) == (3_u32))
    );
    let mut v2: Vec<u64> = core::slice::from_raw_parts(
        src.as_mut_ptr(),
        (src.as_mut_ptr().offset((3) as isize)).offset_from(src.as_mut_ptr()) as usize,
    )
    .iter()
    .map(|x| u64::try_from(x.clone()).ok().unwrap())
    .collect();
    assert!(((v2.len() as u64) == (3_u64)));
    assert!(
        (((v2[(0_u64) as usize]) == (1_u64)) && ((v2[(1_u64) as usize]) == (2_u64)))
            && ((v2[(2_u64) as usize]) == (3_u64))
    );
    let mut v3: Vec<i32> = core::slice::from_raw_parts(
        src.as_mut_ptr(),
        (src.as_mut_ptr().offset((3) as isize)).offset_from(src.as_mut_ptr()) as usize,
    )
    .iter()
    .map(|x| i32::try_from(x.clone()).ok().unwrap())
    .collect();
    assert!(((v3.len() as u64) == (3_u64)));
    assert!(
        (((v3[(0_u64) as usize]) == (1)) && ((v3[(1_u64) as usize]) == (2)))
            && ((v3[(2_u64) as usize]) == (3))
    );
    let src1: [u32; 3] = [1_u32, 2_u32, 3_u32];
    let mut v4: Vec<u32> = core::slice::from_raw_parts(
        src1.as_ptr(),
        (src1.as_ptr().add(src1.len())).offset_from(src1.as_ptr()) as usize,
    )
    .to_vec();
    assert!(((v4.len() as u64) == (3_u64)));
    assert!(
        (((v4[(0_u64) as usize]) == (1_u32)) && ((v4[(1_u64) as usize]) == (2_u32)))
            && ((v4[(2_u64) as usize]) == (3_u32))
    );
    let mut buf: [u8; 5] = [10_u8, 20_u8, 30_u8, 40_u8, 50_u8];
    let mut start: *const u8 = buf.as_mut_ptr().cast_const();
    let mut len: u64 = 5_u64;
    let mut v5: Vec<u8> = core::slice::from_raw_parts(
        start,
        (start.offset((len) as isize)).offset_from(start) as usize,
    )
    .to_vec();
    assert!(((v5.len() as u64) == (5_u64)));
    assert!(((v5[(0_u64) as usize] as i32) == (10)) && ((v5[(4_u64) as usize] as i32) == (50)));
    return 0;
}
