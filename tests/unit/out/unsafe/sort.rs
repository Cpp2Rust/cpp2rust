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
        let __a1 = 10;
        v.push(__a1)
    };
    {
        let __a1 = 1;
        v.push(__a1)
    };
    {
        let __a1 = 9;
        v.push(__a1)
    };
    {
        let __a1 = 2;
        v.push(__a1)
    };
    {
        let __a1 = 8;
        v.push(__a1)
    };
    {
        let __a1 = 3;
        v.push(__a1)
    };
    {
        let __a1 = 7;
        v.push(__a1)
    };
    {
        let __a1 = 4;
        v.push(__a1)
    };
    {
        let __a1 = 5;
        v.push(__a1)
    };
    {
        let __a1 = 6;
        v.push(__a1)
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
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < ((v.len()).wrapping_sub(1_usize))) {
        assert!(((v[(i as usize)]) < (v[(((i).wrapping_add(1_u32)) as usize)])));
        i.prefix_inc();
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
