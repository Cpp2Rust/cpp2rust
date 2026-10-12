extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn array_ref_0(a: *mut [u64; 3]) -> u64 {
    (*a)[(0) as usize] = ((*a)[(0) as usize]).wrapping_add(1_u64);
    return (*a)[((3_u64 as u64).wrapping_sub(1_u64)) as usize];
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct PtrCtor_unsigned_long_ {
    pub v: u64,
}
impl PtrCtor_unsigned_long_ {
    pub unsafe fn new(mut p: *const u64) -> Self {
        let mut this = Self {
            v: (*p.offset((1) as isize)),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RefCtor_unsigned_long_ {
    pub v: u64,
}
impl RefCtor_unsigned_long_ {
    pub unsafe fn new(x: *const u64) -> Self {
        let mut this = Self {
            v: (*x).wrapping_add(1_u64),
        };
        this
    }
}
pub unsafe fn neg_ll_1() -> i64 {
    return -9000000_i64;
}
pub unsafe fn neg_char_2() -> libc::c_char {
    return (b'\x80' as libc::c_char);
}
pub unsafe fn neg_int_3() -> i32 {
    return -1;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a1: [usize; 3] = [1_usize, 2_usize, 3_usize];
    assert!(((unsafe { array_ref_0(&mut *(&raw mut a1).cast::<[u64; 3]>(),) }) == (3_u64)));
    assert!(((a1[(0) as usize]) == (2_usize)));
    let mut a2: [usize; 2] = [4_usize, 5_usize];
    let mut pc: PtrCtor_unsigned_long_ = PtrCtor_unsigned_long_::new({
        ((a2.as_mut_ptr()).cast_const() as *const usize).cast::<u64>()
    });
    assert!(((pc.v) == (5_u64)));
    let mut v1: usize = 6_usize;
    let mut rc: RefCtor_unsigned_long_ =
        RefCtor_unsigned_long_::new({ &*(&raw const v1).cast::<u64>() });
    assert!(((rc.v) == (7_u64)));
    assert!(((unsafe { neg_ll_1() }) == (-9000000_i32 as i64)));
    assert!((((unsafe { neg_char_2() }) as i32) == ((b'\x80' as libc::c_char) as i32)));
    assert!(((unsafe { neg_int_3() }) == (-1_i32)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
