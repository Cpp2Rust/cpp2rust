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
    let mut src: [u32; 3] = [1_u32, 2_u32, 3_u32];
    let mut v1: Vec<u32> = {
        let mut __v = Vec::new();
        let mut __it = src.as_mut_ptr();
        while __it != src.as_mut_ptr().offset((3) as isize) {
            __v.push(u32::try_from((*__it.iter_deref()).clone()).ok().unwrap());
            __it.iter_inc();
        }
        __v
    };
    assert!(((v1.len()) == (3_usize)));
    assert!(
        (((v1[(0_usize)]) == (1_u32)) && ((v1[(1_usize)]) == (2_u32)))
            && ((v1[(2_usize)]) == (3_u32))
    );
    let mut v2: Vec<u64> = {
        let mut __v = Vec::new();
        let mut __it = src.as_mut_ptr();
        while __it != src.as_mut_ptr().offset((3) as isize) {
            __v.push(u64::try_from((*__it.iter_deref()).clone()).ok().unwrap());
            __it.iter_inc();
        }
        __v
    };
    assert!(((v2.len()) == (3_usize)));
    assert!(
        (((v2[(0_usize)]) == (1_u64)) && ((v2[(1_usize)]) == (2_u64)))
            && ((v2[(2_usize)]) == (3_u64))
    );
    let mut v3: Vec<i32> = {
        let mut __v = Vec::new();
        let mut __it = src.as_mut_ptr();
        while __it != src.as_mut_ptr().offset((3) as isize) {
            __v.push(i32::try_from((*__it.iter_deref()).clone()).ok().unwrap());
            __it.iter_inc();
        }
        __v
    };
    assert!(((v3.len()) == (3_usize)));
    assert!((((v3[(0_usize)]) == (1)) && ((v3[(1_usize)]) == (2))) && ((v3[(2_usize)]) == (3)));
    let src1: [u32; 3] = [1_u32, 2_u32, 3_u32];
    let mut v4: Vec<u32> = {
        let mut __v = Vec::new();
        let mut __it = src1.as_ptr();
        while __it != src1.as_ptr().add(src1.len()) {
            __v.push(u32::try_from((*__it.iter_deref()).clone()).ok().unwrap());
            __it.iter_inc();
        }
        __v
    };
    assert!(((v4.len()) == (3_usize)));
    assert!(
        (((v4[(0_usize)]) == (1_u32)) && ((v4[(1_usize)]) == (2_u32)))
            && ((v4[(2_usize)]) == (3_u32))
    );
    let mut buf: [u8; 5] = [10_u8, 20_u8, 30_u8, 40_u8, 50_u8];
    let mut start: *const u8 = (buf.as_mut_ptr()).cast_const();
    let mut len: usize = 5_usize;
    let mut v5: Vec<u8> = {
        let mut __v = Vec::new();
        let mut __it = start;
        while __it != start.offset((len) as isize) {
            __v.push(u8::try_from((*__it.iter_deref()).clone()).ok().unwrap());
            __it.iter_inc();
        }
        __v
    };
    assert!(((v5.len()) == (5_usize)));
    assert!(((v5[(0_usize)] as i32) == (10)) && ((v5[(4_usize)] as i32) == (50)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
