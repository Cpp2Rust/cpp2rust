extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Inner {
    pub values: Vec<i32>,
}
impl Inner {
    pub unsafe fn move_from(_a0: *mut Inner) -> Self {
        let mut this = Self {
            values: std::mem::take(&mut (*_a0).values),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Inner) -> *mut Inner {
        self.values = std::mem::take(&mut (*_a0).values);
        return &mut (*(self as *mut Inner));
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Outer {
    pub values: Vec<i32>,
    pub inner: Inner,
}
impl Outer {
    pub unsafe fn move_from(_a0: *mut Outer) -> Self {
        let mut this = Self {
            values: std::mem::take(&mut (*_a0).values),
            inner: Inner::move_from({ &mut (*_a0).inner }),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Outer) -> *mut Outer {
        self.values = std::mem::take(&mut (*_a0).values);
        (unsafe {
            let _arg0: *mut Inner = &mut (*_a0).inner;
            Inner::move_assign(&mut self.inner, _arg0)
        });
        return &mut (*(self as *mut Outer));
    }
}
pub unsafe fn make_0() -> Outer {
    return Outer {
        values: Vec::from([1, 2, 3]),
        inner: Inner {
            values: Vec::from([4, 5, 6]),
        },
    };
}
pub unsafe fn two_1(a: *const Vec<i32>, b: *const Vec<i32>) -> i32 {
    return (((&(*a))[(0_usize)]) + ((&(*b))[(2_usize)]));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe {
        libc::printf(
            c"%d\n".as_ptr() as *const libc::c_char,
            (unsafe {
                let _a: *const Vec<i32> = &(unsafe { make_0() }).values;
                let _b: *const Vec<i32> = &(unsafe { make_0() }).inner.values;
                two_1(_a, _b)
            }),
        )
    });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
