extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut special: *const ::core::ffi::c_char =
        c"\x07\x08\t\n\x0b\x0c\r !\"#$%&\'()*+,-./:;<=>?@[\\]^_`{|}~\xff".as_ptr();
    static mut expected_0: [::core::ffi::c_char; 41] = unsafe {
        [
            (7 as ::core::ffi::c_char),
            (8 as ::core::ffi::c_char),
            (9 as ::core::ffi::c_char),
            (10 as ::core::ffi::c_char),
            (11 as ::core::ffi::c_char),
            (12 as ::core::ffi::c_char),
            (13 as ::core::ffi::c_char),
            (32 as ::core::ffi::c_char),
            (33 as ::core::ffi::c_char),
            (34 as ::core::ffi::c_char),
            (35 as ::core::ffi::c_char),
            (36 as ::core::ffi::c_char),
            (37 as ::core::ffi::c_char),
            (38 as ::core::ffi::c_char),
            (39 as ::core::ffi::c_char),
            (40 as ::core::ffi::c_char),
            (41 as ::core::ffi::c_char),
            (42 as ::core::ffi::c_char),
            (43 as ::core::ffi::c_char),
            (44 as ::core::ffi::c_char),
            (45 as ::core::ffi::c_char),
            (46 as ::core::ffi::c_char),
            (47 as ::core::ffi::c_char),
            (58 as ::core::ffi::c_char),
            (59 as ::core::ffi::c_char),
            (60 as ::core::ffi::c_char),
            (61 as ::core::ffi::c_char),
            (62 as ::core::ffi::c_char),
            (63 as ::core::ffi::c_char),
            (64 as ::core::ffi::c_char),
            (91 as ::core::ffi::c_char),
            (92 as ::core::ffi::c_char),
            (93 as ::core::ffi::c_char),
            (94 as ::core::ffi::c_char),
            (95 as ::core::ffi::c_char),
            (96 as ::core::ffi::c_char),
            (123 as ::core::ffi::c_char),
            (124 as ::core::ffi::c_char),
            (125 as ::core::ffi::c_char),
            (126 as ::core::ffi::c_char),
            (b'\xff' as ::core::ffi::c_char),
        ]
    };;
    let mut i: i32 = 0;
    'loop_: while ((i)
        < (((::std::mem::size_of::<[::core::ffi::c_char; 41]>() as usize)
            .wrapping_div((::std::mem::size_of::<::core::ffi::c_char>() as usize)))
            as i32))
    {
        assert!((((*special.offset((i) as isize)) as i32) == (expected_0[(i) as usize] as i32)));
        i.postfix_inc();
    }
    return 0;
}
