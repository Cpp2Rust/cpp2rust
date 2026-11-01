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
    let mut vec_: Vec<u8> = Vec::from([195_u8, 167_u8]);
    let mut i: i32 = 27;
    let mut str: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"bar.".as_ptr(),
        (0..).take_while(|&i| *c"bar.".as_ptr().add(i) != 0).count() + 1,
    )
    .to_vec();
    write!((*libcc2rs::cerr_unsafe()), "{:} a", i,);
    (*libcc2rs::cerr_unsafe()).write_all(
        &([
            (&[vec_[(0_usize)] as u8] as &[u8]),
            (&[vec_[(1_usize)] as u8] as &[u8]),
            (&[('o' as libc::c_char) as u8] as &[u8]),
            (&(str)
                .iter()
                .take((str).len() - 1)
                .map(|&c| c as u8)
                .collect::<Vec<u8>>()[..] as &[u8]),
            (&[b'\n'] as &[u8]),
        ]
        .concat()),
    );
    write!((*libcc2rs::cerr_unsafe()), "0x{:x}", 27,);
    (*libcc2rs::cerr_unsafe()).write_all(
        &([
            (b" a\xc3\xa7ordas?" as &[u8]),
            (&[('\n' as libc::c_char) as u8] as &[u8]),
            (b"Sim, 0x" as &[u8]),
        ]
        .concat()),
    );
    write!((*libcc2rs::cerr_unsafe()), "{:x}.\n", i,);
    write!((*libcc2rs::cerr_unsafe()), "Hello, World!\n",);
    (*libcc2rs::cerr_unsafe()).write_all(
        &([
            (&[vec_[(0_usize)] as u8] as &[u8]),
            (&[('\n' as libc::c_char) as u8] as &[u8]),
            (&[vec_[(1_usize)] as u8] as &[u8]),
            (&[('\n' as libc::c_char) as u8] as &[u8]),
        ]
        .concat()),
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
