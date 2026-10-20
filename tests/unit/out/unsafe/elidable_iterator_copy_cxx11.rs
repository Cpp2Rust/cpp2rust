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
    let mut v: Vec<i32> = Vec::new();
    {
        let __a1 = 4;
        v.push(__a1)
    };
    {
        let __a1 = 7;
        v.push(__a1)
    };
    let mut it: UnsafeRandomAccessIterator<i32> = {
        let mut __it = v.begin();
        while __it != v.end() && *__it.iter_deref() != 7 {
            __it.iter_inc();
        }
        __it
    };
    assert!(it != v.end());
    assert!(((*it.iter_deref()) == (7)));
    {
        let __n = v.end().iter_distance(&v.begin());
        for __i in (1..__n).rev() {
            let __j = libc::rand() as isize % (__i + 1);
            cpp_swap_unsafe!(v.begin().iter_at(__i), v.begin().iter_at(__j));
        }
    };
    {
        let __n = v.end().iter_distance(&v.begin());
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| (*v.begin().iter_at(__x)).cmp(&*v.begin().iter_at(__y)));
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(v.begin().iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(v.begin().iter_at(__i), __v);
        }
    };
    assert!(((v[(0_usize)]) == (4)) && ((v[(1_usize)]) == (7)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
