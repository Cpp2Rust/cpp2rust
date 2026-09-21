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
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut buf: [libc::c_char; 6] = [
        ('a' as libc::c_char),
        ('b' as libc::c_char),
        ('\0' as libc::c_char),
        ('c' as libc::c_char),
        ('d' as libc::c_char),
        ('\0' as libc::c_char),
    ];
    libc::fputs((buf.as_mut_ptr()).cast_const(), libcc2rs::stdout_unsafe());
    libc::fputc((('|' as libc::c_char) as i32), libcc2rs::stdout_unsafe());
    libc::puts((buf.as_mut_ptr()).cast_const());
    let mut s: Vec<libc::c_char> = {
        let s = (buf.as_mut_ptr()).cast_const();
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    };
    printf(c"%zu\n".as_ptr() as *const i8, (s.len() - 1));
    printf(
        c"%zu %zu\n".as_ptr() as *const i8,
        libc::strlen((buf.as_mut_ptr()).cast_const()),
        match s.iter().rposition(|&c| {
            ::std::ffi::CStr::from_ptr((buf.as_mut_ptr()).cast_const())
                .to_str()
                .unwrap()
                .contains(c as u8 as char)
        }) {
            Some(idx) => idx,
            None => usize::MAX,
        },
    );
    let mut lit: *const libc::c_char = (&[
        (120 as libc::c_char),
        (121 as libc::c_char),
        (0 as libc::c_char),
        (122 as libc::c_char),
        (119 as libc::c_char),
        (0 as libc::c_char),
    ])
        .as_ptr();
    let mut t: Vec<libc::c_char> = {
        let s = lit;
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    };
    printf(c"%zu\n".as_ptr() as *const i8, (t.len() - 1));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
