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
pub struct HasRefQualifiedTemplateOverloads {}
impl HasRefQualifiedTemplateOverloads {
    pub unsafe fn get_1_int(&mut self, mut value: i32) -> i32 {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut lvalue_calls_0)).prefix_inc();
        return ((value) + (3));
    }
    pub unsafe fn get_2_int(&mut self, mut value: i32) -> i32 {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut rvalue_calls_1)).prefix_inc();
        return ((value) + (4));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut object: HasRefQualifiedTemplateOverloads =
        <HasRefQualifiedTemplateOverloads>::default();
    assert!(((unsafe { HasRefQualifiedTemplateOverloads::get_1_int(&mut object, 10,) }) == (13)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut lvalue_calls_0)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut rvalue_calls_1)) == (0)));
    assert!(
        ((unsafe {
            HasRefQualifiedTemplateOverloads::get_2_int(
                &mut <HasRefQualifiedTemplateOverloads>::default(),
                10,
            )
        }) == (14))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut lvalue_calls_0)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut rvalue_calls_1)) == (1)));
    assert!(((unsafe { HasRefQualifiedTemplateOverloads::get_2_int(&mut object, 10,) }) == (14)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut lvalue_calls_0)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut rvalue_calls_1)) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const lvalue_calls_0);
    std::cell::LazyCell::force(&*&raw const rvalue_calls_1);
}
