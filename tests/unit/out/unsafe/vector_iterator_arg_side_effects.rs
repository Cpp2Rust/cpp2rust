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
    let mut v: Vec<i32> = Vec::from([1, 2, 3, 4]);
    let mut it: UnsafeRandomAccessIterator<i32> = v.begin();
    it.iter_inc();
    let mut r: UnsafeRandomAccessIterator<i32> = {
        let __it = <UnsafeRandomAccessIterator<i32>>::from(it.iter_post_inc());
        let pos = __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr())) as usize;
        v.remove(pos);
        __it
    };
    assert!(((*r.iter_deref()) == (3)));
    assert!(((v.len()) == (3_usize)));
    let mut it2: UnsafeRandomAccessIterator<i32> = v.begin();
    r = {
        let __it = <UnsafeRandomAccessIterator<i32>>::from(it2.iter_post_inc());
        let pos = __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr())) as usize;
        v.insert(pos, 9);
        __it
    };
    assert!((((*r.iter_deref()) == (9)) && ((v[(0_usize)]) == (9))) && ((v.len()) == (4_usize)));
    let c: *const Vec<i32> = &v;
    let mut i: i32 = 1;
    assert!(
        ((*{
            let __idx = (i.postfix_inc() as usize);
            if __idx as usize >= (*c).len() {
                panic!("out of bounds access")
            } else {
                (*c).as_ptr().add(__idx as usize)
            }
        }) == (1))
            && ((i) == (2))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
