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
    let empty_buf: Value<Box<[core::ffi::c_char]>> = Rc::new(RefCell::new(
        vec![0 as core::ffi::c_char; 256].into_boxed_slice(),
    ));
    assert!((((((*empty_buf.borrow())[(0) as usize] as i32) == ('\0' as i32)) as i32) != 0));
    assert!((((((*empty_buf.borrow())[(255) as usize] as i32) == ('\0' as i32)) as i32) != 0));
    let prefix_buf: Value<Box<[core::ffi::c_char]>> = Rc::new(RefCell::new(Box::from(
        libcc2rs::char_array(b"%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
    )));
    assert!((((((*prefix_buf.borrow())[(0) as usize] as i32) == ('%' as i32)) as i32) != 0));
    assert!((((((*prefix_buf.borrow())[(1) as usize] as i32) == ('\0' as i32)) as i32) != 0));
    assert!((((((*prefix_buf.borrow())[(31) as usize] as i32) == ('\0' as i32)) as i32) != 0));
    let short_buf: Value<Box<[core::ffi::c_char]>> = Rc::new(RefCell::new(Box::from(
        libcc2rs::char_array(b"hi\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
    )));
    assert!((((((*short_buf.borrow())[(0) as usize] as i32) == ('h' as i32)) as i32) != 0));
    assert!((((((*short_buf.borrow())[(1) as usize] as i32) == ('i' as i32)) as i32) != 0));
    assert!((((((*short_buf.borrow())[(2) as usize] as i32) == ('\0' as i32)) as i32) != 0));
    assert!((((((*short_buf.borrow())[(15) as usize] as i32) == ('\0' as i32)) as i32) != 0));
    let exact_buf: Value<Box<[core::ffi::c_char]>> =
        Rc::new(RefCell::new(Box::from(libcc2rs::char_array(b"hello\0"))));
    assert!((((((*exact_buf.borrow())[(0) as usize] as i32) == ('h' as i32)) as i32) != 0));
    assert!((((((*exact_buf.borrow())[(4) as usize] as i32) == ('o' as i32)) as i32) != 0));
    assert!((((((*exact_buf.borrow())[(5) as usize] as i32) == ('\0' as i32)) as i32) != 0));
    assert!((((::std::mem::size_of::<[core::ffi::c_char; 6]>() == 6_usize) as i32) != 0));
    assert!(
        ((((::std::mem::size_of::<[core::ffi::c_char; 6]>() as usize).wrapping_sub(1_usize)
            == 5_usize) as i32)
            != 0)
    );
    assert!((((::std::mem::size_of::<[core::ffi::c_char; 1]>() == 1_usize) as i32) != 0));
    assert!(
        ((((::std::mem::size_of::<[core::ffi::c_char; 16]>() as usize).wrapping_sub(1_usize)
            == 15_usize) as i32)
            != 0)
    );
    return 0;
}
