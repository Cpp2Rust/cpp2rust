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
    let mut cond: bool = true;
    let mut os1: *mut std::fs::File = if cond {
        (&raw mut (*libcc2rs::cout_unsafe()) as *mut std::fs::File)
    } else {
        (&raw mut (*libcc2rs::cerr_unsafe()) as *mut std::fs::File)
    };
    write!((*os1), "hello\n",);
    let os2: *mut std::fs::File = if cond {
        (&raw mut (*libcc2rs::cout_unsafe()) as *mut std::fs::File)
    } else {
        (&raw mut (*libcc2rs::cerr_unsafe()) as *mut std::fs::File)
    };
    write!((*os2), "hello\n",);
    let is: *mut std::fs::File = &mut (*libcc2rs::cin_unsafe());
    assert!(((libcc2rs::istream_get(&mut *(is as *mut ::std::fs::File))) == (-1_i32)));
    let mut is_ptr: *mut std::fs::File = (&raw mut (*libcc2rs::cin_unsafe()) as *mut std::fs::File);
    assert!(((libcc2rs::istream_get(&mut *(is_ptr as *mut ::std::fs::File))) == (-1_i32)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
