extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut destroyed_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut last_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct S {
    pub v: i32,
}
impl S {
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)).postfix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut last_1)) = self.v;
    }
}
pub unsafe fn by_value_2(mut s: S) -> i32 {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) == (0)));
    return s.v;
}
pub unsafe fn with_default_3(mut s: Option<S>) -> i32 {
    let mut s: S = s.unwrap_or_else(|| unsafe { S { v: 5 } });
    return s.v;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Holder {
    pub v: i32,
}
impl Holder {
    pub unsafe fn new(mut s: S) -> Self {
        let mut this = Self { v: s.v };
        this
    }
    pub unsafe fn add(&self, mut s: S) -> i32 {
        return ((self.v) + (s.v));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        let mut s: S = S { v: 1 };
        let _dtor_s = ScopedDestructorUnsafe::new(&raw mut s, S::destructor);
        assert!(((unsafe { by_value_2(s.clone(),) }) == (1)));
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) == (1))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut last_1)) == (1))
        );
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) == (2)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) = 0;
    assert!(((unsafe { by_value_2(S { v: 2 },) }) == (2)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut last_1)) == (2))
    );
    (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) = 0;
    assert!(((unsafe { with_default_3(None,) }) == (5)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut last_1)) == (5))
    );
    (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) = 0;
    {
        let mut h: Holder = Holder::new({ S { v: 3 } });
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) == (1))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut last_1)) == (3))
        );
        assert!(((unsafe { Holder::add(&h, S { v: 4 },) }) == (7)));
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) == (2))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut last_1)) == (4))
        );
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destroyed_0);
    std::cell::LazyCell::force(&*&raw const last_1);
}
