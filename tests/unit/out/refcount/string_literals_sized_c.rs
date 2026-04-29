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
    let  empty_buf : Value<Box<[ u8  ]> > = Rc::new(RefCell::new(Box::<[u8]>::from(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0".as_slice()) )) ;
    assert!((((*empty_buf.borrow())[(0) as usize] as i32) == ('\0' as i32)));
    assert!((((*empty_buf.borrow())[(255) as usize] as i32) == ('\0' as i32)));
    let prefix_buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::<[u8]>::from(
        b"%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0".as_slice(),
    )));
    assert!((((*prefix_buf.borrow())[(0) as usize] as i32) == ('%' as i32)));
    assert!((((*prefix_buf.borrow())[(1) as usize] as i32) == ('\0' as i32)));
    assert!((((*prefix_buf.borrow())[(31) as usize] as i32) == ('\0' as i32)));
    let short_buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::<[u8]>::from(
        b"hi\0\0\0\0\0\0\0\0\0\0\0\0\0\0".as_slice(),
    )));
    assert!((((*short_buf.borrow())[(0) as usize] as i32) == ('h' as i32)));
    assert!((((*short_buf.borrow())[(1) as usize] as i32) == ('i' as i32)));
    assert!((((*short_buf.borrow())[(2) as usize] as i32) == ('\0' as i32)));
    assert!((((*short_buf.borrow())[(15) as usize] as i32) == ('\0' as i32)));
    let exact_buf: Value<Box<[u8]>> =
        Rc::new(RefCell::new(Box::<[u8]>::from(b"hello\0".as_slice())));
    assert!((((*exact_buf.borrow())[(0) as usize] as i32) == ('h' as i32)));
    assert!((((*exact_buf.borrow())[(4) as usize] as i32) == ('o' as i32)));
    assert!((((*exact_buf.borrow())[(5) as usize] as i32) == ('\0' as i32)));
    assert!((::std::mem::size_of::<[u8; 6]>() as u64 == 6_u64));
    assert!(((::std::mem::size_of::<[u8; 6]>() as u64 as u64).wrapping_sub(1_u64) == 5_u64));
    assert!((::std::mem::size_of::<[u8; 1]>() as u64 == 1_u64));
    assert!(((::std::mem::size_of::<[u8; 16]>() as u64 as u64).wrapping_sub(1_u64) == 15_u64));
    return 0;
}
