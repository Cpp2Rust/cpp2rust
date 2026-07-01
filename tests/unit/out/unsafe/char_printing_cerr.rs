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
    let mut str: Vec<libc::c_char> = {
        let s = c"rdas.".as_ptr();
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
    write!(
        std::fs::File::from_raw_fd(
            std::io::stderr()
                .as_fd()
                .try_clone_to_owned()
                .unwrap()
                .into_raw_fd(),
        ),
        "0x{:x}",
        27,
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
            (b" a\xc3\xa7ordas?" as &[u8]),
            (&[('\n' as libc::c_char) as u8] as &[u8]),
            (b"Sim, 0x" as &[u8]),
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
        "{:x}.\n",
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
            (&[vec_[(0_usize)] as u8] as &[u8]),
            (&[('\n' as libc::c_char) as u8] as &[u8]),
            (&[vec_[(1_usize)] as u8] as &[u8]),
            (&[('\n' as libc::c_char) as u8] as &[u8]),
        ]
        .concat()),
    );
    return 0;
}
