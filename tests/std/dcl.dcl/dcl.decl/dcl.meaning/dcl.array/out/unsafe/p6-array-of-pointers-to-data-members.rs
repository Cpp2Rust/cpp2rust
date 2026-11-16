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
pub struct HasThreeInts {
    pub first: i32,
    pub second: i32,
    pub third: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut members: [MemberPtrUnsafe<HasThreeInts, i32>; 3] = [
        <MemberPtrUnsafe<HasThreeInts, i32>>::from_offset(::std::mem::offset_of!(
            HasThreeInts,
            third
        )),
        <MemberPtrUnsafe<HasThreeInts, i32>>::from_offset(::std::mem::offset_of!(
            HasThreeInts,
            first
        )),
        <MemberPtrUnsafe<HasThreeInts, i32>>::from_offset(::std::mem::offset_of!(
            HasThreeInts,
            second
        )),
    ];
    let mut obj: HasThreeInts = HasThreeInts {
        first: 4,
        second: 5,
        third: 6,
    };
    assert!(((*(&mut obj as *mut HasThreeInts).member(members[(0) as usize])) == (6)));
    assert!(((*(&mut obj as *mut HasThreeInts).member(members[(1) as usize])) == (4)));
    assert!(((*(&mut obj as *mut HasThreeInts).member(members[(2) as usize])) == (5)));
    let mut element: *mut MemberPtrUnsafe<HasThreeInts, i32> = members.as_mut_ptr();
    assert!(
        ((element.offset((1) as isize))
            == (&mut members[(1) as usize] as *mut MemberPtrUnsafe<HasThreeInts, i32>))
    );
    assert!(((*(&mut obj as *mut HasThreeInts).member((*element.offset((1) as isize)))) == (4)));
    element.prefix_inc();
    element.prefix_inc();
    assert!(((element) == (&mut members[(2) as usize] as *mut MemberPtrUnsafe<HasThreeInts, i32>)));
    assert!(((*(&mut obj as *mut HasThreeInts).member((*element))) == (5)));
    members[(2) as usize] = <MemberPtrUnsafe<HasThreeInts, i32>>::from_offset(
        ::std::mem::offset_of!(HasThreeInts, third),
    );
    assert!(((*(&mut obj as *mut HasThreeInts).member((*element))) == (6)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
