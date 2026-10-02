extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copies_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, Default)]
pub struct Counted {
    pub x: i32,
    pub y: i32,
}
impl Counted {
    pub unsafe fn new(mut x: i32, mut y: i32) -> Self {
        let mut this = Self { x: x, y: y };
        this
    }
    pub unsafe fn copy_from(other: *const Counted) -> Self {
        let mut this = Self {
            x: (((*other).x) * (2)),
            y: (((*other).y) * (2)),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        this
    }
}
impl Clone for Counted {
    fn clone(&self) -> Self {
        unsafe { Counted::copy_from(self as *const Counted) }
    }
}
pub unsafe fn make_counted_1() -> Counted {
    return Counted::new({ 5 }, { 6 });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: Counted = Counted::new({ 1 }, { 2 });
    let mut __decomp_2: Counted = Counted::copy_from({ &s });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (1)));
    assert!(((__decomp_2.x) == (2)));
    assert!(((__decomp_2.y) == (4)));
    let __decomp_3: *mut Counted = &mut s;
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (1)));
    assert!((((*__decomp_3).x) == (1)));
    assert!((((*__decomp_3).y) == (2)));
    let __decomp_4: *const Counted = &s;
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (1)));
    assert!((((*__decomp_4).x) == (1)));
    let mut __decomp_5: Counted = (unsafe { make_counted_1() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (1)));
    assert!(((__decomp_5.x) == (5)));
    assert!(((__decomp_5.y) == (6)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copies_0);
}
