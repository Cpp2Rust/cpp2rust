extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn cmp_int_0(mut a: *const ::libc::c_void, mut b: *const ::libc::c_void) -> i32 {
    let mut x: i32 = (*(a as *const i32));
    let mut y: i32 = (*(b as *const i32));
    return ((((x) > (y)) as i32) - (((x) < (y)) as i32));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut arr: [i32; 8] = [5, 2, 9, 1, 7, 3, 8, 4];
    libc::qsort(
        (arr.as_mut_ptr() as *mut i32 as *mut ::libc::c_void),
        8_usize,
        ::std::mem::size_of::<i32>(),
        Some(std::mem::transmute::<
            *const (),
            unsafe extern "C" fn(*const ::libc::c_void, *const ::libc::c_void) -> i32,
        >(cmp_int_0 as *const ())),
    );
    let mut i: i32 = 0;
    'loop_: while ((((i) < (7)) as i32) != 0) {
        assert!(((((arr[(i) as usize]) <= (arr[((i) + (1)) as usize])) as i32) != 0));
        i.prefix_inc();
    }
    let mut key: i32 = 7;
    let mut hit: *mut i32 = (libc::bsearch(
        ((&mut key as *mut i32) as *const i32 as *const ::libc::c_void),
        (arr.as_mut_ptr() as *const i32 as *const ::libc::c_void),
        8_usize,
        ::std::mem::size_of::<i32>(),
        Some(std::mem::transmute::<
            *const (),
            unsafe extern "C" fn(*const ::libc::c_void, *const ::libc::c_void) -> i32,
        >(cmp_int_0 as *const ())),
    ) as *mut i32);
    assert!((((!((hit).is_null())) as i32) != 0));
    assert!(((((*hit) == (7)) as i32) != 0));
    let mut miss_key: i32 = 42;
    let mut miss: *mut i32 = (libc::bsearch(
        ((&mut miss_key as *mut i32) as *const i32 as *const ::libc::c_void),
        (arr.as_mut_ptr() as *const i32 as *const ::libc::c_void),
        8_usize,
        ::std::mem::size_of::<i32>(),
        Some(std::mem::transmute::<
            *const (),
            unsafe extern "C" fn(*const ::libc::c_void, *const ::libc::c_void) -> i32,
        >(cmp_int_0 as *const ())),
    ) as *mut i32);
    assert!(((((miss).is_null()) as i32) != 0));
    return 0;
}
