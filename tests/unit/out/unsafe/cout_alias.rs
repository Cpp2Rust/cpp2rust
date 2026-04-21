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
    let mut cond: bool = true;
    let mut os1: *mut std::fs::File = if cond {
        libcc2rs::cout_unsafe()
    } else {
        libcc2rs::cerr_unsafe()
    };
    write!((*os1), "hello\n",);
    let os2: *mut std::fs::File = &mut if cond {
        std::fs::File::from_raw_fd(
            std::io::stdout()
                .as_fd()
                .try_clone_to_owned()
                .unwrap()
                .into_raw_fd(),
        )
    } else {
        std::fs::File::from_raw_fd(
            std::io::stderr()
                .as_fd()
                .try_clone_to_owned()
                .unwrap()
                .into_raw_fd(),
        )
    } as *mut std::fs::File;
    write!((*os2), "hello\n",);
    return 0;
}
