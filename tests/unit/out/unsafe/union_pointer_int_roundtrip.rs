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
    let mut arr: [i32; 4] = [10, 20, 30, 40];
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union anon_0 {
        pub p: *mut i32,
        pub bits: u64,
    }
    impl Default for anon_0 {
        fn default() -> Self {
            unsafe { std::mem::zeroed() }
        }
    };
    let mut u: anon_0 = <anon_0>::default();
    u.p = (&mut arr[(1) as usize] as *mut i32);
    u.bits = ((u.bits as u64)
        .wrapping_add(((2_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64)))
        as u64;
    let mut q: *mut i32 = u.p;
    assert!(((((*q) == (40)) as i32) != 0));
    assert!(((((q) == (&mut arr[(3) as usize] as *mut i32)) as i32) != 0));
    u.bits = ((u.bits as u64)
        .wrapping_sub(((3_usize).wrapping_mul((::std::mem::size_of::<i32>() as usize)) as u64)))
        as u64;
    assert!(((((u.p) == (&mut arr[(0) as usize] as *mut i32)) as i32) != 0));
    assert!(((((*u.p) == (10)) as i32) != 0));
    u.p = arr.as_mut_ptr().offset((4) as isize);
    assert!(((((u.p) == (arr.as_mut_ptr().offset((4) as isize))) as i32) != 0));
    return 0;
}
