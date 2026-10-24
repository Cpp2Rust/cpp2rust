extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Item {
    pub key: i32,
    pub value: i32,
}
pub unsafe fn Compare_0(a: *const Item, b: *const Item) -> bool {
    return (((*a).key) < ((*b).key));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: Vec<Item> = Vec::new();
    {
        let __a1 = Item { key: 3, value: 30 };
        v.push(__a1)
    };
    {
        let __a1 = Item { key: 1, value: 10 };
        v.push(__a1)
    };
    {
        let __a1 = Item { key: 2, value: 20 };
        v.push(__a1)
    };
    {
        let __first = v.begin();
        let __n = v.end().iter_distance(&__first);
        let __cmp = (Compare_0 as unsafe fn(*const Item, *const Item) -> bool);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            let __a = __first.iter_at(__x) as *const _;
            let __b = __first.iter_at(__y) as *const _;
            if __cmp.call(__a, __b) {
                std::cmp::Ordering::Less
            } else if __cmp.call(__b, __a) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(__first.iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(__first.iter_at(__i), __v);
        }
    };
    assert!(((v[(0_usize)].key) == (1)));
    assert!(((v[(1_usize)].key) == (2)));
    assert!(((v[(2_usize)].key) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
