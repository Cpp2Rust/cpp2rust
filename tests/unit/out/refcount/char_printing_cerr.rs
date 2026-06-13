extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let vec_: Value<Vec<u8>> = Rc::new(RefCell::new(vec![195_u8, 167_u8]));
    let i: Value<i32> = Rc::new(RefCell::new(27));
    let str: Value<Vec<::core::ffi::c_char>> = Rc::new(RefCell::new(
        Ptr::from_string_literal(b"rdas.")
            .to_c_string_iterator()
            .chain(std::iter::once(0))
            .collect::<Vec<::core::ffi::c_char>>(),
    ));
    write!(libcc2rs::cerr(), "{:} a", (*i.borrow()),);
    libcc2rs::cerr().write_all(
        &([
            (&[((vec_.as_pointer() as Ptr<u8>)
                .offset(0_usize as isize)
                .read()) as u8] as &[u8]),
            (&[((vec_.as_pointer() as Ptr<u8>)
                .offset(1_usize as isize)
                .read()) as u8] as &[u8]),
            (&[('o' as ::core::ffi::c_char) as u8] as &[u8]),
            (&(*str.borrow())
                .iter()
                .take((*str.borrow()).len() - 1)
                .map(|&c| c as u8)
                .collect::<Vec<u8>>()[..] as &[u8]),
            (&[b'\n'] as &[u8]),
        ]
        .concat()),
    );
    write!(libcc2rs::cerr(), "0x{:x}", 27,);
    libcc2rs::cerr().write_all(
        &([
            (b" a\xc3\xa7ordas?" as &[u8]),
            (&[('\n' as ::core::ffi::c_char) as u8] as &[u8]),
            (b"Sim, 0x" as &[u8]),
        ]
        .concat()),
    );
    write!(libcc2rs::cerr(), "{:x}.\n", (*i.borrow()),);
    write!(libcc2rs::cerr(), "Hello, World!\n",);
    libcc2rs::cerr().write_all(
        &([
            (&[((vec_.as_pointer() as Ptr<u8>)
                .offset(0_usize as isize)
                .read()) as u8] as &[u8]),
            (&[('\n' as ::core::ffi::c_char) as u8] as &[u8]),
            (&[((vec_.as_pointer() as Ptr<u8>)
                .offset(1_usize as isize)
                .read()) as u8] as &[u8]),
            (&[('\n' as ::core::ffi::c_char) as u8] as &[u8]),
        ]
        .concat()),
    );
    return 0;
}
