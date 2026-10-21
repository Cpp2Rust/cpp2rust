extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut g_0: std::cell::LazyCell<Vec<u8>> = std::cell::LazyCell::new(|| unsafe {
    (0..(3_usize) as usize)
        .map(|_| <u8>::default())
        .collect::<Vec<_>>()
});
pub unsafe fn get_1() -> *mut Vec<u8> {
    return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut g_0));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (&mut (*(unsafe { get_1() })))[(1_usize)] = 4_u8;
    let mut v: Option<Box<Vec<u8>>> = Some(Box::from_raw(
        (Box::leak(Box::new(
            (0..(3_usize) as usize)
                .map(|_| <u8>::default())
                .collect::<Vec<_>>(),
        )) as *mut Vec<u8>),
    ));
    (&mut (*v.as_deref().map_or(::std::ptr::null_mut(), |v| {
        v as *const Vec<u8> as *mut Vec<u8>
    })))[(2_usize)] = 5_u8;
    assert!((((&mut (*(unsafe { get_1() })))[(1_usize)] as i32) == (4)));
    assert!(
        (((&mut (*v
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const Vec<u8>
                as *mut Vec<u8>)))[(2_usize)] as i32)
            == (5))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const g_0);
}
