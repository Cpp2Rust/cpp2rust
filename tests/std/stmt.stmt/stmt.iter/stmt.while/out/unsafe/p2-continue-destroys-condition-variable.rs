extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut ctorCount_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut dtorCount_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CppFromUnsafe, DestructorUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct ConvertsToBool {
    pub value: i32,
}
impl ConvertsToBool {
    pub unsafe fn new(mut value: i32) -> Self {
        let mut this = Self { value: value };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_1)).prefix_inc();
    }
    pub unsafe fn to_bool(&self) -> bool {
        return ((self.value) != (0));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut remaining: i32 = 3;
    let mut continued: i32 = 0;
    let mut completed: i32 = 0;
    'loop_: while (unsafe { ConvertsToBool::to_bool(&condition) }) {
        remaining.prefix_dec();
        if ((condition.value) == (3)) {
            continued.prefix_inc();
            continue 'loop_;
        }
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) == (2)));
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_1)) == (1)));
        completed.prefix_inc();
        break;
    }
    assert!(((continued) == (1)));
    assert!(((completed) == (1)));
    assert!(((remaining) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) == (2)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_1)) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const ctorCount_0);
    std::cell::LazyCell::force(&*&raw const dtorCount_1);
}
