extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut calls_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut xs_1: std::cell::LazyCell<[i32; 2]> = std::cell::LazyCell::new(|| unsafe { [1, 2] });
pub unsafe fn pick_2(mut i: i32) -> *mut i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_0)).prefix_inc();
    return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut xs_1))[(i) as usize];
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct Bag {
    pub n: i32,
}
impl Bag {
    pub unsafe fn new(mut n: i32) -> Self {
        let mut this = Self { n: n };
        this
    }
    pub unsafe fn copy_from(o: *const Bag) -> Self {
        let mut this = Self { n: (*o).n };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_0)).prefix_inc();
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Bag) -> *mut Bag {
        self.n = (*_a0).n;
        return &mut (*(self as *mut Bag));
    }
}
impl std::cmp::Ord for Bag {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_3(self as *const Bag, other as *const Bag) {
                std::cmp::Ordering::Less
            } else if operator_lt_3(other as *const Bag, self as *const Bag) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Bag {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Bag {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_lt_3(self as *const Bag, other as *const Bag))
                && !(operator_lt_3(other as *const Bag, self as *const Bag))
        }
    }
}
impl std::cmp::Eq for Bag {}
impl Clone for Bag {
    fn clone(&self) -> Self {
        unsafe { Bag::copy_from(self as *const Bag) }
    }
}
pub unsafe fn operator_lt_3(a: *const Bag, b: *const Bag) -> bool {
    return (((*a).n) < ((*b).n));
}
pub unsafe fn make_4(mut n: i32) -> Bag {
    return Bag::new({ n });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x1: i32 = 1;
    let mut x2: i32 = 2;
    let mut x3: i32 = 10;
    let mut x4: i32 = 20;
    let mut p1: *mut i32 = (&mut x1 as *mut i32);
    let mut p2: *mut i32 = (&mut x2 as *mut i32);
    let mut r1: i32 = (*{
        let __a: *const _ = &mut x1;
        let __b: *const _ = &mut x2;
        if *__a < *__b { __b } else { __a }
    });
    let mut r2: i32 = (*{
        let __a: *const _ = &mut x3;
        let __b: *const _ = &mut x4;
        if *__b < *__a { __b } else { __a }
    });
    let mut r3: i32 = (*{
        let __a: *const _ = p1;
        let __b: *const _ = &mut x2;
        if *__a < *__b { __b } else { __a }
    });
    let mut r4: i32 = (*{
        let __a: *const _ = p2;
        let __b: *const _ = &mut x3;
        if *__b < *__a { __b } else { __a }
    });
    let mut r5: i32 = {
        let mut __tmp_0: i32 = 30;
        let mut __tmp_1: i32 = 40;
        (*{
            let __a: *const _ = &mut __tmp_0;
            let __b: *const _ = &mut __tmp_1;
            if *__a < *__b { __b } else { __a }
        })
    };
    assert!(((((((r1) + (r2)) + (r3)) + (r4)) + (r5)) == (56)));
    let mut values: [i32; 4] = [1, 7, 7, 3];
    let mut max: *mut i32 = {
        let __first = values.as_mut_ptr();
        let __last = values.as_mut_ptr().offset((4) as isize);
        let mut __best = __first.clone();
        if __first != __last {
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__best.iter_deref() < *__it.iter_deref() {
                    __best = __it.clone();
                }
                __it.iter_inc();
            }
        }
        __best
    };
    assert!(((max) == (values.as_mut_ptr().offset((1) as isize))));
    assert!(
        ((*{
            let __a: *const _ = (unsafe { pick_2(0) });
            let __b: *const _ = (unsafe { pick_2(1) });
            if *__a < *__b { __b } else { __a }
        }) == (2))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut calls_0)) == (2))
    );
    assert!(
        ((*{
            let __a: *const _ = (unsafe { pick_2(1) });
            let __b: *const _ = (unsafe { pick_2(0) });
            if *__b < *__a { __b } else { __a }
        }) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut calls_0)) == (4))
    );
    (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_0)) = 0;
    assert!(
        (({
            let mut __tmp_2: Bag = (unsafe { make_4(1) });
            let mut __tmp_3: Bag = (unsafe { make_4(2) });
            (*{
                let __a: *const _ = &mut __tmp_2;
                let __b: *const _ = &mut __tmp_3;
                if *__a < *__b { __b } else { __a }
            })
            .n
        }) == (2))
            && (({
                let mut __tmp_4: Bag = (unsafe { make_4(1) });
                let mut __tmp_5: Bag = (unsafe { make_4(2) });
                (*{
                    let __a: *const _ = &mut __tmp_4;
                    let __b: *const _ = &mut __tmp_5;
                    if *__b < *__a { __b } else { __a }
                })
                .n
            }) == (1))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut calls_0)) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const calls_0);
    std::cell::LazyCell::force(&*&raw const xs_1);
}
