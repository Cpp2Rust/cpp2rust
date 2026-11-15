extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copyCount_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct HasConstexprCtor {
    pub m: i32,
}
impl HasConstexprCtor {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { m: v };
        this
    }
    pub unsafe fn copy_from(o: *const HasConstexprCtor) -> Self {
        let mut this = Self { m: (*o).m };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_0)).prefix_inc();
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const HasConstexprCtor) -> *mut HasConstexprCtor {
        self.m = (*_a0).m;
        return &mut (*(self as *mut HasConstexprCtor));
    }
}
impl Clone for HasConstexprCtor {
    fn clone(&self) -> Self {
        unsafe { HasConstexprCtor::copy_from(self as *const HasConstexprCtor) }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let arr: [HasConstexprCtor; 2] = [HasConstexprCtor::new({ 3 }), HasConstexprCtor::new({ 4 })];
    let mut f: FnPtr<fn() -> i32> = FnPtr::<fn() -> i32>::new(|| -> i32 {
        unsafe {
            return [HasConstexprCtor::new({ 3 }), HasConstexprCtor::new({ 4 })][(1) as usize].m;
        }
    });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_0)) == (0)));
    assert!(((unsafe { f.call() }) == (4)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_0)) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copyCount_0);
}
