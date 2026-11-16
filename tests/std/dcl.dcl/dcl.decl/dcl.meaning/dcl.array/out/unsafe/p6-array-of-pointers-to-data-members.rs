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
    let mut members: [isize; 3] = [
        (::std::mem::offset_of!(HasThreeInts, third) as isize),
        (::std::mem::offset_of!(HasThreeInts, first) as isize),
        (::std::mem::offset_of!(HasThreeInts, second) as isize),
    ];
    let mut obj: HasThreeInts = HasThreeInts {
        first: 4,
        second: 5,
        third: 6,
    };
    assert!(
        ((*(&mut obj as *mut HasThreeInts)
            .wrapping_byte_offset(members[(0) as usize])
            .cast::<i32>())
            == (6))
    );
    assert!(
        ((*(&mut obj as *mut HasThreeInts)
            .wrapping_byte_offset(members[(1) as usize])
            .cast::<i32>())
            == (4))
    );
    assert!(
        ((*(&mut obj as *mut HasThreeInts)
            .wrapping_byte_offset(members[(2) as usize])
            .cast::<i32>())
            == (5))
    );
    let mut element: *mut isize = members.as_mut_ptr();
    assert!(((element.offset((1) as isize)) == (&mut members[(1) as usize] as *mut isize)));
    assert!(
        ((*(&mut obj as *mut HasThreeInts)
            .wrapping_byte_offset((*element.offset((1) as isize)))
            .cast::<i32>())
            == (4))
    );
    element.prefix_inc();
    element.prefix_inc();
    assert!(((element) == (&mut members[(2) as usize] as *mut isize)));
    assert!(
        ((*(&mut obj as *mut HasThreeInts)
            .wrapping_byte_offset((*element))
            .cast::<i32>())
            == (5))
    );
    members[(2) as usize] = (::std::mem::offset_of!(HasThreeInts, third) as isize);
    assert!(
        ((*(&mut obj as *mut HasThreeInts)
            .wrapping_byte_offset((*element))
            .cast::<i32>())
            == (6))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
