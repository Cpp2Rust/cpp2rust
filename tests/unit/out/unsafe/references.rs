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
pub struct Pair {
    pub x: i32,
    pub y: i32,
    pub arr: [i32; 2],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Nested {
    pub p: Pair,
}
pub static mut calls_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub unsafe fn make_pair_1() -> Pair {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_0)).prefix_inc();
    return Pair {
        x: 1,
        y: 2,
        arr: [3, 4],
    };
}
pub unsafe fn make_nested_2() -> Nested {
    return Nested {
        p: (unsafe { make_pair_1() }),
    };
}
pub unsafe fn note_3(mut v: i32) {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut calls_0)) += v;
}
pub unsafe fn extended_temporaries_4(mut c: bool, mut v: i32) {
    let mut __tmp_0: Pair;
    let field: *const i32 = &(*{
        __tmp_0 = (unsafe { make_pair_1() });
        &mut __tmp_0
    })
    .y;
    let mut __tmp_1: Nested;
    let nested: *const i32 = &(*{
        __tmp_1 = (unsafe { make_nested_2() });
        &mut __tmp_1
    })
    .p
    .x;
    let mut __tmp_2: Pair;
    let element: *const i32 = &(*{
        __tmp_2 = (unsafe { make_pair_1() });
        &mut __tmp_2
    })
    .arr[(1) as usize];
    let mut __tmp_3: i32 = if c { (unsafe { make_pair_1() }).x } else { v };
    let chosen: *const i32 = &mut __tmp_3;
    let mut __tmp_4: Pair;
    let comma: *const i32 = &{
        (unsafe { note_3(10) });
        (*{
            __tmp_4 = (unsafe { make_pair_1() });
            &mut __tmp_4
        })
        .x
    };
    let mut __tmp_5: Pair;
    let rvalue: *const i32 = &(*{
        __tmp_5 = (unsafe { make_pair_1() });
        &mut __tmp_5
    })
    .y;
    assert!(((*field) == (2)));
    assert!(((*nested) == (1)));
    assert!(((*element) == (4)));
    assert!(((*chosen) == (1)));
    assert!(((*comma) == (1)));
    assert!(((*rvalue) == (2)));
    assert!(((*(*std::cell::LazyCell::force_mut(&mut *&raw mut once_5))) == (3)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut calls_0)) == (18)));
}
static mut __tmp_6: std::cell::LazyCell<Pair> =
    std::cell::LazyCell::new(|| unsafe { (unsafe { make_pair_1() }) });
static mut once_5: std::cell::LazyCell<*const i32> = std::cell::LazyCell::new(|| unsafe {
    &(*{ std::cell::LazyCell::force_mut(&mut *&raw mut __tmp_6) }).arr[(0) as usize]
});
static mut __tmp_7: std::cell::LazyCell<Pair> =
    std::cell::LazyCell::new(|| unsafe { (unsafe { make_pair_1() }) });
pub static mut global_field_6: std::cell::LazyCell<*const i32> =
    std::cell::LazyCell::new(|| unsafe {
        &(*{ std::cell::LazyCell::force_mut(&mut *&raw mut __tmp_7) }).x
    });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: i32 = 1;
    let r: *mut i32 = &mut x;
    (*r) = 5;
    assert!(((x) == (5)));
    (unsafe { extended_temporaries_4(true, 7) });
    assert!(((*(*std::cell::LazyCell::force_mut(&mut *&raw mut global_field_6))) == (1)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const calls_0);
    std::cell::LazyCell::force(&*&raw const global_field_6);
}
