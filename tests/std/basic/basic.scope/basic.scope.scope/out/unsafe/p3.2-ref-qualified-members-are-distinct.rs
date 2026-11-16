extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut lvalue_calls_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut rvalue_calls_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasRefQualifiedOverloads {}
impl HasRefQualifiedOverloads {
    pub unsafe fn get_1(&mut self) -> i32 {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut lvalue_calls_0)).prefix_inc();
        return 3;
    }
    pub unsafe fn get_2(&mut self) -> i32 {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut rvalue_calls_1)).prefix_inc();
        return 4;
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut object: HasRefQualifiedOverloads = <HasRefQualifiedOverloads>::default();
    assert!(((unsafe { HasRefQualifiedOverloads::get_1(&mut object,) }) == (3)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut lvalue_calls_0)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut rvalue_calls_1)) == (0)));
    assert!(
        ((unsafe { HasRefQualifiedOverloads::get_2(&mut <HasRefQualifiedOverloads>::default(),) })
            == (4))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut lvalue_calls_0)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut rvalue_calls_1)) == (1)));
    assert!(((unsafe { HasRefQualifiedOverloads::get_2(&mut object,) }) == (4)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut lvalue_calls_0)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut rvalue_calls_1)) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const lvalue_calls_0);
    std::cell::LazyCell::force(&*&raw const rvalue_calls_1);
}
