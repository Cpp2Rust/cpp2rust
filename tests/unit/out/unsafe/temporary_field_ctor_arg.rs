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
pub struct Outer {
    pub values: Vec<i32>,
}
impl Outer {
    pub unsafe fn move_from(_a0: *mut Outer) -> Self {
        let mut this = Self {
            values: std::mem::take(&mut (*_a0).values),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Outer) -> *mut Outer {
        self.values = std::mem::take(&mut (*_a0).values);
        return &mut (*(self as *mut Outer));
    }
}
pub unsafe fn make_0() -> Outer {
    return Outer {
        values: Vec::from([1, 2, 3, 4]),
    };
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(Vec<i32 > , new, ptr)]
pub struct H {
    pub last: i32,
}
impl H {
    pub unsafe fn new(values: *const Vec<i32>) -> Self {
        let mut this = Self {
            last: (&(*values))[(3_usize)],
        };
        this
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut h: H = H::new({ &(unsafe { make_0() }).values });
    (unsafe { libc::printf(c"%d\n".as_ptr() as *const libc::c_char, (h.last)) });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
