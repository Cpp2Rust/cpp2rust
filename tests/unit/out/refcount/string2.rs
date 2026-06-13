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
    let arr: Value<Vec<::core::ffi::c_char>> = Rc::new(RefCell::new(
        Ptr::from_string_literal(b"foo")
            .to_c_string_iterator()
            .chain(std::iter::once(0))
            .collect::<Vec<::core::ffi::c_char>>(),
    ));
    (arr.as_pointer() as Ptr<::core::ffi::c_char>)
        .offset(1_usize as isize)
        .write(('b' as ::core::ffi::c_char));
    let p: Value<Ptr<::core::ffi::c_char>> = Rc::new(RefCell::new(
        (arr.as_pointer() as Ptr<::core::ffi::c_char>).offset((1) as isize),
    ));
    assert!(((((*p.borrow()).read()) as i32) == (('b' as ::core::ffi::c_char) as i32)));
    assert!(
        (*arr.borrow())
            .iter()
            .copied()
            .take((*arr.borrow()).len().saturating_sub(1))
            .eq(Ptr::from_string_literal(b"fbo").to_c_string_iterator())
    );
    return 0;
}
