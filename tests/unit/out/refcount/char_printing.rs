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
    let str: Value<Vec<u8>> = Rc::new(RefCell::new(
        Ptr::from_string_literal(b"rdas.")
            .to_c_string_iterator()
            .chain(std::iter::once(0))
            .collect::<Vec<u8>>(),
    ));
    write!(libcc2rs::cout(), "{:} a", (*i.borrow()),);
    libcc2rs::cout().write_all(
        &([
            (&[((vec_.as_pointer() as Ptr<u8>).offset(0_usize).read())] as &[u8]),
            (&[((vec_.as_pointer() as Ptr<u8>).offset(1_usize).read())] as &[u8]),
            (&[('o' as u8)] as &[u8]),
            (&(*str.borrow())[..(*str.borrow()).len() - 1] as &[u8]),
            (&[b'\n'] as &[u8]),
        ]
        .concat()),
    );
    write!(libcc2rs::cout(), "0x{:x}", 27,);
    libcc2rs::cout().write_all(
        &([
            (b" a\xc3\xa7ordas?" as &[u8]),
            (&[('\n' as u8)] as &[u8]),
            (b"Sim, 0x" as &[u8]),
        ]
        .concat()),
    );
    write!(libcc2rs::cout(), "{:x}.\n", (*i.borrow()),);
    write!(libcc2rs::cout(), "Hello, World!\n",);
    libcc2rs::cout().write_all(
        &([
            (&[((vec_.as_pointer() as Ptr<u8>).offset(0_usize).read())] as &[u8]),
            (&[('\n' as u8)] as &[u8]),
            (&[((vec_.as_pointer() as Ptr<u8>).offset(1_usize).read())] as &[u8]),
            (&[('\n' as u8)] as &[u8]),
        ]
        .concat()),
    );
    return 0;
}
