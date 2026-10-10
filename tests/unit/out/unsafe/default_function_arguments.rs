extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(mut a: i32, mut b: Option<i32>) -> i32 {
    let mut b: i32 = b.unwrap_or_else(|| unsafe { 10 });
    return ((a) + (b));
}
pub unsafe fn baz_1(mut a: *mut i32, mut b: Option<*mut i32>) -> bool {
    let mut b: *mut i32 = b.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    return ((a) == (b));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Bar {
    pub v: i32,
}
impl Bar {
    pub unsafe fn new(mut v: Option<i32>) -> Self {
        let mut v: i32 = v.unwrap_or_else(|| unsafe { 1 });
        let mut this = Self { v: v };
        this
    }
}
impl Default for Bar {
    fn default() -> Self {
        unsafe { Bar::new(None) }
    }
}
pub static mut counter_2: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub unsafe fn next_3() -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut counter_2)).prefix_inc();
}
pub unsafe fn lazy_4(mut x: Option<i32>) -> i32 {
    let mut x: i32 = x.unwrap_or_else(|| unsafe { (unsafe { next_3() }) });
    return x;
}
pub unsafe fn by_ref_5(b: Option<*const Bar>) -> i32 {
    let mut __b_default: Option<Bar> = None;
    let mut b: *const Bar =
        b.unwrap_or_else(|| unsafe { __b_default.insert(Bar::new({ Some(7) })) as *const Bar });
    return (*b).v;
}
pub static mut global_bar_6: std::cell::LazyCell<Bar> =
    std::cell::LazyCell::new(|| unsafe { Bar::new({ Some(9) }) });
pub unsafe fn by_global_ref_7(b: Option<*const Bar>) -> i32 {
    let mut b: *const Bar = b.unwrap_or_else(|| unsafe {
        &(*std::cell::LazyCell::force_mut(&mut *&raw mut global_bar_6))
    });
    return (*b).v;
}
pub unsafe fn shifted_8(mut x: i32, mut y: Option<i32>) -> i32 {
    let mut y: i32 = y.unwrap_or_else(|| unsafe { -1_i32 });
    return ((x) + (y));
}
pub unsafe fn first_9(r: *mut [i32; 3]) -> i32 {
    return (*r)[(0) as usize];
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Holder_int_ {
    pub v: i32,
}
impl Holder_int_ {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { foo_0(1, None,) }) == (11)));
    assert!(((unsafe { foo_0(1, Some(2),) }) == (3)));
    let mut a: i32 = 0;
    assert!((((unsafe { baz_1((&mut a as *mut i32), None,) }) as i32) == (false as i32)));
    assert!(
        (((unsafe {
            let _a: *mut i32 = (&mut a as *mut i32);
            let _b: *mut i32 = (&mut a as *mut i32);
            baz_1(_a, Some(_b))
        }) as i32)
            == (true as i32))
    );
    let mut b: Bar = Bar::new(None);
    assert!(((b.v) == (1)));
    assert!(((Bar::new({ Some(2) },).v) == (2)));
    let mut arr: [Bar; 3] = [Bar::new(None), Bar::new(None), Bar::new(None)];
    assert!(((arr[(0) as usize].v) == (1)));
    assert!(((arr[(2) as usize].v) == (1)));
    assert!(((unsafe { lazy_4(Some(5),) }) == (5)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut counter_2)) == (0)));
    assert!(((unsafe { lazy_4(None,) }) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut counter_2)) == (1)));
    assert!(((unsafe { by_ref_5(None,) }) == (7)));
    assert!(
        ((unsafe {
            let mut _b: Bar = Bar::new({ Some(3) });
            by_ref_5(Some(&mut _b))
        }) == (3))
    );
    assert!(((unsafe { by_global_ref_7(None,) }) == (9)));
    assert!(
        ((unsafe {
            let mut _b: Bar = Bar::new({ Some(4) });
            by_global_ref_7(Some(&mut _b))
        }) == (4))
    );
    let mut h: Holder_int_ = Holder_int_::new({ 4 });
    assert!(((h.v) == (4)));
    assert!(((unsafe { shifted_8(3, None,) }) == (2)));
    assert!(((unsafe { shifted_8(3, Some(4),) }) == (7)));
    assert!(
        ((unsafe {
            let mut _r: [i32; 3] = [4, 5, 6];
            first_9(&mut _r)
        }) == (4))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const counter_2);
    std::cell::LazyCell::force(&*&raw const global_bar_6);
}
