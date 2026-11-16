extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasTwoInts {
    pub first: i32,
    pub second: i32,
}
pub unsafe fn second_or_null_0(mut choose_second: bool) -> MemberPtrUnsafe<HasTwoInts, i32> {
    return if choose_second {
        <MemberPtrUnsafe<HasTwoInts, i32>>::from_offset(::std::mem::offset_of!(HasTwoInts, second))
    } else {
        <MemberPtrUnsafe<HasTwoInts, i32>>::default()
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut obj: HasTwoInts = HasTwoInts {
        first: 3,
        second: 4,
    };
    assert!((unsafe { second_or_null_0(false,) }).is_null());
    assert!(
        ((unsafe { second_or_null_0(true,) })
            == (<MemberPtrUnsafe<HasTwoInts, i32>>::from_offset(::std::mem::offset_of!(
                HasTwoInts, second
            ))))
    );
    assert!(((*(&mut obj as *mut HasTwoInts).member((unsafe { second_or_null_0(true,) }))) == (4)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
