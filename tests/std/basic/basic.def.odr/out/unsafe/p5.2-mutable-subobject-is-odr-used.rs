extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copies_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct HasMutableMember {
    pub value: i32,
    pub scratch: i32,
}
impl HasMutableMember {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self {
            value: v,
            scratch: 0,
        };
        this
    }
    pub unsafe fn copy_from(o: *const HasMutableMember) -> Self {
        let mut this = Self {
            value: (*o).value,
            scratch: (*o).scratch,
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const HasMutableMember) -> *mut HasMutableMember {
        self.value = (*_a0).value;
        self.scratch = (*_a0).scratch;
        return &mut (*(self as *mut HasMutableMember));
    }
}
impl Clone for HasMutableMember {
    fn clone(&self) -> Self {
        unsafe { HasMutableMember::copy_from(self as *const HasMutableMember) }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let k: HasMutableMember = HasMutableMember::new({ 5 });
    let mut f: FnPtr<fn() -> i32> = lambda_unsafe!(
        {
            let k: HasMutableMember = HasMutableMember::copy_from({ &k });
        },
        || -> i32 {
            return k.value;
        }
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (1)));
    assert!(((unsafe { f.call() }) == (5)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (1)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copies_0);
}
