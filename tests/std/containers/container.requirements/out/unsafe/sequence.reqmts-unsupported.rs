extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() {
    let mut source: (Option<Box<i32>>, i32) =
        (Some(Box::from_raw((Box::leak(Box::new(1)) as *mut i32))), 2);
    let mut pairs: Vec<(Option<Box<i32>>, i32)> = Vec::new();
    match ({ <UnsafeRandomAccessIterator<(Option<Box<i32>>, i32)>>::from(pairs.end()) },) {
        (mut __tmp_0,) => {
            let pos = __tmp_0.iter_distance(&UnsafeRandomAccessIterator::Vec(pairs.as_mut_ptr()))
                as usize;
            pairs.insert(pos, cpp_move_unsafe!(&mut source));
            (__tmp_0).clone()
        }
    };
    assert!(
        ((*pairs[(0_usize)]
            .0
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    assert!(source.0.is_none());
    assert!(((source.1) == (2)));
}
pub unsafe fn test_1() {
    let mut source: (Option<Box<i32>>, i32) =
        (Some(Box::from_raw((Box::leak(Box::new(1)) as *mut i32))), 2);
    let mut pairs: Vec<(Option<Box<i32>>, i32)> = Vec::new();
    {
        let __a1 = std::mem::take(&mut source);
        pairs.push(__a1)
    };
    assert!(
        ((*pairs[(0_usize)]
            .0
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    assert!(source.0.is_none());
    assert!(((source.1) == (2)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_0() });
    (unsafe { test_1() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
