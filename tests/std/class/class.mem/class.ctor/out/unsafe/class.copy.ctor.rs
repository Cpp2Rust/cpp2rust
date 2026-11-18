extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct p11_defaulted_move_constructor_is_trivial_MoveOnly {
    pub value: i32,
}
impl p11_defaulted_move_constructor_is_trivial_MoveOnly {
    pub unsafe fn new(mut value: i32) -> Self {
        let mut this = Self { value: value };
        this
    }
    pub unsafe fn move_from(_a0: *mut p11_defaulted_move_constructor_is_trivial_MoveOnly) -> Self {
        let mut this = Self {
            value: (*_a0).value,
        };
        this
    }
}
pub unsafe fn pass_0(
    mut object: p11_defaulted_move_constructor_is_trivial_MoveOnly,
) -> p11_defaulted_move_constructor_is_trivial_MoveOnly {
    return p11_defaulted_move_constructor_is_trivial_MoveOnly::move_from({ &mut object });
}
pub unsafe fn test_1() {
    let mut result: p11_defaulted_move_constructor_is_trivial_MoveOnly = (unsafe {
        pass_0(p11_defaulted_move_constructor_is_trivial_MoveOnly::new({
            7
        }))
    });
    assert!(((result.value) == (7)));
}
pub unsafe fn test_2() {
    let mut source: Vec<i32> = Vec::from([1, 2, 3]);
    let mut moved: Vec<i32> = source.clone();
    assert!(((moved.len()) == (3_usize)));
    assert!(((source.len()) == (3_usize)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_1() });
    (unsafe { test_2() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
