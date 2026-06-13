extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn strlen_0(s: Ptr<::core::ffi::c_char>) -> usize {
    let s: Value<Ptr<::core::ffi::c_char>> = Rc::new(RefCell::new(s));
    let begin: Value<Ptr<::core::ffi::c_char>> = Rc::new(RefCell::new((*s.borrow()).clone()));
    'loop_: while (((*s.borrow()).read()) != 0) {
        (*s.borrow_mut()).prefix_inc();
    }
    return ((((*s.borrow()).clone() - (*begin.borrow()).clone()) as i64) as usize);
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<Box<[::core::ffi::c_char]>> = Rc::new(RefCell::new(Box::new([
        ('s' as ::core::ffi::c_char),
        ('t' as ::core::ffi::c_char),
        ('r' as ::core::ffi::c_char),
        ('i' as ::core::ffi::c_char),
        ('n' as ::core::ffi::c_char),
        ('g' as ::core::ffi::c_char),
        ('\0' as ::core::ffi::c_char),
    ])));
    return (({
        let _s: Ptr<::core::ffi::c_char> =
            ((s.as_pointer() as Ptr<::core::ffi::c_char>).offset(0 as isize));
        strlen_0(_s)
    }) as i32);
}
