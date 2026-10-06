extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut vec_: Vec<u8> = vec![195_u8, 167_u8];
    let mut i: i32 = 27;
    let str: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"foo.").to_c_bytes();
        __bytes.push(0);
        __bytes
    }));
    write!(libcc2rs::cout(), "{:} a", i,);
    libcc2rs::cout().write_all(
        &([
            (&[vec_[0_usize] as u8] as &[u8]),
            (&[vec_[1_usize] as u8] as &[u8]),
            (&[('o' as i8) as u8] as &[u8]),
            (&(*str.borrow())
                .iter()
                .take((*str.borrow()).len() - 1)
                .map(|&c| c as u8)
                .collect::<Vec<u8>>()[..] as &[u8]),
            (&[b'\n'] as &[u8]),
        ]
        .concat()),
    );
    write!(libcc2rs::cout(), "0x{:x}", 27,);
    libcc2rs::cout().write_all(
        &([
            (b" a\xc3\xa7ordas?" as &[u8]),
            (&[('\n' as i8) as u8] as &[u8]),
            (b"Sim, 0x" as &[u8]),
        ]
        .concat()),
    );
    write!(libcc2rs::cout(), "{:x}.\n", i,);
    write!(libcc2rs::cout(), "Hello, World!\n",);
    libcc2rs::cout().write_all(
        &([
            (&[vec_[0_usize] as u8] as &[u8]),
            (&[('\n' as i8) as u8] as &[u8]),
            (&[vec_[1_usize] as u8] as &[u8]),
            (&[('\n' as i8) as u8] as &[u8]),
        ]
        .concat()),
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
