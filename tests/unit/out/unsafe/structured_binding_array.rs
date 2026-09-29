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
    let mut arr: [i32; 3] = [1, 2, 3];
    let mut __decomp_0: [i32; 3] = std::array::from_fn::<_, 3, _>(|__i: usize| arr[(__i)]);
    __decomp_0[(0) as usize] = 10;
    assert!(((__decomp_0[(0) as usize]) == (10)));
    assert!(((__decomp_0[(1) as usize]) == (2)));
    assert!(((__decomp_0[(2) as usize]) == (3)));
    assert!(((arr[(0) as usize]) == (1)));
    let __decomp_1: *mut [i32; 3] = &mut arr;
    (*__decomp_1)[(0) as usize] = 7;
    (*__decomp_1)[(2) as usize] += (*__decomp_1)[(1) as usize];
    assert!(((arr[(0) as usize]) == (7)));
    assert!(((arr[(2) as usize]) == (5)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
