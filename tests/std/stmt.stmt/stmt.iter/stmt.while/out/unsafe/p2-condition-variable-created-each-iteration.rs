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
    let mut remaining: i32 = 2;
    let mut iterations: i32 = 0;
    'loop_: loop {
        let mut condition: ConvertsToBool = ConvertsToBool::new({ remaining });
        let _dtor_condition =
            std::mem::DropGuard::new(&raw mut condition, |__p| unsafe { (*__p).destructor() });
        if !(unsafe { ConvertsToBool::to_bool(&condition) }) {
            break;
        }
        iterations.prefix_inc();
        assert!(((condition.value) == (remaining)));
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) == (iterations)));
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_1)) == ((iterations) - (1)))
        );
        remaining.prefix_dec();
    }
    assert!(((iterations) == (2)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) == (3)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_1)) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const ctorCount_0);
    std::cell::LazyCell::force(&*&raw const dtorCount_1);
}
