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
    let mut vec_: Vec<u8> = vec![195_u8, 167_u8];
    let mut i: i32 = 27;
    let mut str: Vec<u8> = {
        let s = b"rdas.\0".as_ptr();
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    };
    write!(
        std::fs::File::from_raw_fd(
            std::io::stderr()
                .as_fd()
                .try_clone_to_owned()
                .unwrap()
                .into_raw_fd(),
        ),
        "{:} a",
        i,
    );
    std::fs::File::from_raw_fd(
        std::io::stderr()
            .as_fd()
            .try_clone_to_owned()
            .unwrap()
            .into_raw_fd(),
    )
    .write_all(
        &([
            (&[vec_[(0_u64) as usize]] as &[u8]),
            (&[vec_[(1_u64) as usize]] as &[u8]),
            (&[('o' as u8)] as &[u8]),
            (&(str)[..(str).len() - 1] as &[u8]),
            (&[b'\n'] as &[u8]),
        ]
        .concat()),
    );
    write!(
        std::fs::File::from_raw_fd(
            std::io::stderr()
                .as_fd()
                .try_clone_to_owned()
                .unwrap()
                .into_raw_fd(),
        ),
        "0x{:x} açordas?\nSim, 0x{:x}.\n",
        27,
        i,
    );
    write!(
        std::fs::File::from_raw_fd(
            std::io::stderr()
                .as_fd()
                .try_clone_to_owned()
                .unwrap()
                .into_raw_fd(),
        ),
        "Hello, World!\n",
    );
    std::fs::File::from_raw_fd(
        std::io::stderr()
            .as_fd()
            .try_clone_to_owned()
            .unwrap()
            .into_raw_fd(),
    )
    .write_all(
        &([
            (&[vec_[(0_u64) as usize]] as &[u8]),
            (&[('\n' as u8)] as &[u8]),
            (&[vec_[(1_u64) as usize]] as &[u8]),
            (&[('\n' as u8)] as &[u8]),
        ]
        .concat()),
    );
    return 0;
}
