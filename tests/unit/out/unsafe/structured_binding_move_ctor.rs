extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copies_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut moves_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, Default)]
pub struct Movable {
    pub x: i32,
    pub y: i32,
}
impl Movable {
    pub unsafe fn new(mut x: i32, mut y: i32) -> Self {
        let mut this = Self { x: x, y: y };
        this
    }
    pub unsafe fn copy_from(other: *const Movable) -> Self {
        let mut this = Self {
            x: (*other).x,
            y: (*other).y,
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        this
    }
    pub unsafe fn move_from(other: *mut Movable) -> Self {
        let mut this = Self {
            x: (*other).x,
            y: (*other).y,
        };
        (*other).x = 0;
        (*other).y = 0;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)).prefix_inc();
        this
    }
}
impl Clone for Movable {
    fn clone(&self) -> Self {
        unsafe { Movable::copy_from(self as *const Movable) }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, Default)]
pub struct Holder {
    pub xs: Vec<i32>,
    pub ys: Vec<i32>,
}
impl Holder {
    pub unsafe fn extract(&mut self) -> Holder {
        return Holder {
            xs: std::mem::take(&mut self.xs),
            ys: std::mem::take(&mut self.ys),
        };
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut m: Movable = Movable::new({ 3 }, { 4 });
    let mut __decomp_2: Movable = Movable::move_from({ &mut m });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    assert!(((__decomp_2.x) == (3)));
    assert!(((__decomp_2.y) == (4)));
    assert!(((m.x) == (0)));
    assert!(((m.y) == (0)));
    let mut h: Holder = Holder {
        xs: vec![1, 2, 3],
        ys: vec![4, 5],
    };
    let mut __decomp_3: Holder = (unsafe { Holder::extract(&mut h) });
    assert!(((__decomp_3.xs.len()) == (3_usize)));
    assert!(((__decomp_3.ys.len()) == (2_usize)));
    assert!(((__decomp_3.xs[(2_usize)]) == (3)));
    assert!(((__decomp_3.ys[(0_usize)]) == (4)));
    assert!(h.xs.is_empty());
    assert!(h.ys.is_empty());
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copies_0);
    std::cell::LazyCell::force(&*&raw const moves_1);
}
