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
pub struct Elem {
    pub v: i32,
}
impl Elem {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn copy_from(other: *const Elem) -> Self {
        let mut this = Self {
            v: (((*other).v) + (100)),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        this
    }
    pub unsafe fn move_from(other: *mut Elem) -> Self {
        let mut this = Self {
            v: (((*other).v) + (1000)),
        };
        (*other).v = -1_i32;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)).prefix_inc();
        this
    }
}
impl Clone for Elem {
    fn clone(&self) -> Self {
        unsafe { Elem::copy_from(self as *const Elem) }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, Default)]
pub struct Two {
    pub a: Elem,
    pub b: Elem,
}
pub unsafe fn make_two_2() -> Two {
    return Two {
        a: Elem::new({ 1 }),
        b: Elem::new({ 2 }),
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut t: Two = Two {
        a: Elem::new({ 1 }),
        b: Elem::new({ 2 }),
    };
    let mut base_copies: i32 = (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0));
    let mut base_moves: i32 = (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1));
    let mut __decomp_3: Two = t.clone();
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == ((base_copies) + (2))));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)) == (base_moves)));
    assert!(((__decomp_3.a.v) == (101)));
    assert!(((__decomp_3.b.v) == (102)));
    let __decomp_4: *mut Two = &mut t;
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == ((base_copies) + (2))));
    assert!((((*__decomp_4).a.v) == (1)));
    (*__decomp_4).a.v = 50;
    assert!(((t.a.v) == (50)));
    let mut __decomp_5: Two = t.clone();
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)) == ((base_moves) + (2))));
    assert!(((__decomp_5.a.v) == (1050)));
    assert!(((__decomp_5.b.v) == (1002)));
    assert!(((t.a.v) == (-1_i32)));
    let mut before_copies: i32 = (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0));
    let mut before_moves: i32 = (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1));
    let mut __decomp_6: Two = (unsafe { make_two_2() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (before_copies)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)) == (before_moves)));
    assert!(((__decomp_6.a.v) == (1)));
    let mut p: (Elem, Elem) = (Elem::new({ 7 }).into(), Elem::new({ 8 }).into());
    before_copies = (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0));
    let mut __decomp_7: (Elem, Elem) = p.clone();
    let x: *mut Elem = (unsafe { get_8(&mut __decomp_7) });
    let y: *mut Elem = (unsafe { get_9(&mut __decomp_7) });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == ((before_copies) + (2)))
    );
    assert!((((*x).v) == ((p.0.v) + (100))));
    assert!((((*y).v) == ((p.1.v) + (100))));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copies_0);
    std::cell::LazyCell::force(&*&raw const moves_1);
}
